// repo_adapter/src/video/danmaku/mod.rs -- 🔌 适配器 - VIDEO - 弹幕 - mod
// 2026/8/9 22:16 Created.

////////

use port::cola_video::danmaku::VideoDanmakuPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 活跃
pub mod check; // 检查
pub mod del; // 删除
pub mod dislike; // 弹幕不喜欢
pub mod get; // 获取
pub mod like; // 弹幕点赞
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频弹幕端口构造器
pub fn build_video_danmaku_port() -> VideoDanmakuPort {
    VideoDanmakuPort {
        add: Arc::new(add::VideoDanmakuaddAdapter),
        check: Arc::new(check::VideDanmakuCheckAdapter),
        del: Arc::new(del::VideoDanmakuDelAdapter),
        dislike: Arc::new(dislike::VideoDanmakuDislikeAdapter),
        get: Arc::new(get::VideoDanmakuGetAdapter),
        like: Arc::new(like::VideoDanmakuLikeAdapter),
        list: Arc::new(list::VideoDanmakuListAdapter),
        manage: Arc::new(manage::VideoDanmakuManageAdapter),
        stat: Arc::new(stat::VideoDanmakuStatAdapter),
    }
}

//////// END
