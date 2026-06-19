// command/live  -- 数据中心 - 命令层 - 开播记录
// 2026/6/12 23:22

////////

use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::live::entity::record::LiveStreamRecordEntity;
use crate::live::utils::record::build_stream_name;

////////

/// # [COMMAND] - 直播记录创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStreamRecordCommand {
    pub uid: i64,                    // 用户ID (主播)
    pub room_id: i64,                // 直播间唯一标识
    pub title: Option<String>,       // 本场直播标题
    pub province: Option<String>,    // 省份
    pub city: Option<String>,        // 城市
    pub address: Option<String>,     // 详细地址
    pub lng: Option<f64>,            // 经度
    pub lat: Option<f64>,            // 纬度
    pub thumb: Option<String>,       // 直播封面图
    pub channel_id: Option<i32>,     // 频道分类ID
    pub live_type: Option<String>,   // 房间类型: 0.普通 1.付费 2.密码 3.门票
    pub type_val: Option<String>,    // 类型对应值: 如密码内容或门票价格
    pub anyway: Option<i16>,         // 屏幕方向：0.横屏 1.竖屏
    pub device_info: Option<String>, // 开播设备信息 (ios/android)
    pub voice_type: Option<String>,  // 聊天室类型 0语音 1 视频
}


/// # [BUILD] - 绑定实体并自动生成默认值

impl LiveStreamRecordCommand {
    /// 构造函数：关联表结构，全自动计算并填充所有默认值，生成第一个干净合法的直播记录
    pub fn new(cmd: Self, push_domain: &str, play_domain_flv: &str) -> LiveStreamRecordEntity {
        let now_ts = Utc::now().timestamp() as i32;

        // 1. 🔥 自动调用刚写好的工具函数，生成绝不重复的流名称
        let stream_name = build_stream_name(cmd.uid, cmd.room_id);

        // 2. ⚡️ 组装动态推流与拉流地址
        // 实际开发中，你可以在这里根据厂商逻辑拼接真实的 txSecret 鉴权签名，这里做标准拼装
        let push_url = format!("rtmp://{}/live/{}", push_domain, stream_name);
        let pull_url = format!("http://{}/live/{}.flv", play_domain_flv, stream_name);

        // 3. 🎯 返回组装完毕的完整表结构实体（Entity）
        LiveStreamRecordEntity {
            id: 0,                         // 数据库自增主键，初始化填 0
            uid: cmd.uid,
            room_id: cmd.room_id,
            showid: now_ts as i64,         // 直播标识，传统做法常用当前开播时间戳代表 showid
            nums: 0,                       // 刚开播，当前人数自然是 0
            starttime: now_ts,             // 开播时间
            endtime: 0,                    // 未关播，结束时间给 0
            title: cmd.title,
            province: cmd.province,
            city: cmd.city,
            address: cmd.address,
            lng: cmd.lng,
            lat: cmd.lat,
            thumb: cmd.thumb,
            channel_id: cmd.channel_id,
            device_info: cmd.device_info,
            live_type: cmd.live_type.or(Some("0".to_string())), // 默认普通房间
            type_val: cmd.type_val,
            anyway: cmd.anyway.or(Some(1)),                     // 默认竖屏
            voice_type: cmd.voice_type.or(Some("1".to_string())), // 默认视频直播

            // 📡 流媒体核心注入
            stream: Some(stream_name.clone()),
            push_url,
            pull: Some(pull_url),

            // 🛡️ 业务状态初始化
            status: 1,                     // 📌 核心状态：1 代表直播中！
            is_mic: Some(0),               // 默认关麦
            is_hot: Some(0),               // 默认非热门
            is_recommend: Some(0),         // 默认不推荐
            is_off: Some(0),               // 默认没有被动关闭

            // 📊 计数控制全清零
            likes: 0,
            recommends: 0,
            good_num: Some(0),
            hot_votes: Some(0),
            gift_total_coin: Some(0),
            gift_user_total: Some(0),
            banker_coin: Some(0),

            // ⚔️ 连麦/PK 字段初始化全留空
            pk_uid: None,
            pk_stream: None,
            video_url: None,               // 回放等关播后由云厂商 Webhook 异步回调补上

            // 🔌 兼容老项目里的混淆字段，安全给 0 或 None
            pull_flv: Some(0),
            pull_hls: Some(0),
            sw_player_status: Some(0),
            sw_player_id: None,
            sw_pull_url: None,
            recommend_time: None,
            distance: None,                // 经纬度计算字段，默认忽略
            game_action: None,
        }
    }
}