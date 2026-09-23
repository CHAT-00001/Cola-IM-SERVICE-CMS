# 🔐 用户权限系统 - 完整文档

**系统版本**: 1.0  
**开发时间**: 2026-09-19  
**完成度**: 80%（核心功能已实现）

---

## 📚 文档导航

| 文档 | 描述 | 适合人群 |
|------|------|--------|
| [快速开始](QUICK-START.md) | 5分钟快速上手 | 🔰 新手 |
| [完整规划](PLAN-2026-09-19-005900-user-permission.md) | 详细设计文档 | 👨‍💼 PM / 📐 架构师 |
| [实现总结](IMPLEMENTATION-SUMMARY.md) | 代码实现细节 | 👨‍💻 开发者 |
| [进度报告](PROGRESS-2026-09-19.md) | 开发进度跟踪 | 📊 管理者 |

---

## 🎯 系统概述

### 目标

构建一个**分层解耦**的权限系统：
- ✅ **Cola User** 负责系统级权限（等级 1-12）
- ✅ **业务模块**（Video/GIS/Live）负责私有权限
- ✅ **网关层**负责权限聚合
- ✅ **无 token = 游客权限 = 1**

### 核心特性

| 特性 | 说明 |
|------|------|
| **分层设计** | 网关 → API → CASE → SERVICE → REPO |
| **权限聚合** | 网关层一次性聚合，下层直接使用 |
| **游客支持** | 无 token 自动标记为权限=1 |
| **容错降级** | 权限查询失败时降级为游客 |
| **零耦合** | 业务模块不需要修改 cola_user |

---

## 🏗️ 架构设计

### 系统分层

```
┌─────────────────────────────────────┐
│  [🌐 GATEWAY] 权限聚合层            │
│  - 检查 token                        │
│  - 调用权限服务                      │
│  - 构建 AuthContext                 │
└─────────────────────────────────────┘
          ↓
┌─────────────────────────────────────┐
│  [🗣️ API] 业务处理层                 │
│  - 双重权限检查                      │
│  - 调用 CASE 层                      │
│  - 返回响应                          │
└─────────────────────────────────────┘
          ↓
┌─────────────────────────────────────┐
│  [🗣️ CASE] 逻辑编排层                │
│  - 业务逻辑（不处理权限）            │
│  - 调用 SERVICE 层                   │
└─────────────────────────────────────┘
          ↓
┌─────────────────────────────────────┐
│  [🔌 ADAPTER] 适配器层               │
│  - 调用权限查询服务                  │
│  - 权限缓存管理                      │
└─────────────────────────────────────┘
```

### 权限查询流程

```
无 token（游客）              有 token（登录用户）
      ↓                            ↓
网关自动处理               verify_login() → SessionContext
      ↓                            ↓
base_level = 1         UserPermissionQueryService.get()
权限 = GUEST                 ↓
      ↓              base_level >= 2
├─ 可读操作           权限 = USER/VIP/...
└─ 无写操作                  ↓
                      ├─ 可读操作
                      └─ 可写操作（取决于权限等级）
```

---

## ✨ 主要功能

### 1. 游客权限（权限 = 1）

**自动触发**: 请求无 `access_token`

**特点**:
- 自动设置权限等级 = 1
- 只能执行只读操作
- 网关层处理，业务层无感知

**代码示例**:
```rust
// 网关层
if !has_access_token {
    let perm_ctx = UserPermissionContext::guest();
    // base_level = 1
}

// 业务层
if auth.has_permission_level(2) {
    return Err("权限不足");  // 游客无法执行
}
```

### 2. 登录用户权限（权限 >= 2）

**自动触发**: 请求有 `access_token`

**特点**:
- 验证会话信息
- 查询 cola_user 权限中心
- 支持权限升级

**代码示例**:
```rust
// 网关层
if has_access_token {
    match SessionStateApi::verify_login() {
        Ok(session) => {
            perm_ctx = UserPermissionQueryService::get(uid).await;
            // base_level >= 2
        }
    }
}

// 业务层
if auth.has_permission_level(4) {
    // 创作者可执行此操作
}
```

### 3. 权限检查方法

```rust
// 方法 1: 检查权限等级
auth.has_permission_level(4)

// 方法 2: 检查系统权限
auth.has_sys_permission("video.publish")

// 方法 3: 检查角色
auth.has_role("broadcaster")
```

---

## 📦 代码实现

### 核心数据结构

**UserPermissionContext** (cola_data/src/cola_user/info/permission.rs)
```rust
pub struct UserPermissionContext {
    pub uid: i64,                        // 用户ID
    pub base_level: i16,                 // 权限等级 1-12
    pub level_name: String,              // 等级名称
    pub roles: Vec<String>,              // 拥有的角色
    pub sys_permissions: Vec<String>,    // 系统权限列表
    pub is_vip: bool,                    // 是否VIP
    pub is_creator: bool,                // 是否创作者
    pub is_operator: bool,               // 是否运营
    pub is_admin: bool,                  // 是否管理员
}
```

**扩展 AuthContext** (cola_data/src/auth/info/auth.rs)
```rust
pub struct AuthContext {
    pub uid: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub device_id: String,
    pub iam_roles: Vec<String>,
    pub is_anonymous: bool,
    
    // 🆕 权限信息
    pub permission_context: Option<UserPermissionContext>,
}
```

---

## 🧪 测试场景

### 场景 1: 游客获取弹幕

```bash
# 请求（无 token）
GET /api/v2/video/gateway?service=get_danmaku&video_id=1

# 预期
✅ 返回弹幕列表
📊 权限等级 = 1
```

### 场景 2: 游客发送弹幕失败

```bash
# 请求（无 token）
POST /api/v2/video/gateway
body: { "cmd": { "video_id": 1, "content": "test" } }

# 预期
❌ 返回权限拒绝
📊 权限等级 = 1
💬 错误信息: "权限不足"
```

### 场景 3: 登录用户发送弹幕

```bash
# 请求（有 token）
POST /api/v2/video/gateway
body: {
  "auth": { "access_token": "xxx" },
  "cmd": { "video_id": 1, "content": "test" }
}

# 预期
✅ 弹幕发送成功
📊 权限等级 >= 2
```

---

## 📊 权限等级对照表

| 等级 | 名称 | 标签 | 能力 |
|-----|------|------|------|
| 1 | GUEST | 👤 游客 | 只读 |
| 2 | USER | 👤 普通用户 | 基础写 |
| 3 | VIP | 👑 VIP会员 | VIP功能 |
| 4 | CREATOR | 🎬 创作者 | 发布视频 |
| 5 | MERCHANT | 🏪 商户 | 商业功能 |
| 6 | OP_JUNIOR | 👨‍💼 运营初级 | 基础运营 |
| 7 | OP_MID | 👨‍💼 运营中级 | 审核权限 |
| 8 | OP_SENIOR | 👨‍💼 运营高级 | 主管权限 |
| 10 | SYS_OPS | 🔧 系统运维 | 系统操作 |
| 12 | SUPER_ADMIN | 👑 超级管理员 | 全部权限 |

---

## 🚀 快速启动

### 1. 编译检查

```bash
cd d:\rust\short-video
cargo check -p cola_data
cargo check -p service
cargo check -p gate_http
```

### 2. 测试游客权限

```bash
# 无 token 请求
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=get_danmaku'

# 检查日志
# [🌐 GATEWAY]: 👤 游客访问 - 权限等级=1
```

### 3. 测试登录用户权限

```bash
# 有 token 请求
curl -X POST 'http://...' \
  -d '{"auth":{"access_token":"xxx"}}'

# 检查日志
# [🌐 GATEWAY]: ✅️ 权限查询成功 - uid=123, level=2
```

---

## ⚠️ 重要提示

### 游客权限 = 1

- ✅ 所有无 token 的请求自动标记为游客
- ✅ 游客权限严格限制
- ✅ 无法进行写操作

### 权限查询失败

- 降级为游客权限
- 不会拒绝请求
- 日志中记录警告

### 权限变更

- 升级后需清除缓存
- 新请求会查询新权限
- 实时生效

---

## 📈 开发进度

| 项目 | 状态 | 完成度 |
|------|------|--------|
| 核心数据结构 | ✅ 完成 | 100% |
| 网关层集成 | ✅ 完成 | 100% |
| 游客权限支持 | ✅ 完成 | 100% |
| 登录用户权限 | ✅ 完成 | 100% |
| 业务模块适配 | ⏳ 进行中 | 0% |
| 数据库实现 | ⏳ 计划中 | 0% |
| 集成测试 | ⏳ 待做 | 0% |

---

## 📞 支持

- 📖 详细技术文档：见上方文档导航
- 🐛 问题报告：在 PLAN 文件中记录
- 💬 讨论：修改此文件的备注部分

---

**版本**: 1.0 Alpha  
**最后更新**: 2026-09-19 01:20  
**维护者**: AI Development Team

