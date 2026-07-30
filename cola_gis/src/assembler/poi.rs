// cola_gis/src/assembler/poi.rs  -- GIS - 组装 - 兴趣点响应体
// 2026-07-07 07:06

////////

use crate::model::vo::poi::{PoiListResponse, PoiSingleResponse, PoiVo};
use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::gis::info::poi::PoiInfo;
use cola_data::music::info::music::MusicInfo;
use cola_data::user::info::user::UserInfo;
use repository::user::service::user::UserService;
use std::collections::HashMap;

////////

/// # [BUILD] - 构建单兴趣点响应函数
/// * 机制：纯静态服务层调用，自带未查到博主时的 UserInfo::default 强力兜底
pub async fn build_poi_single_response(
    poi_info: PoiInfo,         // 兴趣点源数据
    _current_uid: Option<i64>, // 用户 ID
) -> Result<PoiSingleResponse> {
    // 1. 获取该兴趣点的作者 ID
    let author_uid = poi_info.uid;

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

    // 4. 🚀 大聚合：调用 combine 生成前端需要的扁平化 PoiVo
    let poi_vo = PoiVo::combine(poi_info, author, music_info);

    // 5. 包装进单兴趣点响应体返回
    Ok(PoiSingleResponse { info: poi_vo })
}

////////

/// # [BUILD] - 构建多兴趣点列表响应体
/// * 机制：调用服务层 find_user_info_by_uids 批量补全，上层零判空、零等待，高性能组装
pub async fn build_poi_list_response(
    infos: Vec<PoiInfo>, // 🌟 1. 类型对齐：完美接收 Service 层脱敏后的元数据 Info
    _current_uid: Option<i64>,
    // 外部传入的分页基础原始数据
    page: i64,   // 当前页码
    qty: i64,    // 每页数量
    _total: i64, // 🌟 2. 数量对齐：接收 Case 层传进来的第 6 个参数 total
) -> Result<PoiListResponse> {
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

    // 2. 🌟 迭代组装完美的 PoiVo 列表
    let list: Vec<PoiVo> = infos
        .into_iter()
        .map(|poi_info| {
            let author_uid = poi_info.uid;

            // 💡 因为 UserService 保证了请求的 id 只要大于 0 必然有值在 map 里，
            // 这里直接 cloned() 拿走即可，无需多余转换。
            let author = authors_map.get(&author_uid).cloned().unwrap_or_default();
            let music_info = MusicInfo::default();

            // 🌟 核心修正：干掉了旧的 from_entity，直接将纯净的 poi_info 拿来融合成大视图对象 Vo
            PoiVo::combine(poi_info, author, music_info)
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

    Ok(PoiListResponse { list, page_info })
}

//////// END
