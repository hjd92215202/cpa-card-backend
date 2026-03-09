# Polymath Hub 🎓

一个基于Rust的**考试学习卡片管理系统**后端服务，支持学科管理、知识体系组织、学习卡片管理和智能复习计划。

## 📋 项目概述

为考试学习者设计的**考试学习卡片管理系统**，提供RESTful API接口用于：

- 📚 **学科管理** - 组织会计、税法、经济法等科目
- 📑 **知识体系** - 支持无限级树形章节结构  
- 🃏 **学习卡片** - 创建/编辑/查询知识卡片
- 🔍 **全文搜索** - 高性能的PostgreSQL全文搜索
- 📅 **复习追踪** - 自动计算复习间隔和下次复习日期
- 📊 **学习统计** - 记录复习历史和学习进度

## 🏗️ 系统架构

```
Polymath Hub
├── Models       - 数据模型定义（Card、Category、Subject）
├── Routes       - HTTP路由定义
├── Handlers     - 请求处理逻辑
├── Repository   - 数据访问层（DAO）
├── Services     - 业务逻辑层
├── Error        - 统一错误处理
└── Config       - 配置管理
```

### 数据库设计

四个核心表结构：

| 表名 | 说明 | 特性 |
|------|------|------|
| `subjects` | 科目表 | 主题色、图标、描述 |
| `categories` | 章节表 | 支持多级树形结构、排序 |
| `cards` | 卡片表 | 正面/背面内容、复习计划、全文搜索索引 |
| `review_logs` | 复习日志表 | 学习进度统计 |

**关键索引：**
- 全文搜索GIN索引 (`search_vector`)
- 复习日期B树索引 (`next_review_date`)
- 分类关联索引 (`category_id`)

## 🚀 快速启动

### 前置条件

- Rust 1.70+
- PostgreSQL 12+
- Cargo

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd cpa-card-backend

# 查看依赖
cat Cargo.toml
```

### 环境配置

创建 `.env` 文件：

```env
DATABASE_URL=postgresql://user:password@localhost:5432/cpa_cards
RUST_LOG=cpa_card_backend=debug,tower_http=debug
```

### 数据库初始化

```bash
# 创建数据库
createdb cpa_cards

# 执行初始化脚本
psql -U user -d cpa_cards -f init.sql
```

### 编译与运行

```bash
# 开发模式
cargo run

# 发布模式
cargo build --release
./target/release/cpa-card-backend
```

服务启动后监听 `http://127.0.0.1:3000`

## 📡 API文档

### 学科管理

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/subjects` | 获取所有科目 |
| POST | `/api/subjects` | 创建新科目 |
| GET | `/api/subjects/:id` | 获取科目详情 |

### 章节管理

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/categories` | 获取所有章节 |
| POST | `/api/categories` | 创建新章节 |
| GET | `/api/categories/:id` | 获取章节详情 |

### 学习卡片

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/api/cards` | 查询卡片（需指定 `category_id`） |
| GET | `/api/cards?category_id=1` | 按章节查询卡片 |
| GET | `/api/cards/by_subject?subject_id=1` | 按科目查询所有卡片 |
| GET | `/api/cards/search?keyword=关键词` | 全文搜索卡片 |
| GET | `/api/cards/:id` | 获取卡片详情 |
| POST | `/api/cards` | 创建新卡片 |
| PATCH | `/api/cards/:id/review` | 记录复习进度 |

### 请求/响应示例

**创建卡片：**

```bash
curl -X POST http://127.0.0.1:3000/api/cards \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "title": "会计的基本假设",
    "essence": "会计基本假设包括：持续经营、会计分期、货币计量、实质重于形式",
    "insights": "考试常考点！注意区分与会计核心概念的关系",
    "difficulty": 3,
    "importance": "A",
    "card_type": "question-answer"
  }'
```

**搜索卡片：**

```bash
curl http://127.0.0.1:3000/api/cards/search?keyword=会计分录
```

**记录复习进度：**

```bash
curl -X PATCH http://127.0.0.1:3000/api/cards/1/review \
  -H "Content-Type: application/json" \
  -d '{"interval_days": 3}'
```

## 🛠️ 技术栈

| 层级 | 技术 | 版本 |
|------|------|------|
| Web框架 | Axum | 0.7 |
| 异步运行时 | Tokio | 1.0 (full) |
| 数据库 | PostgreSQL | 12+ |
| ORM | SQLx | 0.8 |
| 日志 | tracing + tracing-subscriber | 0.3 |
| 中间件 | tower-http (CORS, Trace) | 0.5 |
| 序列化 | Serde + serde_json | 1.0 |
| 时间处理 | Chrono | 0.4 |
| 环境配置 | dotenvy | 0.15 |

**Rust版本要求：** 2024 edition

## 📝 项目结构详解

```
src/
├── main.rs              # 应用入口、Router配置、启动逻辑
├── config.rs            # 配置管理（预留）
├── error.rs             # 统一错误处理、HTTP状态映射
├── models/
│   ├── mod.rs
│   ├── card.rs          # Card数据模型 & CreateCardDto
│   ├── category.rs      # Category数据模型
│   └── subject.rs       # Subject数据模型
├── handlers/            # HTTP请求处理层
│   ├── mod.rs
│   ├── card_handler.rs  # 卡片操作处理器
│   ├── category_handler.rs
│   └── subject_handler.rs
├── routes/              # 路由定义层
│   ├── mod.rs
│   ├── card_routes.rs
│   ├── category_routes.rs
│   └── subject_routes.rs
├── repository/          # 数据访问层(DAO)
│   ├── mod.rs
│   ├── card_repo.rs     # 卡片数据库操作
│   ├── category_repo.rs
│   └── subject_repo.rs
└── services/            # 业务逻辑层（预留）
    ├── mod.rs
    └── subject_service.rs
```

## ✨ 核心特性

### 1️⃣ 多级章节支持
```sql
-- 支持无限级树形目录
categories.parent_id → 自引用外键
sort_order → 保证大纲排列顺序
```

### 2️⃣ 智能复习计划
```rust
pub struct Card {
    pub interval_days: i32,          // 用户设置的复习间隔
    pub next_review_date: NaiveDate, // 下次复习日期（自动计算）
    pub last_review_date: NaiveDate, // 上次复习日期
    pub review_count: i32,           // 累计复习次数
}
```

### 3️⃣ 全文搜索
自动生成搜索向量包含：
- 卡片标题 (`title`)
- 知识精华 (`essence`)
- 备考心得 (`insights`)

```sql
search_vector tsvector GENERATED ALWAYS AS (
  to_tsvector('simple', title || ' ' || coalesce(essence, '') || ' ' || coalesce(insights, ''))
)
```

### 4️⃣ 灵活的卡片类型
```rust
pub card_type: Option<String>  // "question-answer" / "note" / "code"
```

### 5️⃣ 自动化时间戳
```sql
-- 自动跟踪创建和修改时间
created_at TIMESTAMPTZ DEFAULT NOW()
updated_at TIMESTAMPTZ DEFAULT NOW()
-- 通过触发器保证自动更新
```

## 📊 复习管理逻辑

### 复习流程
1. 用户查看卡片内容（正面）
2. 用户查看答案（背面）
3. 调用 `PATCH /api/cards/:id/review` 更新复习进度
4. 后端计算 `next_review_date = NOW() + interval_days`
5. 系统记录 `review_logs` 表用于统计

### 复习查询
```bash
# 获取今天应复习的卡片
SELECT * FROM cards WHERE next_review_date <= CURRENT_DATE;
```

## 🔒 错误处理

统一的 `AppError` 枚举：

```rust
pub enum AppError {
    DatabaseError(sqlx::Error),      // 500 Internal Server Error
    NotFound,                         // 404 Not Found
    BadRequest(String),               // 400 Bad Request
}
```

所有错误响应格式：
```json
{
  "error": "错误信息描述"
}
```

## 🎯 日志系统

使用 `tracing` 框架记录：
- ✅ 数据库连接成功
- ✅ HTTP请求追踪
- ✅ 业务操作日志（创建、查询、更新）
- ⚠️ 警告和异常

日志配置：
```rust
tracing_subscriber::fmt()
    .with_env_filter("cpa_card_backend=debug,tower_http=debug")
    .init();
```

## 🚀 性能优化

| 优化项 | 实现 |
|--------|------|
| 全文搜索 | GIN索引 + PostgreSQL tsvector |
| 复习查询 | B树索引 on `next_review_date` |
| 分类查询 | 外键索引 on `category_id` |
| 连接池 | Tokio + 5个最大连接 |
| GZIP压缩 | tower-http内置支持 |

## 📋 待实现功能

- [ ] 用户认证与授权 (JWT)
- [ ] 批量导入卡片（Excel/JSON）
- [ ] 学习统计仪表板
- [ ] 图片/富媒体支持
- [ ] WebSocket实时更新
- [ ] 爬虫工具自动生成卡片
- [ ] 卡片版本控制与协作编辑

## 🤝 开发指南

### 添加新的API端点

1. **定义模型** (`src/models/`)
```rust
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MyEntity {
    pub id: i32,
    // ... fields
}
```

2. **实现DAO** (`src/repository/`)
```rust
impl MyRepository {
    pub async fn fetch_all(db: &PgPool) -> Result<Vec<MyEntity>, AppError> {
        sqlx::query_as("SELECT * FROM my_table")
            .fetch_all(db)
            .await
            .map_err(Into::into)
    }
}
```

3. **编写处理器** (`src/handlers/`)
```rust
#[instrument(skip(state))]
pub async fn list_all(
    State(state): State<AppState>,
) -> Result<Json<Vec<MyEntity>>, AppError> {
    let items = MyRepository::fetch_all(&state.db).await?;
    Ok(Json(items))
}
```

4. **配置路由** (`src/routes/`)
```rust
pub fn my_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(my_handler::list_all))
        .route("/", post(my_handler::create_one))
}
```

5. **注册路由** (`src/main.rs`)
```rust
let app = Router::new()
    .nest("/api/myentity", routes::my_routes::my_routes())
    // ...
```

### 测试

```bash
# 单元测试
cargo test

# 集成测试
cargo test --test '*'

# 查看日志
RUST_LOG=debug cargo run
```

## 📚 参考资源

- [Axum官方文档](https://github.com/tokio-rs/axum)
- [SQLx文档](https://github.com/launchbadge/sqlx)
- [Tokio运行时](https://tokio.rs)
- [PostgreSQL全文搜索](https://www.postgresql.org/docs/current/textsearch.html)

## 📄 许可证

MIT License

## 👨‍💻 贡献者

欢迎提交Issue和Pull Request！

---

**Last Updated:** 2026年3月9日  
**Rust Edition:** 2024  
**API Version:** v1
