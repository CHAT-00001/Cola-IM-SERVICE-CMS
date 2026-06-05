// cola_video/src/assembler/video.rs  -- VIDEO - 组装 -  视频响应体
// 2026/06/05 03:40 by wx: cestbon10080

////////

use std::collections::HashMap;
use anyhow::Result;
//
use cola_data::app::page::PageInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::video::entity::video::VideoEntity;
use cola_music::model::info::music::MusicInfo;
use repo::user::service::user::UserService;
//
use crate::model::info::video::VideoInfo;
use crate::model::vo::video::{VideoListResponse, VideoSingleResponse, VideoVo};

////////

/// # [BUILD] - 构建单视频响应函数
/// * 机制：纯静态服务层调用，自带未查到博主时的 UserInfo::default 强力兜底
pub async fn build_video_single_response(
    video_info: VideoInfo,     // 视频源数据
    _current_uid: Option<i64>, // 用户 ID
) -> Result<VideoSingleResponse> {
    // 1. 获取该视频的作者 ID
    let author_uid = video_info.user_id;

    // 2. 🚀 直接静态调用服务层的单条查询（内部已做好 None 时的 default 兜底）
    let author = if author_uid > 0 {
        UserService::find_user_info_by_id(author_uid)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 详情页获取用户信息失败: {}", e))?
    } else {
        UserInfo::default()
    };

    // 3. 原声占位（统一改为你的 MusicInfo 类型）
    let music_info = MusicInfo::default();

    // 5. 🚀 大聚合：调用 combine 生成前端需要的扁平化 VideoVo
    let video_vo = VideoVo::combine(video_info, author, music_info);

    // 6. 包装进单视频响应体返回
    Ok(VideoSingleResponse { info: video_vo })
}

////////

/// # [BUILD] - 构建多视频列表响应体
/// * 机制：调用服务层 find_user_info_by_uids 批量补全，上层零判空、零等待，高性能组装
pub async fn build_video_list_response(
    entities: Vec<VideoEntity>,
    _current_uid: Option<i64>,
    // 外部传入的分页基础原始数据
    page: i64,   // 当前页码
    qty: i64,    // 每页数量
    _total: i64, // 总记录数
) -> Result<VideoListResponse> {

    // 1. 批量获取作者用户信息 (全静态服务化)
    let authors_map: HashMap<i64, UserInfo> = if entities.is_empty() {
        HashMap::new()
    } else {
        let author_ids: Vec<i64> = entities
            .iter()
            .map(|v| v.user_id)
            .filter(|&id| id > 0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // 🚀 静态批量获取：不管数据库少没少人，UserService 会严格按照 author_ids 的数量全部喂饱
        UserService::find_user_info_by_uids(&author_ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 批量获取用户信息失败: {}", e))?
    };

    // 2. 🌟 迭代组装完美的 VideoVo 列表
    let list: Vec<VideoVo> = entities.into_iter().map(|entity| {
        let author_uid = entity.user_id;

        // 💡 极其舒适：因为 UserService 保证了请求的 id 只要大于 0 必然有值在 map 里，
        // 这里直接 cloned() 拿走即可，哪怕账号不存在，里面拿到的也是带“用户不存在”占位符的安全 Info！
        let author = authors_map.get(&author_uid).cloned().unwrap_or_default();
        let music_info = MusicInfo::default();

        // 转换为 VideoInfo
        let video_info = VideoInfo::from_entity(entity);

        // 融合成大视图对象 Vo
        VideoVo::combine(video_info, author, music_info)
    }).collect();

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