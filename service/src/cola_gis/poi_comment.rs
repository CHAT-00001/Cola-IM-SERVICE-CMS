// service/src/cola_gis/comment/poi_comment.rs
// 服务 - 可乐GIS - 评论 - 评论服务
// 2026/7/6 12:04

////////

use cola_data::cola_gis::command::comment::PoiCommentCommand;
use cola_data::cola_gis::info::comment::PoiCommentInfo;
use repository::cola_gis::pg::count::CountRepo;
use repository::cola_gis::pg::poi_comment::PoiCommentRepo;

////////

/// # [COMMENT SERVICE] - 评论服务
/// * `desc`: `📍 兴趣点 评论`
pub struct PoiCommentService;

// 构造实现
impl PoiCommentService {
    //

    /////////

    /// # [ADAPTER] - ✅️ 👤 根据兴趣点ID保存评论记录
    pub async fn save_comment_and_update_count(
        uid: i64,               // 用户 ID
        poi_id: i64,            // POI ID
        cmd: PoiCommentCommand, // CMD
        visibility: i16,        // 可见范围
    ) -> Result<PoiCommentInfo, anyhow::Error> {
        let entity = PoiCommentRepo::save_comment_by_poi_id(uid, poi_id, cmd, visibility).await?;
        let async_poi_id = poi_id;
        tokio::spawn(async move {
            if let Err(e) = CountRepo::pg_update_gis_comments(async_poi_id, 1).await {
                tracing::error!(
                    "[🔌 ADAPTER]: 👤 用户保存兴趣点评论记录失败: poi_id={}, err={:?}",
                    async_poi_id,
                    e
                );
            }
        });
        Ok(PoiCommentInfo::from_entity(entity))
    }

    /////////

    /// # [ADAPTER] - ✅️ 👤 保存兴趣点评论记录
    pub async fn delete_comment_and_update_count(
        uid: i64,        // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> Result<bool, anyhow::Error> {
        let entity = PoiCommentRepo::user_del_comment_by_id(uid, comment_id).await?;
        let poi_id = entity.poi_id; // reuse video_id field
        CountRepo::pg_update_gis_comments(poi_id, -1).await?;
        Ok(true)
    }

    /////////

    /// # [ADAPTER] - ✅️ 👤 保存兴趣点评论记录
    pub async fn find_comments_by_poi_id(
        poi_id: i64, // POI ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<PoiCommentInfo>, anyhow::Error> {
        let entities = PoiCommentRepo::find_new_comments_by_poi_id(poi_id, limit, offset).await?;
        Ok(entities
            .into_iter()
            .map(PoiCommentInfo::from_entity)
            .collect())
    }
}

//////// END
