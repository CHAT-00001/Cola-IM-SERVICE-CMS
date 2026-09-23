// cola_infra/src/api/mod.rs  -- INFRA - api - mod
// 2026/4/13 01:25 by wx: cestbon10080

////////

pub mod endpoint; // 接入点
pub mod gate; // 网关
pub mod node; // 节点
pub mod region; // 区域
pub mod service; // 服务

// # [INFRA] - 基础设施说明
// * 🌐 Gate: 全球网关, 可嗅探, 可刷新
// * 🍚 Region: 区域 -- 例: EastAsia(东南亚)
// * 💡 Node: 节点机房 -- 例: SGR-1 (新加坡1)
// * 🔌 Endpoint: 接入点 -- 例: SGR-1-01(新加坡1-01)
// * 👤 Service: 服务 -- 例: API / IM / PG / MG / Redis / ..
// * 🖥️ Machine: 物理机器实例 -- 例: SGR-1-01-IM-1dec887ab

//////// END
