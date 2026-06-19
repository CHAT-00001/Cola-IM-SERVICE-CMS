// cola_user/src/case/home.rs  -- 用户中心 - 用例层 - home
// 2026/6/18 09:06

//////

use crate::model::vo::user::UserVo;
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;

//////

/// # [HOME CASE] - 用户主页 用例
pub struct UserHomeCase;

impl UserHomeCase {

    ////////

    /// # 1. [CASE] - 获取最新注册的用户列表
    /// * `uid` 当前操作者
    /// * `url` 请求参数（含分页、经纬度）
    /// * `ctx` 应用上下文（通过 ctx.user.home 调用端口）
    pub async fn case_get_newest_users(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<Vec<UserVo>> {
        // 1. 从端口获取最新注册用户列表
        let user_infos = ctx.user.home.get_newest_users(url.limit, url.offset).await?;

        // 2. 基础转换 UserInfo → UserVo
        let mut vo_list = UserVo::from_info_list(&user_infos);

        // 3. 如果带坐标(经纬度)，调用 nearby 端口补充距离
        if let (Some(lat), Some(lng)) = (url.lat, url.lng) {
            let nearby = ctx.user.home.get_nearby_users(lat, lng, url.limit, url.offset).await?;
            // 转成 Map: user_id -> distance
            let distance_map: std::collections::HashMap<i64, Option<f64>> = nearby
                .into_iter()
                .map(|(info, dist)| (info.id, dist))
                .collect();

            for vo in vo_list.iter_mut() {
                if let Some(&dist) = distance_map.get(&vo.id) {
                    vo.geo_distance = dist;
                }
            }
        }

        // 4. 检查是否关注（使用 following port）
        let following_ids = ctx.user.following.get_following_ids(uid).await?;
        let following_set: std::collections::HashSet<i64> = following_ids.into_iter().collect();

        for vo in vo_list.iter_mut() {
            vo.is_following = following_set.contains(&vo.id);
        }

        Ok(vo_list)
    }
}
