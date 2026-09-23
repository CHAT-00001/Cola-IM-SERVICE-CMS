# 权限系统快速开始

**版本**: 1.0  
**最后更新**: 2026-09-19 01:20

---

## ⚡ 5分钟入门

### 1️⃣ 游客用户（无 token）

```bash
# 请求示例
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=get_danmaku&video_id=1'

# 网关自动处理：
# - 检测无 access_token
# - 设置权限等级 = 1 (GUEST)
# - 调用 UserPermissionContext::guest()

# 业务层检查：
if auth.has_permission_level(2) {  // false
    // 拒绝发送弹幕（需要权限 >= 2）
}
```

### 2️⃣ 登录用户（有 token）

```bash
# 请求示例
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -d '{
    "auth": { "access_token": "xxx" },
    "cmd": { "video_id": 1, ... }
  }'

# 网关自动处理：
# - 检测有 access_token
# - 调用 SessionStateApi::verify_login()
# - 调用 UserPermissionQueryService::get_user_permission_context(uid)
# - 返回用户权限等级 >= 2

# 业务层检查：
if auth.has_permission_level(2) {  // true
    // 允许发送弹幕
}
```

### 3️⃣ 权限检查方法

```rust
// 方法1：检查权限等级
if auth.has_permission_level(4) {
    // 创作者权限 (>= 4)
}

// 方法2：检查系统权限
if auth.has_sys_permission("video.publish") {
    // 可发布视频
}

// 方法3：检查角色
if auth.has_role("broadcaster") {
    // 是主播
}
```

---

## 🔑 关键概念

### 权限等级（1-12）

| 等级 | 名称 | 场景 |
|-----|------|------|
| **1** | 👤 GUEST | 游客（无 token） |
| **2** | 👤 USER | 普通用户 |
| **3** | 👑 VIP | VIP会员 |
| **4** | 🎬 CREATOR | 创作者 |
| **6** | 👨‍💼 OPERATOR | 运营人员 |
| **10** | 🔧 SYS_OPS | 系统运维 |
| **12** | 👑 SUPER_ADMIN | 超级管理员 |

### 核心流程

```
客户端请求
    ↓
网关层：检查 access_token
    ├─ 有 token → 查询权限 → base_level >= 2
    └─ 无 token → 游客权限 → base_level = 1
    ↓
业务层：检查权限后执行
    ├─ 有权限 → 执行操作
    └─ 无权限 → 返回 403 Forbidden
    ↓
返回结果
```

---

## 🧪 快速测试

### 测试1：游客能否发送弹幕？

```bash
# 无 token 的请求
curl 'http://..../gateway?service=send_danmaku&video_id=1&content=test'

# 预期结果：
# ❌ 失败
# 原因：权限等级 = 1，无法执行写操作
# 错误码：403 或 4003
```

### 测试2：登录用户是否能发送弹幕？

```bash
# 有 token 的请求
curl -X POST 'http://..../gateway' \
  -d '{
    "auth": { "access_token": "valid_token" },
    "cmd": { "video_id": 1, "content": "test" }
  }'

# 预期结果：
# ✅ 成功
# 原因：权限等级 >= 2，可以发送弹幕
```

### 测试3：权限检查日志

```bash
# 查看网关层日志
# 游客日志：
# [🌐 GATEWAY]: 👤 游客访问 - 权限等级=1

# 登录用户日志：
# [🌐 GATEWAY]: ✅️ 权限查询成功 - uid=123, level=2

# 权限查询失败时：
# [🌐 GATEWAY]: ⚠️ 权限查询失败: ..., 使用游客权限
```

---

## 📖 常见问题

### Q1: 游客真的没有权限吗？

**A**: 对的。游客权限等级 = 1，只能执行明确允许游客的操作（如只读接口）。

### Q2: 无 token 请求会被拒绝吗？

**A**: 不会。网关会自动将其标记为游客，后续根据权限决定是否允许。

### Q3: 权限查询失败怎么办？

**A**: 网关层会降级为游客权限，保证系统可用性，并在日志中记录警告。

### Q4: 如何升级用户权限？

**A**: 通过 cola_user 的权限管理接口升级（管理员操作）。升级后缓存会自动清除，新请求会查询新权限。

### Q5: 业务模块如何使用权限？

**A**: 在 API 或 CASE 层检查 `auth.has_permission_level()` 或业务模块自定义的权限函数。

---

## 🚀 下一步

### 立即可做

- ✅ 测试游客请求（无 token）
- ✅ 测试登录请求（有 token）
- ✅ 查看日志验证权限流程

### 近期待做

- ⏳ 为业务模块添加私有权限检查（Cola Video）
- ⏳ 创建权限管理接口
- ⏳ 完成集成测试

### 中期待做

- ⏳ 实现数据库存储
- ⏳ 性能优化（缓存）
- ⏳ 权限审计报告

---

## 📞 更多帮助

- 📄 详细设计文档: `/PLAN/PLAN-2026-09-19-005900-user-permission.md`
- 📊 实现总结: `/PLAN/IMPLEMENTATION-SUMMARY.md`  
- 📈 进度报告: `/PLAN/PROGRESS-2026-09-19.md`
- 🧪 测试报告: `/PLAN/test-permission-system.md`

---

**记住**: 网关层负责权限聚合，业务层只需检查权限！🎯

