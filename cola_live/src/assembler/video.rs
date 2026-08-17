// cola_live/src/assembler/video.rs
// core - LIVE - 组装器 -  视频VO响应体
// 2026/06/11 10:20

////////

use crate::model::vo::video::{VideoListResponse, VideoSingleResponse, VideoVo};
use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::cola_music::info::music::MusicInfo;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_video::info::video::VideoInfo;
use service::cola_user::user::active::UserService;
use std::collections::HashMap;

////////

fn resolve_cdn_url(path: String, cdn_domain: &str) -> String {
    if path.is_empty() || path.starts_with("http://") || path.starts_with("https://") || path.starts_with("//") {
        return path;
    }
    if path.starts_with('/') {
        format!("{}{}", cdn_domain.trim_end_matches('/'), path)
    } else {
        format!("{}/{}", cdn_domain.trim_end_matches('/'), path)
    }
}

fn resolve_cdn_url_opt(path: Option<String>, cdn_domain: &str) -> Option<String> {
    path.map(|value| resolve_cdn_url(value, cdn_domain))
}

////////

/// # [BUILD] - 构建单视频响应函数
/// * 机制：纯静态服务层调用，自带未查到博主时的 UserInfo::default 强力兜底
pub async fn build_video_single_response(
    video_info: VideoInfo,     // 视频源数据
    _current_uid: Option<i64>, // 用户 ID
) -> Result<VideoSingleResponse> {
    let cdn_domain = std::env::var("CDN_DOMAIN")
        .unwrap_or_else(|_| "https://cdn.shortvideo.com".to_string());
    build_video_single_response_with_cdn(video_info, _current_uid, &cdn_domain).await
}

////////

/// # [BUILD] - 使用业务层解析出的 CDN 域名构建单视频响应
/// * `desc`: `CASE 通过 Port 查询 CDN 后，将域名传入组装器`
pub async fn build_video_single_response_with_cdn(
    video_info: VideoInfo, // 视频源数据
    _current_uid: Option<i64>, // 用户 ID
    cdn_domain: &str, // 已解析的 CDN 域名
) -> Result<VideoSingleResponse> {
    // 1. 获取该视频的作者 ID
    let author_uid = video_info.uid;

    // 2. 🚀 直接静态调用服务层的单条查询（内部已做好 None 时的 default 兜底）
    let author = if author_uid > 0 {
        UserService::get_user_info_by_id(author_uid)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 详情页获取用户信息失败: {}", e))?
    } else {
        UserInfo::default()
    };

    // 3. 原声占位（统一改为你的 MusicInfo 类型）
    let music_info = MusicInfo::default();

    // 4. 🚀 CDN 域名组装
    let mut video_info = video_info;
    video_info.href = resolve_cdn_url(video_info.href, cdn_domain);
    video_info.thumb = resolve_cdn_url(video_info.thumb, cdn_domain);
    video_info.thumbnail = resolve_cdn_url_opt(video_info.thumbnail, cdn_domain);
    video_info.original_url = resolve_cdn_url_opt(video_info.original_url, cdn_domain);

    // 5. 🚀 大聚合：调用 combine 生成前端需要的扁平化 VideoVo
    let video_vo = VideoVo::combine(video_info, author, music_info);

    // 5. 包装进单视频响应体返回
    Ok(VideoSingleResponse { info: video_vo })
}

////////

/// # [BUILD] - 构建多视频列表响应体
/// * 机制：调用服务层 find_user_info_by_uids 批量补全，上层零判空、零等待，高性能组装
pub async fn build_video_list_response(
    infos: Vec<VideoInfo>, // 🌟 1. 类型对齐：完美接收 Service 层脱敏后的元数据 Info
    _current_uid: Option<i64>,
    // 外部传入的分页基础原始数据
    page: i64,   // 当前页码
    qty: i64,    // 每页数量
    _total: i64, // 🌟 2. 数量对齐：接收 Case 层传进来的第 6 个参数 total
) -> Result<VideoListResponse> {
    let cdn_domain = std::env::var("CDN_DOMAIN")
        .unwrap_or_else(|_| "https://cdn.shortvideo.com".to_string());
    build_video_list_response_with_cdn(infos, _current_uid, page, qty, _total, &cdn_domain).await
}

////////

/// # [BUILD] - 使用业务层解析出的 CDN 域名构建视频列表
/// * `desc`: `CASE 通过 Port 查询 CDN 后，将域名传入组装器`
pub async fn build_video_list_response_with_cdn(
    infos: Vec<VideoInfo>, // 视频源数据
    _current_uid: Option<i64>, // 用户 ID
    page: i64, // 当前页码
    qty: i64, // 每页数量
    _total: i64, // 总数量
    cdn_domain: &str, // 已解析的 CDN 域名
) -> Result<VideoListResponse> {
    // 1. 批量获取作者用户信息 (全静态服务化)
    let authors_map: HashMap<i64, UserInfo> = if infos.is_empty() {
        HashMap::new()
    } else {
        let author_ids: Vec<i64> = infos
            .iter()
            .map(|v| v.uid)
            .filter(|&id| id > 0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // 🚀 静态批量获取：UserService 会严格按照 author_ids 的数量全部喂饱
        UserService::get_user_info_by_ids(&author_ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 批量获取用户信息失败: {}", e))?
    };

    // 2. 🌟 迭代组装完美的 VideoVo 列表
    let list: Vec<VideoVo> = infos
        .into_iter()
        .map(|video_info| {
            let author_uid = video_info.uid;

            // 💡 因为 UserService 保证了请求的 id 只要大于 0 必然有值在 map 里，
            // 这里直接 cloned() 拿走即可，无需多余转换。
            let author = authors_map.get(&author_uid).cloned().unwrap_or_default();
            let music_info = MusicInfo::default();

            let mut video_info = video_info;
            video_info.href = resolve_cdn_url(video_info.href, cdn_domain);
            video_info.thumb = resolve_cdn_url(video_info.thumb, cdn_domain);
            video_info.thumbnail = resolve_cdn_url_opt(video_info.thumbnail, cdn_domain);
            video_info.original_url = resolve_cdn_url_opt(video_info.original_url, cdn_domain);

            // 🌟 核心修正：使用已解析 CDN 域名后的数据组装 VO
            VideoVo::combine(video_info, author, music_info)
        })
        .collect();

    // 3. 动态计算是否还有下一页 (根据当前页列表长度与每页申请数量对比)
    let has_more = list.len() >= (qty as usize);

    // 4. 完美匹配你的 PageInfo 结构字面量
    let page_info = PageInfo {
        page,
        qty,
        has_more,
    };

    Ok(VideoListResponse { list, page_info })
}

//////// END
