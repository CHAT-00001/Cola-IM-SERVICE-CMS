// gate_http/src/router_v2/three/dispatcher/mod.rs  -- 第三方服务分发器
// 2026/7/27

////////

pub mod bucket; // 存储桶
pub mod category; // 服务分类/类型管理
pub mod cdn; // 内容分发
pub mod email; // 邮件
pub mod fs; // 文件存储
pub mod im; // IM 第三方
pub mod pay; // 支付
pub mod provider; // 服务商
pub mod sign; // 第三方登录
pub mod sms; // 短信
pub mod stream; // 推流
