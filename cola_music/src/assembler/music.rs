// cola_music/src/assembler/music.rs  -- MUSIC - 组装 - 音乐响应体
// 2026-07-08

//////

use std::collections::HashMap;
use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::music::info::music::MusicInfo;
use cola_data::music::vo::music_vo::{MusicListResponse, MusicSingleResponse, MusicVo};
use cola_data::user::info::user::UserInfo;
use repo::user::service::user::UserService;

//////

/// # [BUILD] - 构建单音乐响应函数
pub async fn build_music_single_response(
    music_info: MusicInfo,
    _current_uid: Option<i64>,
) -> Result<MusicSingleResponse> {
    let author_uid = music_info.user_id.unwrap_or(0);

    let author = if author_uid > 0 {
        UserService::get_user_info_by_id(author_uid)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 详情页获取用户信息失败: {}", e))?
    } else {
        UserInfo::default()
    };

    let music_vo = MusicVo::combine(music_info, author);

    Ok(MusicSingleResponse { info: music_vo })
}

//////

/// # [BUILD] - 构建多音乐列表响应体
pub async fn build_music_list_response(
    infos: Vec<MusicInfo>,
    _current_uid: Option<i64>,
    page: i64,
    qty: i64,
    _total: i64,
) -> Result<MusicListResponse> {

    let authors_map: HashMap<i64, UserInfo> = if infos.is_empty() {
        HashMap::new()
    } else {
        let author_ids: Vec<i64> = infos
            .iter()
            .filter_map(|v| v.user_id)
            .filter(|&id| id > 0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        UserService::get_user_info_by_ids(&author_ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 批量获取用户信息失败: {}", e))?
    };

    let list: Vec<MusicVo> = infos.into_iter().map(|music_info| {
        let author_uid = music_info.user_id.unwrap_or(0);
        let author = authors_map.get(&author_uid).cloned().unwrap_or_default();
        MusicVo::combine(music_info, author)
    }).collect();

    let has_more = list.len() >= (qty as usize);

    let page_info = PageInfo {
        page,
        qty,
        has_more,
    };

    Ok(MusicListResponse { list, page_info })
}