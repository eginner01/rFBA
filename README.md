# FBA Rust

FBA Rust 是基于 [FastAPI Best Architecture](https://github.com/fastapi-practices/fastapi_best_architecture) 的 Rust 实现，专注于企业级高性能 Web 后端脚手架。

**前端项目**: [fastapi_best_architecture_ui](https://github.com/fastapi-practices/fastapi_best_architecture_ui)

---

## 🎉 最新更新日志

### 2025-11-17 - 数据权限系统重构完成

#### ✅ 数据范围 (Data Scope) 模块
- **重构核心架构**
  - ✅ 修正 `data_scope` entity 定义
  - ✅ 创建关联表 entities：`role_data_scope`（角色-数据范围）、`data_scope_rule`（数据范围-规则）
  - ✅ 废弃 `data_scope_repo`，service 层直接使用 entity

- **重构核心架构**
  - ✅ 完善插件管理（上传、下载）

- **完整 CRUD 功能实现**
  - ✅ `POST /api/v1/sys/data-scopes` - 创建数据范围（名称唯一性检查）
  - ✅ `PUT /api/v1/sys/data-scopes/{id}` - 更新数据范围（名称冲突检查）
  - ✅ `GET /api/v1/sys/data-scopes` - 分页查询数据范围列表
  - ✅ `GET /api/v1/sys/data-scopes/{id}` - 获取数据范围详情
  - ✅ `DELETE /api/v1/sys/data-scopes` - 批量删除数据范围
  - ✅ `PUT /api/v1/sys/data-scopes/{id}/rules` - 更新数据范围规则关联

- **修复分页响应格式** ⚠️ 重要
  - ✅ 添加分页元数据：`page`、`size`、`total_pages`
  - ✅ 实现数据库分页（使用 SeaORM Paginator）
  - 🔧 **解决前端无法显示数据的问题**

#### ✅ 数据规则 (Data Rule) 模块
- **完全重构**
  - ✅ 重写 `data_rule` entity
  - ✅ 重写所有 DTO（CreateDataRuleRequest, UpdateDataRuleRequest, DataRuleDetailResponse）
  - ✅ 重写 service 层所有方法：create, update, delete, get_list, get_all
  - ✅ 废弃 `data_rule_repo`，直接使用 entity + ActiveModel
  - ✅ 移除不存在的字段（code, columns, field_permissions 等）
  
- **Bug 修复**
  - ✅ 修复 422 错误：调整 `Extension<AuthContext>` 参数顺序
  - ✅ 修复更新逻辑：`id` 从路径参数获取，而非请求体
  - ✅ 修复部分更新：添加 `#[serde(default)]` 支持字段缺失

#### ✅ 用户 (User) 模块
- **权限更新接口修复**
  - ✅ 修改为只接收 `type` 参数
  - ✅ 实现 `update_permission_toggle` 方法，自动切换权限状态

#### ✅ 角色 (Role) 模块
- **DTO 修复**
  - ✅ 创建 `UpdateRoleMenuRequest` 和 `UpdateRoleScopeRequest`
  - ✅ 修复更新角色菜单和数据权限的请求体格式

#### ✅ 路由修复
- ✅ 修正 `data_scope` 路由注册路径


#### ✅ 中间件优化
- ✅ JWT 认证中间件添加详细日志
- ✅ 记录认证成功、失败、白名单放行等关键事件

---

### 🛠️ 技术要点

1. **使用关联表架构**
   - `sys_role_data_scope`：角色 ↔ 数据范围（多对多）
   - `sys_data_scope_rule`：数据范围 ↔ 数据规则（多对多）

2. **SeaORM 最佳实践**
   - 废弃自定义 Repository，直接使用 `EntityTrait` + `ActiveModelTrait`
   - 使用 `Paginator` 实现高效分页
   - 使用 `ActiveValue::Set/NotSet` 控制字段更新

3. **API 设计规范**
   - 完全对齐 Python 版本的 API 路径和参数
   - 使用标准的分页响应格式（items, total, page, size, total_pages）
   - 统一的错误处理和日志记录

4. **数据完整性保障**
   - 创建时检查名称唯一性
   - 更新时检查名称冲突（排除自身）
   - 删除时级联删除关联表数据

### 📝 待优化项
- ⏳ 角色数据权限配置（需要使用 `sys_role_data_scope` 关联表）
- ⏳ 用户数据权限查询和过滤功能
- ⏳ 数据权限树查询功能

---

## 快速开始

### 1. 克隆项目

```bash
git clone git@github.com:eginner01/FBA_Rust.git
cd FBA_Rust
```

### 2. 配置环境

```bash
cp .env.example .env
# 编辑 .env，配置数据库和 Redis 等连接信息
```

### 3. 数据库迁移

#### 3.1 初次使用（已有数据库）

如果你已经有现成的数据库，可以从数据库生成 Entity：

```bash
# 安装 SeaORM CLI
cargo install sea-orm-cli

# 从数据库生成所有表的 Entity
sea-orm-cli generate entity \
    --database-url "mysql://user:pass@localhost/dbname" \
    --output-dir src/database/entity \
    --with-serde both

# 生成后的文件在：src/database/entity/
```

#### 3.2 创建新迁移

```bash
# 1. 生成迁移文件（会在 migration/src/ 创建新文件）
cargo run --package migration -- generate create_users_table

# 2. 编辑迁移文件，定义表结构
# 文件位置：migration/src/m20250117_xxxxxx_create_users_table.rs

# 3. 在 migration/src/lib.rs 中注册迁移
# 添加：Box::new(m20250117_xxxxxx_create_users_table::Migration)

# 4. 应用迁移到数据库
cargo run --package migration -- up

# 5. 从数据库重新生成 Entity（推荐）
sea-orm-cli generate entity \
    --database-url "$DATABASE_URL" \
    --output-dir src/database/entity
```

#### 3.3 常用迁移命令

```bash
# 应用所有未执行的迁移
cargo run --package migration -- up

# 应用指定数量的迁移
cargo run --package migration -- up -n 1

# 回滚最后一次迁移
cargo run --package migration -- down

# 回滚指定数量的迁移
cargo run --package migration -- down -n 2

# 查看迁移状态
cargo run --package migration -- status

# 刷新数据库（删除所有表并重新应用）⚠️ 危险操作
cargo run --package migration -- fresh

# 回滚所有迁移后重新应用
cargo run --package migration -- refresh
```

#### 3.4 环境变量配置

在 `.env` 文件中设置：

```bash
# 数据库连接URL
DATABASE_URL=mysql://root:password@localhost:3306/fba_rust

# 或使用单独配置
DATABASE_TYPE=mysql
DATABASE_HOST=localhost
DATABASE_PORT=3306
DATABASE_NAME=fba_rust
DATABASE_USER=root
DATABASE_PASSWORD=password
```

### 4. 启动服务

```bash
# 开发模式
cargo run

# 生产模式（自动运行迁移）
RUN_MIGRATIONS=true cargo build --release
./target/release/fastapi_best_architecture_rust
```

## 功能概览

- Rust + Axum 高性能异步 Web 框架
- 支持 MySQL / PostgreSQL / SQLite
- 内置 JWT 认证与 RBAC 权限控制
- 集成 Redis 缓存与会话管理
- SeaORM Migration 数据库迁移系统
- 插件化架构，支持代码生成等扩展
- 完整操作日志 / 访问日志 / 错误日志

## 📚 数据库迁移详解

### 工作流程

FBA Rust 使用 **SeaORM Migration** 进行数据库结构管理。与 Python 的 Alembic 不同，SeaORM 采用 **Schema-First** 工作流：

```
手写迁移 → 应用到数据库 → 生成 Entity → 开发业务逻辑
```

### 迁移文件示例

```rust
// migration/src/m20250117_000001_create_users.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(ColumnDef::new(User::Id).big_integer().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User { Table, Id, Username }
```

### 最佳实践

1. **开发阶段**：频繁使用 `sea-orm-cli generate entity` 同步 Entity
2. **测试阶段**：在测试环境验证迁移的 `up` 和 `down` 方法
3. **生产部署**：先备份数据库，再应用迁移
4. **版本控制**：所有迁移文件必须提交到 Git

### 自动化脚本

```bash
# scripts/sync-db.sh - 同步数据库和代码
#!/bin/bash
set -e

echo "📦 应用迁移..."
cargo run --package migration -- up

echo "🔄 重新生成 Entity..."
sea-orm-cli generate entity \
    --database-url "$DATABASE_URL" \
    --output-dir src/database/entity \
    --with-serde both

echo "✅ 同步完成！"
```

## 仓库结构

```
fastapi_best_architecture_rust/
├── src/                    # 核心业务代码
│   ├── app/               # 业务模块（用户、角色、权限等）
│   ├── common/            # 公共组件（异常、响应、枚举等）
│   ├── core/              # 核心功能（配置、注册器等）
│   ├── database/          # 数据库相关（实体、仓储、连接）
│   ├── middleware/        # 中间件（JWT、CORS等）
│   └── utils/             # 工具类（加密、验证等）
├── migration/             # 数据库迁移包 ⭐
│   ├── src/
│   │   ├── lib.rs        # 迁移管理器（注册所有迁移）
│   │   ├── main.rs       # CLI 入口
│   │   └── m*.rs         # 迁移文件（手动编写）
│   ├── Cargo.toml        # 独立包配置
│   └── README.md         # 迁移说明
├── plugins/               # 可插拔插件
│   ├── code_generator/   # 代码生成器
│   ├── config/           # 配置管理
│   └── notice/           # 通知插件
├── sql/                   # 初始化与测试数据脚本
├── docs/                  # 完整文档 📖
│   ├── database_migration.md           # 迁移使用指南
│   ├── python_alembic_implementation.md # Python 实现对比
│   ├── rust_seaorm_implementation.md    # Rust 实现详解
│   └── rust_auto_migration.md          # 自动迁移讨论
├── .env.example           # 配置模板
└── README.md             # 项目说明
```

## 📄 License

MIT

## 🔗 相关项目

- [FastAPI Best Architecture](https://github.com/fastapi-practices/fastapi_best_architecture) - 原 Python 版本
