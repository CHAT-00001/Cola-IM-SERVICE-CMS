// cola_video/src/live/gateway/live.rs -- 直播信息 - 数据实体映射
// 2026-02-06 21:00:32

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// ENTITY - 直播间实体
/// 兼容性标注：
/// 1. 主键使用 uid (映射 router 表 id)
/// 2. room_id 使用无分隔符 UUIDv4 (用于未来 IM/聊天室识别)
/// 3. 状态位统一使用 i16 以适配 PHP/MySQL 的 tinyint (0/1)
/// 4. 时间戳统一使用 i64 (Unix Timestamp)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveEntity {
    // --- 核心标识 ---
    pub uid: i64,                // 用户ID (主键，映射user表id)
    pub room_id: String,         // 直播间唯一标识 (无分隔符 UUIDv4)
    pub show_id: Option<String>, // 秀场ID / 房间逻辑ID
    pub title: Option<String>,   // 直播标题
    pub thumb: Option<String>,   // 直播封面图URL
    pub pull: Option<String>,    // 播流/拉流地址
    pub stream: Option<String>,  // 推流名称 (唯一标识流名)
    pub channel_id: Option<i32>, // 频道分类ID

    // --- 状态开关 (兼容PHP：0. 否 / 1. 是) ---
    pub is_video: Option<i16>,     // 是否视频直播
    pub is_mic: Option<i16>,       // 是否连麦
    pub is_hot: Option<i16>,       // 是否热门
    pub is_recommend: Option<i16>, // 是否推荐
    pub is_live: Option<i16>,      // 是否在直播
    pub is_shop: Option<i16>,      // 是否开启店铺
    pub is_off: Option<i16>,       // 是否手动关闭
    pub status: i16,               // 状态: 0.已关闭 1.直播中 2.禁播/审核中

    // --- 业务数值 ---
    pub good_num: Option<i64>,        // 挂载商品数量
    pub anyway: Option<i16>,          // 屏幕方向：0.横屏 1.竖屏
    pub hot_votes: Option<i64>,       // 当前热度值/票数
    pub gift_total_coin: Option<i64>, // 该场直播主播收入总金币
    pub gift_user_total: Option<i64>, // 该场直播观众消耗总金币
    pub banker_coin: Option<i64>,     // 庄家/游戏池金币

    // --- 互动与位置 ---
    pub pk_uid: Option<i64>,         // 正在PK的对象UID
    pub pk_stream: Option<String>,   // 正在PK的对方流名
    pub video_url: Option<String>,   // 回放视频地址
    pub province: Option<String>,    // 省份
    pub city: Option<String>,        // 城市
    pub address: Option<String>,     // 详细地理位置
    pub lng: Option<f64>,            // 经度
    pub lat: Option<f64>,            // 纬度
    pub live_type: Option<String>,   // 房间类型: 0.普通 1.付费 2.密码 3.门票
    pub type_val: Option<String>,    // 类型对应值: 如密码内容或门票价格
    pub device_info: Option<String>, // 开播设备信息 (ios/android)
    pub game_action: Option<String>, // 当前游戏状态/动作
    pub voice_type: Option<String>,  // 语音房细分类型

    // --- 第三方/CDN配置 ---
    pub sw_player_status: Option<i16>, // 播放器状态 (网宿等)
    pub sw_player_id: Option<String>,  // 播放器实例ID
    pub sw_pull_url: Option<String>,   // CDN拉流地址

    // --- 时间轴 (Unix 时间戳) ---
    pub start_time: Option<i64>,     // 开播时间戳 (秒)
    pub off_time: Option<i64>,       // 关播时间戳 (秒)
    pub recommend_time: Option<i64>, // 加入推荐时间戳 (秒)

    // --- 扩展字段 ---
    #[sqlx(default)]
    pub distance: Option<f64>, // 距离 (SQL 实时计算得出，单位：km)
}
