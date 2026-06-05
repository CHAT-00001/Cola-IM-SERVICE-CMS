// cola_data/src/dynamic/command/dynamic.rs  -- DYNAMIC - Command - 动态发布命令
// 2026/5/22 20:25 by wx: cestbon10080
// * --------
// * --------

use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 动态 - 发布动态命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCommand {
    pub send_id: String,              // 发送 ID
    pub user_id: i64,                 // 用户 ID
    pub r#type: i16,                  // 类型
    pub title: String,                // 标题
    pub description: Option<String>,  // 简介
    pub original_url: Option<String>, // 原始 URL
    pub thumb: Option<String>,        // 封面图片（兼容旧版PHP）
    pub video_thumb: Option<String>,  // 封面图片（兼容旧版PHP）
    pub video_url: Option<String>,    // 封面图片（兼容旧版PHP）
    pub voice_url: Option<String>,    // 封面图片（兼容旧版PHP）
    pub length: Option<i64>,          // 封面图片（兼容旧版PHP）
    pub payload: Option<String>,      // 新版媒体负载
    pub address: Option<String>,      // 地址
    pub add_time: i64,                // 创建时间（兼容旧版PHP）
}

//////// END
