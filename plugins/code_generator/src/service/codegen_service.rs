//! 代码生成服务层

use sea_orm::*;
use std::collections::HashMap;
use std::io::{Write, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;

use crate::dto::*;
use crate::error::CodeGenError;

/// 代码生成服务
pub struct CodeGenService;

impl CodeGenService {
    /// 获取所有表
    pub async fn get_tables(
        db: &DatabaseConnection,
        schema: &str,
    ) -> Result<Vec<TableInfo>, CodeGenError> {
        let query = format!(
            r#"
            SELECT 
                table_name,
                table_schema,
                table_comment
            FROM information_schema.tables
            WHERE table_schema = '{}'
            AND table_type = 'BASE TABLE'
            ORDER BY table_name
            "#,
            schema
        );

        let tables = db
            .query_all(Statement::from_string(
                DatabaseBackend::MySql,
                query,
            ))
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;

        let result = tables
            .iter()
            .map(|row| TableInfo {
                table_name: row.try_get("", "table_name").unwrap_or_default(),
                table_schema: row.try_get("", "table_schema").unwrap_or_default(),
                table_comment: row.try_get("", "table_comment").ok(),
            })
            .collect();

        Ok(result)
    }

    /// 获取表字段信息
    pub async fn get_columns(
        db: &DatabaseConnection,
        schema: &str,
        table_name: &str,
    ) -> Result<Vec<ColumnInfo>, CodeGenError> {
        let query = format!(
            r#"
            SELECT 
                column_name,
                data_type,
                column_type,
                is_nullable,
                column_key,
                column_comment
            FROM information_schema.columns
            WHERE table_schema = '{}'
            AND table_name = '{}'
            ORDER BY ordinal_position
            "#,
            schema, table_name
        );

        let columns = db
            .query_all(Statement::from_string(
                DatabaseBackend::MySql,
                query,
            ))
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;

        let result = columns
            .iter()
            .map(|row| ColumnInfo {
                column_name: row.try_get("", "column_name").unwrap_or_default(),
                data_type: row.try_get("", "data_type").unwrap_or_default(),
                column_type: row.try_get("", "column_type").unwrap_or_default(),
                is_nullable: row.try_get("", "is_nullable").unwrap_or_default(),
                column_key: row.try_get("", "column_key").unwrap_or_default(),
                column_comment: row.try_get("", "column_comment").ok(),
            })
            .collect();

        Ok(result)
    }

    /// 获取可用模板列表
    pub fn get_templates() -> Vec<String> {
        vec![
            "entity".to_string(),
            "dto".to_string(),
            "service".to_string(),
            "api".to_string(),
            "crud_full".to_string(),
        ]
    }

    /// 生成代码预览
    pub async fn preview_code(
        db: &DatabaseConnection,
        schema: &str,
        table_name: &str,
        module_name: &str,
    ) -> Result<CodePreview, CodeGenError> {
        let columns = Self::get_columns(db, schema, table_name).await?;
        
        let mut files = HashMap::new();
        
        // 生成Entity代码
        let entity_code = Self::generate_entity(table_name, module_name, &columns)?;
        files.insert(format!("entity/{}.rs", module_name), entity_code);
        
        // 生成DTO代码
        let dto_code = Self::generate_dto(table_name, module_name, &columns)?;
        files.insert(format!("dto/{}_dto.rs", module_name), dto_code);
        
        // 生成Service代码
        let service_code = Self::generate_service(table_name, module_name)?;
        files.insert(format!("service/{}_service.rs", module_name), service_code);
        
        // 生成API代码
        let api_code = Self::generate_api(table_name, module_name)?;
        files.insert(format!("api/{}_api.rs", module_name), api_code);
        
        Ok(CodePreview { files })
    }

    /// 生成Entity代码
    fn generate_entity(
        table_name: &str,
        module_name: &str,
        columns: &[ColumnInfo],
    ) -> Result<String, CodeGenError> {
        let mut code = String::new();
        code.push_str(&format!("//! {} Entity\n\n", module_name));
        code.push_str("use sea_orm::entity::prelude::*;\n");
        code.push_str("use serde::{Deserialize, Serialize};\n\n");
        code.push_str("#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]\n");
        code.push_str(&format!("#[sea_orm(table_name = \"{}\")]\n", table_name));
        code.push_str("pub struct Model {\n");
        
        for col in columns {
            if let Some(comment) = &col.column_comment {
                code.push_str(&format!("    /// {}\n", comment));
            }
            if col.column_key == "PRI" {
                code.push_str("    #[sea_orm(primary_key)]\n");
            }
            let rust_type = Self::map_db_type_to_rust(&col.data_type, &col.is_nullable);
            code.push_str(&format!("    pub {}: {},\n", col.column_name, rust_type));
        }
        
        code.push_str("}\n\n");
        code.push_str("#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]\n");
        code.push_str("pub enum Relation {}\n\n");
        code.push_str("impl ActiveModelBehavior for ActiveModel {}\n");
        
        Ok(code)
    }

    /// 生成DTO代码
    fn generate_dto(
        _table_name: &str,
        module_name: &str,
        columns: &[ColumnInfo],
    ) -> Result<String, CodeGenError> {
        let mut code = String::new();
        code.push_str(&format!("//! {} DTO\n\n", module_name));
        code.push_str("use serde::{Deserialize, Serialize};\n");
        code.push_str("use validator::Validate;\n\n");
        
        // 创建请求DTO
        code.push_str(&format!("#[derive(Debug, Deserialize, Validate)]\n"));
        code.push_str(&format!("pub struct Create{}Param {{\n", Self::to_pascal_case(module_name)));
        
        for col in columns {
            if col.column_key != "PRI" {
                let rust_type = Self::map_db_type_to_rust(&col.data_type, &col.is_nullable);
                code.push_str(&format!("    pub {}: {},\n", col.column_name, rust_type));
            }
        }
        code.push_str("}\n\n");
        
        // 更新请求DTO
        code.push_str(&format!("#[derive(Debug, Deserialize, Validate)]\n"));
        code.push_str(&format!("pub struct Update{}Param {{\n", Self::to_pascal_case(module_name)));
        
        for col in columns {
            if col.column_key != "PRI" {
                let rust_type = Self::map_db_type_to_rust(&col.data_type, &col.is_nullable);
                code.push_str(&format!("    pub {}: {},\n", col.column_name, rust_type));
            }
        }
        code.push_str("}\n");
        
        Ok(code)
    }

    /// 生成Service代码
    fn generate_service(
        _table_name: &str,
        module_name: &str,
    ) -> Result<String, CodeGenError> {
        let pascal_name = Self::to_pascal_case(module_name);
        let code = format!(
            r#"//! {} Service

use sea_orm::*;

pub struct {}Service;

impl {}Service {{
    /// 创建
    pub async fn create(db: &DatabaseConnection, param: Create{}Param) -> Result<Model, DbErr> {{
        // TODO: 实现创建逻辑
        unimplemented!()
    }}
    
    /// 更新
    pub async fn update(db: &DatabaseConnection, id: i64, param: Update{}Param) -> Result<Model, DbErr> {{
        // TODO: 实现更新逻辑
        unimplemented!()
    }}
    
    /// 删除
    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), DbErr> {{
        // TODO: 实现删除逻辑
        unimplemented!()
    }}
    
    /// 根据ID获取
    pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<Model>, DbErr> {{
        // TODO: 实现查询逻辑
        unimplemented!()
    }}
}}
"#,
            module_name, pascal_name, pascal_name, pascal_name, pascal_name
        );
        
        Ok(code)
    }

    /// 生成API代码
    fn generate_api(
        _table_name: &str,
        module_name: &str,
    ) -> Result<String, CodeGenError> {
        let pascal_name = Self::to_pascal_case(module_name);
        let code = format!(
            r#"//! {} API

use axum::{{
    extract::{{Path, State}},
    response::Json,
    routing::{{get, post, put, delete}},
    Router,
}};
use sea_orm::DatabaseConnection;

/// 创建{}
pub async fn create_{}(
    State(db): State<DatabaseConnection>,
    Json(param): Json<Create{}Param>,
) -> Result<Json<ApiResponse<Model>>, AppError> {{
    let data = {}Service::create(&db, param).await?;
    Ok(Json(ApiResponse::success(data)))
}}

/// 更新{}
pub async fn update_{}(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(param): Json<Update{}Param>,
) -> Result<Json<ApiResponse<Model>>, AppError> {{
    let data = {}Service::update(&db, id, param).await?;
    Ok(Json(ApiResponse::success(data)))
}}

/// 删除{}
pub async fn delete_{}(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, AppError> {{
    {}Service::delete(&db, id).await?;
    Ok(Json(ApiResponse::success_msg("删除成功")))
}}

/// 获取{}
pub async fn get_{}(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Model>>, AppError> {{
    let data = {}Service::get_by_id(&db, id).await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(ApiResponse::success(data)))
}}

/// 创建路由
pub fn {}_routes() -> Router<DatabaseConnection> {{
    Router::new()
        .route("/", post(create_{}))
        .route("/{{id}}", get(get_{}))
        .route("/{{id}}", put(update_{}))
        .route("/{{id}}", delete(delete_{}))
}}
"#,
            module_name,
            module_name, module_name, pascal_name, pascal_name,
            module_name, module_name, pascal_name, pascal_name,
            module_name, module_name, pascal_name,
            module_name, module_name, pascal_name,
            module_name, module_name, module_name, module_name, module_name
        );
        
        Ok(code)
    }

    /// 数据库类型映射到Rust类型
    fn map_db_type_to_rust(db_type: &str, is_nullable: &str) -> String {
        let base_type = match db_type {
            "int" | "integer" | "tinyint" | "smallint" => "i32",
            "bigint" => "i64",
            "varchar" | "char" | "text" | "longtext" => "String",
            "datetime" | "timestamp" => "DateTime",
            "decimal" | "float" | "double" => "f64",
            "bool" | "boolean" => "bool",
            _ => "String",
        };
        
        if is_nullable == "YES" {
            format!("Option<{}>", base_type)
        } else {
            base_type.to_string()
        }
    }

    /// 转换为PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(c).collect(),
                }
            })
            .collect()
    }

    /// 下载代码为ZIP
    pub async fn download_code(
        db: &DatabaseConnection,
        schema: &str,
        table_name: &str,
        module_name: &str,
    ) -> Result<Vec<u8>, CodeGenError> {
        let preview = Self::preview_code(db, schema, table_name, module_name).await?;
        
        let mut cursor = Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut cursor);
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            
            for (file_path, content) in preview.files {
                zip.start_file(file_path, options)
                    .map_err(|e| CodeGenError::GenerateError(e.to_string()))?;
                zip.write_all(content.as_bytes())
                    .map_err(|e| CodeGenError::GenerateError(e.to_string()))?;
            }
            
            zip.finish()
                .map_err(|e| CodeGenError::GenerateError(e.to_string()))?;
        }
        
        Ok(cursor.into_inner())
    }

    /// 生成代码到本地文件系统
    pub async fn generate_to_filesystem(
        db: &DatabaseConnection,
        schema: &str,
        table_name: &str,
        module_name: &str,
        base_path: Option<&str>,
    ) -> Result<Vec<String>, CodeGenError> {
        use tokio::fs;
        use std::path::PathBuf;
        
        let preview = Self::preview_code(db, schema, table_name, module_name).await?;
        let base = base_path.unwrap_or("./generated");
        let base_path = PathBuf::from(base);
        
        let mut created_files = Vec::new();
        
        for (file_path, content) in preview.files {
            let full_path = base_path.join(&file_path);
            
            // 创建父目录
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| CodeGenError::GenerateError(format!("创建目录失败: {}", e)))?;
                
                // 为Rust项目创建mod.rs（相当于Python的__init__.py）
                let mod_file = parent.join("mod.rs");
                if !mod_file.exists() {
                    fs::write(&mod_file, "// Auto-generated module\n")
                        .await
                        .map_err(|e| CodeGenError::GenerateError(format!("创建mod.rs失败: {}", e)))?;
                }
            }
            
            // 写入代码文件
            fs::write(&full_path, content.as_bytes())
                .await
                .map_err(|e| CodeGenError::GenerateError(format!("写入文件失败: {}", e)))?;
            
            created_files.push(full_path.display().to_string());
        }
        
        Ok(created_files)
    }

    /// 基于业务ID生成代码到文件系统
    pub async fn generate_by_business_id(
        db: &DatabaseConnection,
        business_id: i64,
    ) -> Result<Vec<String>, CodeGenError> {
        use crate::service::BusinessService;
        
        // 获取业务信息
        let business = BusinessService::get_by_id(db, business_id).await?;
        
        // 生成代码
        let base_path = business.gen_path.as_deref().unwrap_or("./generated");
        let schema = "fba"; // 可以从配置读取
        
        Self::generate_to_filesystem(
            db,
            schema,
            &business.table_name,
            business.filename.as_deref().unwrap_or(&business.table_name),
            Some(base_path),
        ).await
    }
}
