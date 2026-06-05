// cola_video/src/live/gateway/channel.rs -- 直播频道实体映射表
// 2026-03-11 11:18:02






/// # 直播频道
pub struct LiveChannel {
    pub id: i64,                // ID
    pub icon: Option<String>,   // 图标
    pub bg_img: Option<String>, // 背景图
    pub name: String,           // 名称
    pub name_en: String,        // 英文名称
    pub description: String,    // 描述
    pub description_en: String, // 描述英文
}
