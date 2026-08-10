// service/src/user/user/list.rs
// 👤 服务 - 🗣 可乐用户 - 用户 - 前台列表服务
// 2026/8/7 17:11 Created.

////////

use anyhow::Result;
use cola_data::cola_user::info::user::UserInfo;
use repository::cola_user::pg::user::get::UserGetRepo;
use repository::cola_user::pg::user::list::UserListRepo;
use std::collections::HashMap;
use cola_data::cola_user::command::user::add::UserCommand;
////////

/// # [LIST SERVICE] - 前台列表
/// * `desc`: `给case层获取用户信息列表的服务`
pub struct UserListService;

// 构造实现
impl UserListService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个查找用户信息`
    /// * `return`: `返回的entity转换成info`
    pub async fn get_user_info_by_id(
        user_id: i64, // 目标用户ID
    ) -> Result<UserInfo, anyhow::Error> {
        let option_entity = UserGetRepo::single_find_user_by_id(user_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 底层查询用户失败: {}", e))?;

        match option_entity {
            Some(entity) => Ok(UserInfo::from(entity)),
            None => Ok(UserInfo::default()),
        }
    }

    ////////

    /// # 3. [SERVICE] - 批量
    /// * `desc`: `批量查找用户信息`
    pub async fn get_user_info_by_ids(
        user_ids: &[i64],
    ) -> Result<HashMap<i64, UserInfo>, anyhow::Error> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let entity_list = UserGetRepo::batch_find_users_by_ids(user_ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 批量获取用户数据失败: {}", e))?;

        let mut info_map = HashMap::with_capacity(user_ids.len());
        for entity in entity_list {
            info_map.insert(entity.id, UserInfo::from(entity));
        }

        for &uid in user_ids {
            if uid > 0 {
                info_map.entry(uid).or_insert_with(UserInfo::default);
            }
        }

        Ok(info_map)
    }

    ////////

    /// # 4. [SERVICE] - 前台最新用户列表
    /// * `desc`: `对接 UserListRepo::find_new_list 并转换为 UserInfo 列表`
    pub async fn get_new_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取最新用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 5. [SERVICE] - 附近的用户列表
    /// * `desc`: `对接 UserListRepo::find_nearby_list 并转换为 UserInfo 列表`
    pub async fn get_nearby_user_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_nearby_list(lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取附近用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 6. [SERVICE] - 分类频道下的用户列表
    /// * `desc`: `对接 UserListRepo::find_category_list 并转换为 UserInfo 列表`
    pub async fn get_category_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_category_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取分类用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 7. [SERVICE] - 同城
    /// * `desc`: `对接 UserListRepo::find_category_list 并转换为 UserInfo 列表`
    pub async fn get_city_user_list(
        city_id: i64, // 城市 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_category_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取分类用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    /// # 8. [SERVICE] - 角色分类下的用户列表
    /// * `desc`: `对接 UserListRepo::find_category_list 并转换为 UserInfo 列表`
    pub async fn get_role_user_list(
        role_id: i64, // 角色 ID
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_category_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取分类用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 9. [SERVICE] - 精选用户列表
    /// * `desc`: `对接 UserListRepo::find_featured_list 并转换为 UserInfo 列表`
    pub async fn get_featured_user_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::find_featured_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 获取精选用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 10. [SERVICE] - 搜索关键词用户列表
    /// * `desc`: `对接 UserListRepo::search_keyword 并转换为 UserInfo 列表`
    pub async fn search_user_keyword(
        keyword: &str,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        let list = UserListRepo::search_keyword(keyword, lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 搜索用户列表失败: {}", e))?;

        Ok(list.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 11. [SERVICE] - 修改用户信息
    pub async fn update_user_info_by_id(
        user_id: i64,
        cmd: UserCommand,
    ) -> Result<UserInfo, anyhow::Error> {
        let _ = (user_id, cmd);
        // TODO: 底层更新逻辑
        Ok(UserInfo::default())
    }

    ////////
    // 👇 关注/朋友动态流关系筛选静态函数

    pub async fn find_following_ids(_current_uid: i64) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }

    pub async fn find_friend_ids(_current_uid: i64) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }
}

//////// END
