# 用户权限系统开发计划

**文件**: PLAN-2026-09-19-005900-user-permission.md  
**创建时间**: 2026-09-19 00:59:00  
**状态**: 开发中  
**优先级**: 🔴 高

---

## 📋 项目概述

构建分层解耦的权限系统，将系统级权限管理放在 **cola_user** 核心模块中，业务级权限管理分散到各业务模块。

### 核心需求

1. **Cola User 权限中心**
   - 权限等级体系（1-12）
   - 全局角色管理
   - 用户权限配置聚合
   - 权限查询和缓存

2. **业务模块私有权限**
   - Cola Video: video.publish, danmaku.send 等
   - Cola GIS: poi_add, poi_edit 等
   - 各模块独立定义，不依赖 cola_user 修改

3. **网关层权限聚合**
   - 获取 access_token → SessionContext
   - 查询 cola_user → UserPermissionContext
   - 构建 AuthContext（包含权限信息）
   - 游客用户（无 token）默认权限为 1

4. **权限流转链路**
   - 网关层：前置权限检查 + 权限聚合
   - API 层：双重检查 + 日志记录
   - CASE 层：业务逻辑执行
   - SERVICE 层：操作记录

---

## 🗂️ 架构设计

### 分层模型

```
Cola User（核心权限中心）
├─ 权限等级定义（1-12）
├─ 全局角色定义
├─ 用户权限配置
└─ 权限查询服务
      ↑
      │ 依赖
      │
┌─────┴──────────┬──────────┬──────────┐
│                │          │          │
Cola Video    Cola GIS   Cola Live  Cola Comment
├─ 私有权限    ├─ 私有    ├─ 私有    ├─ 私有
└─ 检查等级    │ 权限     │ 权限     │ 权限
```

---

## 📝 开发任务清单

### Phase 1: Cola User 核心实现

#### 1.1 数据库迁移 (migrations)
- [ ] 创建权限等级表
- [ ] 创建角色定义表
- [ ] 创建用户权限配置表
- [ ] 创建权限审计日志表
- [ ] 初始化权限等级数据（1-12）

#### 1.2 数据结构 (cola_data)
- [ ] `entity/permission_level.rs` - 权限等级实体
- [ ] `entity/role_definition.rs` - 角色定义实体
- [ ] `entity/permission_config.rs` - 用户权限配置实体
- [ ] `info/permission.rs` - UserPermissionContext 信息结构

#### 1.3 Repository 仓储层
- [ ] `user_permission_config` 仓储

#### 1.4 Service 业务层
- [ ] `PermissionQueryService` - 权限查询
- [ ] Redis 缓存集成

#### 1.5 Port/Adapter 模式
- [ ] `repo_adapter/cola_user/permission.rs` - 适配器实现

### Phase 2: 网关层集成

#### 2.1 AuthContext 扩展
- [ ] 扩展 `AuthContext` 添加权限字段
- [ ] 添加权限检查方法

#### 2.2 网关层修改
- [ ] `gate_http/router_v2/video/gateway.rs`
  - 调用权限查询服务
  - 构建完整的 AuthContext
  - 无 token 时默认游客权限（权限=1）
  - 权限前置检查示例

### Phase 3: 业务模块私有权限

#### 3.1 Cola Video
- [ ] `permission/mod.rs` - 私有权限定义
- [ ] API 层集成权限检查

### Phase 4: 测试

- [ ] 单元测试
- [ ] 集成测试
- [ ] 无 token 游客权限测试

---

## 🎯 权限等级定义

| 等级 | 名称 | 说明 |
|-----|------|------|
| 1 | GUEST | 游客（未登录） |
| 2 | USER | 普通用户 |
| 3 | VIP | VIP会员 |
| 4 | CREATOR | 签约创作者 |
| 5 | MERCHANT | 特许商户 |
| 6 | OP_JUNIOR | 运营初级 |
| 7 | OP_MID | 运营中级 |
| 8 | OP_SENIOR | 运营高级 |
| 10 | SYS_OPS | 系统运维 |
| 12 | SUPER_ADMIN | 超级管理员 |

---

## 🔒 关键实现点

### 1. 无 Token 时的游客权限

```rust
if auth_request.access_token.is_none() || 
   auth_request.access_token.as_ref().unwrap().is_empty() {
    // 游客用户，权限等级 = 1
    let guest_perm = UserPermissionContext::guest();
}
```

### 2. 权限查询流程

```
网关接收 access_token
  ↓
verify_login() → SessionContext (uid)
  ↓
get_user_permission_context(uid) → UserPermissionContext
  ↓
构建 AuthContext（包含权限信息）
  ↓
权限前置检查 → 通过则分发到 API
```

---

## ✅ 验收标准

- [ ] 权限系统与业务模块完全解耦
- [ ] 网关层能正确聚合用户权限
- [ ] 无 token 时自动标记为游客（权限=1）
- [ ] 权限检查在网关层和 API 层双重执行
- [ ] 接口响应时间 < 100ms

---

**更新日志**

| 时间 | 操作 | 说明 |
|------|------|------|
| 2026-09-19 | 创建 | 初始化开发计划 |
