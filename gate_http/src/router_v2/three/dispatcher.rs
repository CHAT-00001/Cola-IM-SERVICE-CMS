// gate_http/src/v2/three/dispatcher.rs  -- 路由分发器
// 2026/7/24 06:19

////////

pub mod bucket; // 存储桶
pub mod cdn; // 分发网络
pub mod fs; // 文件存储
pub mod pay; // 支付
pub mod signin; // 登录
pub mod sms; // 短信
pub mod stream; // 推流
