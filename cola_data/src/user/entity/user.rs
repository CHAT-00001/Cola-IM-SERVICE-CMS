// cola_data/src/user/entity/user.rs  -- USER - Entity - 用户表
// 2026/3/30 05:33

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 用户 - 用户表
/// * table name: user
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub id: i64,                 // id
    pub send_id: Option<String>, // 发送 ID （客户端构造uuid v4 无分隔符）
    pub sync_id: Option<String>, // 同步 ID （服务端生成，uuid v4 无分隔符）
    pub user_type: Option<i16>,  // 用户类型 （默认 2 普通用户）
    // -- 资料 --
    pub user_nickname: Option<String>, // 昵称
    pub signature: Option<String>,     // 签名
    pub avatar: Option<String>,        // 头像
    pub bg_img: Option<String>,        // 主页背景图
    pub email: Option<String>,         // 邮箱
    pub phone: Option<String>,         // 电话号码
    pub sns_url: Option<String>,       // 视频原始url
    pub birthday: Option<i64>,         // 生日
    pub sex: Option<i16>,              // 性别
    // -- 权限
    pub perm_id: i16, // 权限
    // -- 统计
    pub likes: Option<i64>,        // 收到的点赞数量
    pub fans: Option<i64>,         // 粉丝数量
    pub follows: Option<i64>,      // 关注数量
    pub level: Option<i16>,        // 用户等级
    pub author_level: Option<i16>, // 主播等级
    // -- 来源 --
    pub lat: Option<f64>,    // 纬度
    pub lng: Option<f64>,    // 经度
    pub login_ip: String,    // 当前登录IP
    pub register_ip: String, // 注册IP
    // -- 状态 --
    pub status: Option<i16>, // 状态
    // -- time --
    pub add_time: i32,                      // 创建时间（兼容PHP程序）
    pub sync_time: Option<i64>,             // 同步时间
    pub create_time: Option<DateTime<Utc>>, // 创建时间戳（UTC）
    pub update_time: Option<DateTime<Utc>>, // 修改时间 （UTC）
}
