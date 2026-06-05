// cola_video/src/video/app/home.rs  -- VIDEO - 应用层 - 主页
// 2026-04-16 08:00

////////

use cola_data::app::data::AppData;
use cola_data::app::request::ApiUrlParamsQuery;
use crate::biz;
use crate::model::vo::video::VideoListResponse;

////////
pub struct CaseHome;

impl CaseHome {
    /// # 1. [CASE] - 最新
    pub async fn case_home_new(
        query: ApiUrlParamsQuery,
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_new_list(query,).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);

                AppData::err(5001, "获取最新视频失败", None)
            }
        }
    }

    ////////

    /// # 2. [CASE] - 热门
    pub async fn case_home_hotlist(
        query: ApiUrlParamsQuery,
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_hot_list(query,).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("Recommend Error: {:?}", e);

                AppData::err(5001, "获取热门视频失败", None)
            }
        }
    }

    ////////

    /// # 3. [CASE] - 推荐
    pub async fn case_recommend(
        query: ApiUrlParamsQuery,
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_recommend_list(query,).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Recommend Error: {:?}", e);
                AppData::err(5001, "获取推荐视频失败", None)
            }
        }
    }

    ////////

    /// # 4. [CASE] - 同城
    pub async fn case_home_city(
        query: ApiUrlParamsQuery,
        city_id: Option<i16>,
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_city_videos(query, city_id).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Nearby Error: {:?}", e);
                AppData::err(5001, "获取同城视频失败", None)
            }
        }
    }

    ////////

    /// # 5. [CASE] - 分类
    pub async fn case_home_category(
        query: ApiUrlParamsQuery,
        category_id: i16,
    ) -> AppData<VideoListResponse> {
        if category_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 category_id", None);
        }

        match biz::home::logic_get_category_list(query, category_id, user_port, video_port).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Category List Error: {:?}", e);
                AppData::err(5006, format!("获取分类视频失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 6. [CASE] - 精选
    pub async fn case_home_featured(
        query: ApiUrlParamsQuery,
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_best_list(query, ).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Featured Error: {:?}", e);
                AppData::err(5001, "获取精选视频失败", None)
            }
        }
    }

    ////////

    /// # 7. [CASE] - 搜索
    pub async fn case_home_search(
        query: ApiUrlParamsQuery,
        keyword: String,  // 关键词
    ) -> AppData<VideoListResponse> {
        match biz::home::logic_get_keyword_list(query, keyword,).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5006, format!("获取用户视频失败: {}", e), None),
        }
    }
}
//////// END
