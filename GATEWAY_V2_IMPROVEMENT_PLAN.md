# 🚀 gate_http v2 网关改进计划

**制定日期**: 2026/8/8  
**目标**: 将 v2 网关从 ⭐⭐⭐⭐ 优化到 ⭐⭐⭐⭐⭐  
**预计时间**: 10-15 小时  

---

## 📋 改进项清单

### 🔴 第一阶段: 核心优化 (4-6 小时)

#### 1. IM 模块拆分 (2-3 小时)

**目标**: 将 611 行的 gateway.rs 分解为 5 个子模块

```rust
// ✅ 新结构
router_v2/im/
├─ mod.rs           (路由入口)
├─ contact.rs       (联系人, ~100 lines)
├─ card.rs          (名片, ~95 lines)
├─ message.rs       (消息, ~145 lines)
├─ chat.rs          (聊天, ~125 lines)
└─ gateway.rs       (网关聚合, ~150 lines)
```

#### 2. 去重代码 (1-2 小时)

**目标**: 消除重复的处理模式

**创建通用处理函数**:
```rust
// router_v2/kit/handler.rs
pub async fn handle_list_api<F>(
    uid: i64,
    query: &GatewayQuery,
    auth: AuthContext,
    req: &HttpRequest,
    start: Instant,
    handler: F,
) -> impl Responder
where
    F: Fn(AuthContext, String, &AppContext) 
        -> BoxFuture<'static, Result<AppData<Vec<...>>, String>>,
{
    let url = ApiGatewayRequest {
        uid: Some(uid),
        page: query.page,
        qty: query.qty,
        ..Default::default()
    }
    .build();

    match handler(auth, url, ctx).await {
        Ok(data) => data.finish(req, start),
        Err(e) => AppData::err(500, e, None).finish(req, start),
    }
}
```

#### 3. 替换占位符 API (1-2 小时)

**目标**: 用真实的处理器替换 HomeApi::handler_get_new

**映射**:
```
"contact_add" -> ImApi::handler_contact_add
"contact_sync" -> ImApi::handler_contact_sync
"card_new" -> ImApi::handler_card_create
"message_send" -> ImApi::handler_message_send
"chat_add" -> ImApi::handler_chat_create
```

---

### 🟡 第二阶段: 质量提升 (4-6 小时)

#### 4. 添加参数验证 (2-3 小时)

**创建验证层**:
```rust
// router_v2/kit/validator.rs
pub fn validate_uid(uid: i64) -> Result<i64, String> {
    if uid <= 0 {
        Err("Invalid uid".to_string())
    } else {
        Ok(uid)
    }
}

pub fn validate_cmd<T>(cmd: Option<T>) -> Result<T, String> {
    cmd.ok_or_else(|| "Missing command".to_string())
}
```

#### 5. 添加结构化日志 (1-2 小时)

**在网关添加日志**:
```rust
async fn im_gateway(...) -> impl Responder {
    let start = Instant::now();
    
    info!("[🌐 GATEWAY] IM request: service={}, uid={}", 
          query.service, uid);
    
    match gateway_req.service.as_str() {
        "contact_add" => {
            match ImApi::handler_contact_add(...).await {
                Ok(data) => {
                    info!("[🗣️ API] ✅️ contact_add success: uid={}", uid);
                    AppData::ok(data).finish(&req, start)
                }
                Err(e) => {
                    error!("[🤐 API] ❌️ contact_add failed: {}", e);
                    AppData::err(500, e.to_string(), None)
                        .finish(&req, start)
                }
            }
        }
        _ => {
            warn!("[🌐 GATEWAY]: Unknown service: {}", gateway_req.service);
            AppData::<()>::err(400, "Unknown service", None)
                .finish(&req, start)
        }
    }
}
```

---

## 📊 改进效果

```
改进前:
├─ 总行数: 2211 行
├─ IM 模块: 611 行 (单文件)
├─ 代码重复: 8+ 处
├─ 占位符 API: 全部是 HomeApi::handler_get_new
└─ 日志: 无

改进后:
├─ 总行数: 2400+ 行 (添加验证和日志)
├─ IM 模块: 150 行 (gateway) + 5 子模块
├─ 代码重复: 0 处
├─ 占位符 API: 0 个
└─ 日志: 完整生命周期
```

---

## ✅ 验收标准

- [ ] IM 模块所有文件 < 150 行
- [ ] 代码重复: 0 处
- [ ] 占位符: 0 个
- [ ] 测试覆盖: > 90%
- [ ] 编译警告: 0

---

## 📝 预期收益

✅ 代码质量: ⭐⭐⭐⭐ -> ⭐⭐⭐⭐⭐  
✅ 维护成本: -40%  
✅ 新功能开发速度: +30%  
✅ Bug 修复周期: -40%  
✅ 技术债务: -80%  

**投入**: 10-15 小时  
**收益周期**: 6+ 个月  
**ROI**: 非常高 ✅

