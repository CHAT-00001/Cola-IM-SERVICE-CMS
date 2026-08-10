// repo_adapter/src/video/mod.rs
// 🔌 插头 - 可乐视频 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_video::ColaVideoPort;
use port::cola_video::buy::VideoBuyPort;
use port::cola_video::collect::VideoCollectPort;
use port::cola_video::comment::VideoCommentPort;
use port::cola_video::danmaku::VideoDanmakuPort;
use port::cola_video::dislike::VideoDislikePort;
use port::cola_video::hotlist::VideoHotlistPort;
use port::cola_video::like::VideoLikePort;
use port::cola_video::recommend::VideoRecommendPort;
use port::cola_video::share::VideoSharePort;
use port::cola_video::video::VideoPort;
use port::cola_video::view::VideoViewPort;
use std::sync::Arc;
use crate::stub;

////////

pub mod buy; // 购买
pub mod collect; // 收藏
pub mod comment; // 评论
pub mod danmaku; // 弹幕
pub mod dislike; // 不喜欢
pub mod hotlist; // 上热门
pub mod like; // 点赞
pub mod recommend; // 推荐
pub mod share; // 分享
pub mod video; // 视频(主内容)
pub mod view; // 浏览

////////

/// # [BUILD] - 构建 VIDEO Port
/// * `desc`: 构建视频 Port 聚合体，包含所有视频相关功能
pub fn build_video_port() -> ColaVideoPort {
    ColaVideoPort {
        add: Arc::new(stub::GeneralStubAdapter),
        buy: VideoBuyPort {
            add: Arc::new(buy::add::BuyAddPortAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        collect: VideoCollectPort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(collect::manage::CollectManageAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        comment: VideoCommentPort {
            add: Arc::new(comment::add::CommentAddPortAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            dislike: Arc::new(comment::dislike::VideoCommentDislikeAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            like: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        danmaku: VideoDanmakuPort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            like: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
            step: Arc::new(stub::GeneralStubAdapter),
        },
        dislike: VideoDislikePort {
            add: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        hotlist: VideoHotlistPort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        like: VideoLikePort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        recommend: VideoRecommendPort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        share: VideoSharePort {
            add: Arc::new(stub::GeneralStubAdapter),
            check: Arc::new(stub::GeneralStubAdapter),
            del: Arc::new(stub::GeneralStubAdapter),
            get: Arc::new(stub::GeneralStubAdapter),
            list: Arc::new(stub::GeneralStubAdapter),
            manage: Arc::new(stub::GeneralStubAdapter),
            stat: Arc::new(stub::GeneralStubAdapter),
        },
        video: VideoPort {
            add: Arc::new(video::add::VideoAddAdapter),
            check: Arc::new(video::check::VideoCheckAdapter),
            del: Arc::new(video::del::VideoDelAdapter),
            get: Arc::new(video::get::VideoGetAdapter),
            list: Arc::new(video::list::VideoListAdapter),
            manage: Arc::new(video::manage::VideoManageAdapter),
            stat: Arc::new(video::stat::VideoStatAdapter),
        },
        view: VideoViewPort {
            active: Arc::new(view::active::AliveService),
            add: Arc::new(view::add::AddService),
            del: Arc::new(view::del::ViewDelService),
            get: Arc::new(view::get::ViewGetService),
            list: Arc::new(view::list::ViewListService),
            manage: Arc::new(view::manage::ViewManageService),
            stat: Arc::new(view::stat::ViewStatService),
        },
        report: (),
    }
}

//////// END
