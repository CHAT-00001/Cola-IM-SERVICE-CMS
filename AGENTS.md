# 🤖 AGENTS 编码规范约束

> 本项目所有 Rust 代码必须严格遵守以下规范，AI 助手与开发者共同遵循。

---

## 1. 🔤 代码块分隔符（强制）

- **必须使用 8 个斜杠 `////////`** 作为代码块分隔符，**禁止使用 6 个斜杠 `//////`**
- 文件内各逻辑段之间使用 `////////` 分隔
- 文件末尾使用 `//////// END` 标记结束（同样 8 个斜杠)

**正确：**
```rust
////////

use xxx::yyy;

////////

pub fn foo() {}

//////// END
```

**错误（禁止）：**
```rust
//////

use xxx::yyy;

//////
```

---

## 2. 🗂️ 文件头部注释（强制）

每个源文件顶部必须有注释头，格式：
```rust
// 完整路径  -- 简短说明
// 日期 时间
```

示例：
```rust
// cola_data/src/auth/port/session.rs  -- 数据 - AUTH - 会话接口
// 2026/8/1 10:25
```

---

## 3. 🔤 编码与字符（强制）

- 所有中文注释必须使用 **UTF-8** 编码，**禁止 GBK**
- 注释中使用 emoji 标注模块职责（如 `🚧` 开发中、`✅` 完成、`🌟` 重点）

---

## 4. 🏗️ 分层架构（强制）

请求自上而下的层级，各层职责不可混淆：

| 层级 | 职责 |
|------|------|
| `GATEWAY` | web 框架（Actix），解析/合并参数、前置身份校验、按 service 分发 |
| `DISPATCHER` | 纯路由转发器，只按 action 路由到 handler，**不做业务/验证**，返回 `AppData<Value>` |
| `API HANDLER` | 应用权限编排、统一响应壳 `AppData<T>` |
| `CASE` | 逻辑流程编排，调用 Port 接口 |
| `PORT` | 定义 trait 接口（如 `SessionPort`、`TypePort`） |
| `ADAPTER` | 实现 Port trait，包装 repository 静态服务 |
| `REPOSITORY` | 仓储层（pg / redis / grpc / mock） |

**分层原则：**
- 网关层负责：URL + Body(JSON) 双重解析（Body 为主）、`verify_session` 会话验证、分发
- 转发器层（dispatcher）**不得自带会话验证**，session 由 gateway 传入
- 底层函数需要什么参数自己从链路中取，不层层透传整包

---

## 5. 🔌 Port / Adapter 规范（强制）

- `cola_data` 中定义 `trait`（如 `SessionPort`），配 `Arc<dyn Trait + Send + Sync + 'static>` 注入结构体
- `repo_adapter` 层实现该 trait，并通过 `build_app_context()` 统一装配
- trait 方法用 `#[async_trait::async_trait]` 标注

---

## 6. 🌐 网关参数规范（强制）

- 统一使用 `cola_data::app::query::ApiGatewayRequest` 作为网关请求体
- 禁止在网关层自建私有解析参数结构体（如 `GatewayQuery`）
- URL 与 Body(JSON) 双重命中，**Body 为主**：`url_req.merge(body_req)` 字段级合并

---

## 7. 🎯 命名规范（建议）

- 动作码常量集中在 `dispatcher/xxx.rs` 的 `pub mod action` 中
- 转发器命名 `xxx_dispatch`，handler 命名 `xxx_xxx`
- Port 结构体命名 `XxxPorts`（复数，含多个 Arc 端口）
- Adapter 命名 `XxxPortAdapter`

---

## 8. ✅ 其他约定

- 分页字段：请求用 `page` / `qty`，后端计算 `limit` / `offset`（由 `ApiGatewayRequest::build()` 处理）
- 统一响应壳：`AppData<T>`（`AppData::ok` / `AppData::err` / `AppData::empty`）
- 会话上下文：`SessionContext`（请求侧可信会话），`AuthContext`（业务侧登录态），两者职责区分