# 🚀 IM 网关快速参考指南

**重构完成**: 2026/8/8  
**状态**: ✅ 已就绪

---

## 📂 新文件结构

```
gate_http/src/router_v2/im/
├─ mod.rs           ← 模块声明
├─ gateway.rs       ← 主分发器 ⭐
├─ contact.rs       ← 联系人处理
├─ card.rs          ← 名片处理
├─ message.rs       ← 消息处理
├─ chat.rs          ← 聊天处理
└─ helper.rs        ← 工具函数
```

---

## 🔄 请求流程

```
HTTP Request: POST /api/v2/cola_im/gateway?service=contact_add

    ↓

gateway.rs::im_gateway()
    ├─ 解析 service: "contact_add"
    ├─ 提取 uid, auth
    └─ 匹配前缀 "contact_*" → contact_dispatch()

    ↓

contact.rs::contact_dispatch()
    ├─ 匹配 "contact_add"
    └─ 调用 handle_contact_add()

    ↓

handle_contact_add()
    ├─ 构建请求
    ├─ 调用 API
    └─ 返回响应
```

---

## 💡 常见操作

### 添加新的联系人操作

```rust
// 1. 在 contact.rs 中添加分发规则
"contact_xxx" => handle_contact_xxx(...).await

// 2. 实现处理函数
async fn handle_contact_xxx(
    req: &HttpRequest,
    query: &GatewayQuery,
    state: &web::Data<AppState>,
    auth: AuthContext,
    uid: i64,
    start: Instant,
) -> impl Responder {
    // 实现业务逻辑
}
```

### 在现有操作中添加验证

```rust
// 在 handle_xxx 函数中添加
let cmd: SomeCommand = helper::extract_cmd(&body)
    .ok_or_else(|| "Invalid command".to_string())?;

// 验证 cmd 的有效性
if cmd.is_invalid() {
    return AppData::err(400, "Invalid params", None)
        .finish(req, start);
}
```

### 提取公共工具函数

```rust
// 1. 在 helper.rs 中添加
pub fn extract_phone(cmd: &Command) -> Option<String> {
    // 实现
}

// 2. 在各 dispatcher 中使用
let phone = helper::extract_phone(&cmd)?;
```

---

## 📊 模块职责速查表

| 模块 | 文件 | 前缀 | 操作数 | 用途 |
|------|------|------|--------|------|
| Contact | contact.rs | `contact_*` | 8 | 联系人管理 |
| Card | card.rs | `card_*` | 6 | 用户名片 |
| Message | message.rs | `message_*` | 9 | 消息管理 |
| Chat | chat.rs | `chat_*` | 7 | 聊天管理 |

---

## ⚠️ 注意事项

### ✅ 推荐做法

- ✅ 在对应的分发器中添加新操作
- ✅ 在 helper.rs 中放通用工具
- ✅ 使用统一的参数处理方式
- ✅ 保持函数签名一致

### ❌ 避免做法

- ❌ 在 gateway.rs 中添加复杂逻辑
- ❌ 分发器之间互相调用
- ❌ 绕过 dispatcher 直接处理
- ❌ 添加模块级别的全局状态

---

## 🧪 测试建议

### 单元测试示例

```rust
// contact.rs 中
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_contact_add() {
        // 测试逻辑
    }
}
```

### 集成测试

```bash
curl -X POST "http://localhost:8000/api/v2/cola_im/gateway" \
  -H "Content-Type: application/json" \
  -d '{"service": "contact_add", "page": 1, "qty": 20}'
```

---

## 📞 支持服务速查

### Contact (1xxx)
- `contact_add` - 添加联系人
- `contact_sync` - 同步联系人
- `contact_del` - 删除联系人
- `contact_list` - 联系人列表
- `contact_search` - 搜索联系人
- `contact_detail` - 联系人详情
- `contact_update` - 修改联系人
- `contact_black_list` - 黑名单

### Card (2xxx)
- `card_get` - 获取名片
- `card_update` - 更新名片
- `card_batch` - 批量获取
- `card_search` - 搜索名片
- `card_delete` - 删除名片
- `card_visibility` - 名片可见性

### Message (3xxx)
- `message_send` - 发送消息
- `message_list` - 消息列表
- `message_mark_read` - 标记已读
- `message_delete` - 删除消息
- `message_recall` - 消息撤回
- `message_edit` - 编辑消息
- `message_search` - 搜索消息
- `message_forward` - 转发消息
- `message_unread_count` - 未读数

### Chat (4xxx)
- `chat_add` - 新增聊天
- `chat_del` - 删除聊天
- `chat_list` - 聊天列表
- `chat_manage` - 管理聊天
- `chat_setting` - 聊天设置
- `chat_pin` - 置顶聊天
- `chat_sync` - 同步聊天

---

## 🔧 常见问题

### Q: 如何添加新模块？

A: 
1. 创建 `newmodule.rs` 文件
2. 实现 `newmodule_dispatch()` 函数
3. 在 `mod.rs` 中声明 `pub mod newmodule;`
4. 在 `gateway.rs` 中添加匹配规则

### Q: 如何修改现有操作的逻辑？

A: 只需修改对应模块的 handle 函数即可，无需触及其他模块。

### Q: 性能会不会下降？

A: 不会。分发逻辑完全相同，只是代码组织更清晰。

### Q: 向后兼容吗？

A: 完全兼容。API 接口完全不变。

---

**快速开始**: 查看对应模块的 dispatcher 函数理解流程  
**遇到问题**: 参考 IM_GATEWAY_REFACTOR_SUMMARY.md 文档

