//! 业务模型服务层

use sea_orm::*;

use crate::entity::{gen_business, gen_column};
use crate::dto::*;
use crate::error::CodeGenError;
use crate::service::CodeGenService;

/// 业务模型服务
pub struct BusinessService;

impl BusinessService {
    /// 获取所有业务
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<GenBusinessDetail>, CodeGenError> {
        let businesses = gen_business::Entity::find()
            .order_by_desc(gen_business::Column::CreatedTime)
            .all(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        Ok(businesses.into_iter().map(Self::to_detail).collect())
    }
    
    /// 根据ID获取业务
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<GenBusinessDetail, CodeGenError> {
        let business = gen_business::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?
            .ok_or(CodeGenError::NotFound("业务不存在".to_string()))?;
        
        Ok(Self::to_detail(business))
    }
    
    /// 分页获取业务
    pub async fn get_list(
        db: &DatabaseConnection,
        table_name: Option<String>,
        pagination: PaginationQuery,
    ) -> Result<PageData<GenBusinessDetail>, CodeGenError> {
        let mut select = gen_business::Entity::find();
        
        if let Some(name) = table_name {
            select = select.filter(gen_business::Column::TableName.contains(&name));
        }
        
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        let businesses = select
            .order_by_desc(gen_business::Column::CreatedTime)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        let items = businesses.into_iter().map(Self::to_detail).collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 创建业务
    pub async fn create(
        db: &DatabaseConnection,
        param: CreateGenBusinessParam,
    ) -> Result<GenBusinessDetail, CodeGenError> {
        let now = chrono::Utc::now().naive_utc();
        
        let business = gen_business::ActiveModel {
            app_name: Set(param.app_name),
            table_name: Set(param.table_name),
            doc_comment: Set(param.doc_comment),
            table_comment: Set(param.table_comment),
            class_name: Set(param.class_name),
            schema_name: Set(param.schema_name),
            filename: Set(param.filename),
            default_datetime_column: Set(param.default_datetime_column.unwrap_or(true)),
            api_version: Set(param.api_version.unwrap_or_else(|| "v1".to_string())),
            gen_path: Set(param.gen_path),
            remark: Set(param.remark),
            created_time: Set(now),
            ..Default::default()
        };
        
        let result = business
            .insert(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        Ok(Self::to_detail(result))
    }
    
    /// 更新业务
    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        param: UpdateGenBusinessParam,
    ) -> Result<u64, CodeGenError> {
        let business = gen_business::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?
            .ok_or(CodeGenError::NotFound("业务不存在".to_string()))?;
        
        let now = chrono::Utc::now().naive_utc();
        let mut business: gen_business::ActiveModel = business.into();
        
        if let Some(app_name) = param.app_name {
            business.app_name = Set(app_name);
        }
        if let Some(doc_comment) = param.doc_comment {
            business.doc_comment = Set(doc_comment);
        }
        if let Some(table_comment) = param.table_comment {
            business.table_comment = Set(Some(table_comment));
        }
        if let Some(class_name) = param.class_name {
            business.class_name = Set(Some(class_name));
        }
        if let Some(schema_name) = param.schema_name {
            business.schema_name = Set(Some(schema_name));
        }
        if let Some(filename) = param.filename {
            business.filename = Set(Some(filename));
        }
        if let Some(default_datetime_column) = param.default_datetime_column {
            business.default_datetime_column = Set(default_datetime_column);
        }
        if let Some(api_version) = param.api_version {
            business.api_version = Set(api_version);
        }
        if let Some(gen_path) = param.gen_path {
            business.gen_path = Set(Some(gen_path));
        }
        if let Some(remark) = param.remark {
            business.remark = Set(Some(remark));
        }
        
        business.updated_time = Set(Some(now));
        
        business
            .update(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        Ok(1)
    }
    
    /// 删除业务
    pub async fn delete(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<u64, CodeGenError> {
        // 先删除关联的列
        gen_column::Entity::delete_many()
            .filter(gen_column::Column::BusinessId.eq(id))
            .exec(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        // 再删除业务
        let result = gen_business::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected)
    }
    
    /// 获取业务的所有列
    pub async fn get_columns(
        db: &DatabaseConnection,
        business_id: i64,
    ) -> Result<Vec<GenColumnDetail>, CodeGenError> {
        let columns = gen_column::Entity::find_by_business_id(business_id)
            .all(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        Ok(columns.into_iter().map(Self::column_to_detail).collect())
    }
    
    /// 转换为详情DTO
    fn to_detail(model: gen_business::Model) -> GenBusinessDetail {
        GenBusinessDetail {
            id: model.id,
            app_name: model.app_name,
            table_name: model.table_name,
            doc_comment: model.doc_comment,
            table_comment: model.table_comment,
            class_name: model.class_name,
            schema_name: model.schema_name,
            filename: model.filename,
            default_datetime_column: model.default_datetime_column,
            api_version: model.api_version,
            gen_path: model.gen_path,
            remark: model.remark,
            created_time: model.created_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
    
    /// 列转换为详情DTO
    fn column_to_detail(model: gen_column::Model) -> GenColumnDetail {
        GenColumnDetail {
            id: model.id,
            business_id: model.business_id,
            column_name: model.column_name,
            column_comment: model.column_comment,
            column_type: model.column_type,
            python_type: model.python_type,
            ts_type: model.ts_type,
            required: model.required,
            is_pk: model.is_pk,
            is_fk: model.is_fk,
            is_query: model.is_query,
            is_list: model.is_list,
            is_form: model.is_form,
            query_type: model.query_type,
            form_type: model.form_type,
            sort: model.sort,
        }
    }
    
    /// 导入表结构到业务模型
    pub async fn import_table(
        db: &DatabaseConnection,
        param: ImportTableParam,
    ) -> Result<GenBusinessDetail, CodeGenError> {
        let schema = param.table_schema.as_deref().unwrap_or("fba");
        
        // 检查表是否存在
        let table = CodeGenService::get_tables(db, schema)
            .await?
            .into_iter()
            .find(|t| t.table_name == param.table_name)
            .ok_or(CodeGenError::NotFound("数据库表不存在".to_string()))?;
        
        // 检查是否已导入
        let existing = gen_business::Entity::find_by_table_name(&param.table_name)
            .one(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        if existing.is_some() {
            return Err(CodeGenError::ValidationError("已存在相同数据库表业务".to_string()));
        }
        
        let now = chrono::Utc::now().naive_utc();
        let class_name = CodeGenService::to_pascal_case(&param.table_name);
        let doc_comment = table.table_comment.clone()
            .unwrap_or_else(|| param.table_name.split('_').last().unwrap_or("").to_string());
        
        // 创建业务记录
        let business = gen_business::ActiveModel {
            app_name: Set(param.app.clone()),
            table_name: Set(param.table_name.clone()),
            doc_comment: Set(doc_comment),
            table_comment: Set(table.table_comment),
            class_name: Set(Some(class_name.clone())),
            schema_name: Set(Some(class_name)),
            filename: Set(Some(param.table_name.clone())),
            default_datetime_column: Set(true),
            api_version: Set("v1".to_string()),
            gen_path: Set(None),
            remark: Set(None),
            created_time: Set(now),
            ..Default::default()
        };
        
        let business_result = business
            .insert(db)
            .await
            .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        
        // 获取列信息并创建列记录
        let columns = CodeGenService::get_columns(db, schema, &param.table_name).await?;
        
        for (idx, col) in columns.iter().enumerate() {
            let column = gen_column::ActiveModel {
                business_id: Set(business_result.id),
                column_name: Set(col.column_name.clone()),
                column_comment: Set(col.column_comment.clone()),
                column_type: Set(col.data_type.clone()),
                python_type: Set(Some(Self::map_db_type_to_python(&col.data_type))),
                ts_type: Set(Some(Self::map_db_type_to_ts(&col.data_type))),
                required: Set(col.is_nullable == "NO"),
                is_pk: Set(col.column_key == "PRI"),
                is_fk: Set(col.column_key == "MUL"),
                is_query: Set(true),
                is_list: Set(true),
                is_form: Set(col.column_key != "PRI"),
                query_type: Set(Some("eq".to_string())),
                form_type: Set(Some("input".to_string())),
                sort: Set(idx as i32),
                created_time: Set(now),
                ..Default::default()
            };
            
            column
                .insert(db)
                .await
                .map_err(|e| CodeGenError::DatabaseError(e.to_string()))?;
        }
        
        Ok(Self::to_detail(business_result))
    }
    
    /// 获取代码生成路径列表
    pub async fn get_generate_paths(
        db: &DatabaseConnection,
        business_id: i64,
    ) -> Result<Vec<String>, CodeGenError> {
        let business = Self::get_by_id(db, business_id).await?;
        
        let base_path = business.gen_path.unwrap_or_else(|| 
            format!("backend/app/{}", business.app_name)
        );
        
        let mut paths = Vec::new();
        paths.push(format!("{}/api/{}/{}.rs", base_path, business.api_version, business.filename.clone().unwrap_or(business.table_name.clone())));
        paths.push(format!("{}/model/{}.rs", base_path, business.filename.clone().unwrap_or(business.table_name.clone())));
        paths.push(format!("{}/schema/{}.rs", base_path, business.filename.clone().unwrap_or(business.table_name.clone())));
        paths.push(format!("{}/service/{}_service.rs", base_path, business.filename.clone().unwrap_or(business.table_name.clone())));
        paths.push(format!("{}/crud/crud_{}.rs", base_path, business.filename.clone().unwrap_or(business.table_name.clone())));
        
        Ok(paths)
    }
    
    /// 数据库类型映射到Python类型
    fn map_db_type_to_python(db_type: &str) -> String {
        match db_type.to_lowercase().as_str() {
            "int" | "integer" | "tinyint" | "smallint" | "bigint" => "int",
            "varchar" | "char" | "text" | "longtext" => "str",
            "datetime" | "timestamp" => "datetime",
            "decimal" | "float" | "double" => "float",
            "bool" | "boolean" => "bool",
            _ => "str",
        }.to_string()
    }
    
    /// 数据库类型映射到TypeScript类型
    fn map_db_type_to_ts(db_type: &str) -> String {
        match db_type.to_lowercase().as_str() {
            "int" | "integer" | "tinyint" | "smallint" | "bigint" => "number",
            "varchar" | "char" | "text" | "longtext" => "string",
            "datetime" | "timestamp" => "string",
            "decimal" | "float" | "double" => "number",
            "bool" | "boolean" => "boolean",
            _ => "string",
        }.to_string()
    }
}
