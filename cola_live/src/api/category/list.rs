// cola_live/src/api/category/list.rs  -- LIVE - api - 分类 - 列表
// 2026/5/20 02:00

////////

use cola_data::app::data::AppData;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_video::info::video::VideoListResponse;

////////

pub struct LiveCategoryListApi;

impl LiveCategoryListApi {
    //

    ////////

    /// # 1. [API HANDLER] - 最新
    pub async fn api_get_new_list(
        query: ApiUrlParamsQuery,
        category_id: i16,
    ) -> AppData<VideoListResponse> {
        // 1. 检查分类ID是否有效
        if category_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 category_id", None);
        }

        AppData::err(4000, "直播分类视频列表尚未接入直播房间查询", None)
    }

    ////////

    /// 2. [API HANDLER] - 搜索
    pub async fn api_search_key_list(query: ApiUrlParamsQuery) -> AppData<VideoListResponse> {
        AppData::err(4000, "直播分类搜索尚未接入直播房间查询", None)
    }
}
//////// END
