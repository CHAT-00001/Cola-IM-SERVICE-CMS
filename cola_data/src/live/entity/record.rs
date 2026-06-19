// handler/record.rs  - handler 直播记录
// 2026/6/13 04:38

////////

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 直播流记录 实体
/// * `table_name` live_stream_record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LiveStreamRecordEntity {
    pub id: i64,                       // ID
    pub uid: i64,                      // 用户ID (主键，映射user表id)
    pub room_id: i64,                  // 直播间唯一标识 (无分隔符 UUIDv4)
    pub showid: i64,                   // 直播标识
    pub nums: i64,                     // 关播时人数
    pub starttime: i32,                // 开始时间
    pub endtime: i32,                  // 结束时间
    pub title: Option<String>,         // 标题
    pub province: Option<String>,      // 省份
    pub city: Option<String>,          // 城市
    pub thumb: Option<String>,         // 直播封面图URL
    pub pull: Option<String>,          // 播流/拉流地址
    pub stream: Option<String>,        // 推流名称 (唯一标识流名)
    pub channel_id: Option<i32>,       // 频道分类ID
    pub push_url: String,              // 推流URL
    pub pull_flv: Option<i16>,         // 是否连麦
    pub pull_hls: Option<i16>,         // 是否热门
    pub is_mic: Option<i16>,           // 连麦开关(默认0)
    pub is_hot: Option<i16>,           // 是否热门(默认0)
    pub is_recommend: Option<i16>,     // 是否推荐(默认0)
    pub likes: i64,                    // 点赞数量(默认0)
    pub recommends: i64,               // 推荐数量(默认0)
    pub is_off: Option<i16>,           // 是否手动关闭
    pub status: i16,                   // 状态: 0.已关闭 1.直播中 2.禁播/审核中
    pub good_num: Option<i64>,         // 挂载商品数量(默认0)
    pub anyway: Option<i16>,           // 屏幕方向：0.横屏 1.竖屏
    pub hot_votes: Option<i64>,        // 当前热度值/票数(默认0)
    pub gift_total_coin: Option<i64>,  // 该场直播主播收入总金币(默认0)
    pub gift_user_total: Option<i64>,  // 该场直播观众消耗总金币(默认0)
    pub banker_coin: Option<i64>,      // 庄家/游戏池金币(默认0)
    pub pk_uid: Option<i64>,           // 正在PK的对象UID
    pub pk_stream: Option<String>,     // 正在PK的对方流名
    pub video_url: Option<String>,     // 回放视频地址(关播后等第三方回执)
    pub address: Option<String>,       // 详细地理位置
    pub lng: Option<f64>,              // 经度
    pub lat: Option<f64>,              // 纬度
    pub live_type: Option<String>,     // 房间类型: 0.普通 1.付费 2.密码 3.门票
    pub type_val: Option<String>,      // 类型对应值: 如密码内容或门票价格
    pub device_info: Option<String>,   // 开播设备信息 (ios/android)
    pub game_action: Option<String>,   // 当前游戏状态/动作
    pub voice_type: Option<String>,    // 聊天室类型 0语音 1 视频
    pub sw_player_status: Option<i16>, // 播放器状态 (网宿等)
    pub sw_player_id: Option<String>,  // 播放器实例ID
    pub sw_pull_url: Option<String>,   // CDN拉流地址
    pub recommend_time: Option<i64>,   // 加入推荐时间戳 (秒)
    #[sqlx(default)]
    pub distance: Option<f64>, // 距离 (SQL 实时计算得出，单位：km)
}
