// service/src/cola_user/cola_user/active.rs
// 服务层 - 可乐用户 - 用户 - 活跃
// 2026/06/05 03:10 Created.

////////

use anyhow::Result;
use cola_data::cola_user::command::new::UserCommand;
use cola_data::cola_user::info::user::UserInfo;
use repository::cola_user::pg::user::get::UserGetRepo;
use std::collections::HashMap;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `给case层获取用户信息的服务`
pub struct UserService;

// 构造实现
impl UserService {
    //

    /// # 1. [SERVICE] - 保存用户信息
    pub async fn save_user_info(cmd: UserCommand) -> Result<UserInfo, anyhow::Error> {
        // TODO: 底层落库逻辑
        Ok(UserInfo::default())
    }

    ////////

    /// # 2. [SERVICE] - 单个
    /// * `desc`: `单个查找用户信息`
    /// * `return`: `返回的entity转换成info并回填缓存`
    pub async fn get_user_info_by_id(
        user_id: i64, // 目标用户ID
    ) -> Result<UserInfo, anyhow::Error> {
        let option_entity = UserGetRepo::single_find_user_by_id(user_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 底层查询用户失败: {}", e))?;

        match option_entity {
            Some(entity) => Ok(UserInfo::from(entity)),
            // 🚀 单条兜底：Repo 返回 None，直接用构造函数吐出空的 UserInfo 扔给上层 VO
            None => Ok(UserInfo::default()),
        }
    }

    ////////

    /// # 3. [SERVICE] - 批量
    /// * `desc`: `批量查找用户信息`
    /// * `return`: `返回的entity转换成info并回填缓存`
    pub async fn get_user_info_by_ids(
        user_ids: &[i64],
    ) -> Result<HashMap<i64, UserInfo>, anyhow::Error> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }

        // 1. 物理层击中多少抓多少
        let entity_list = UserGetRepo::batch_find_users_by_ids(user_ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 批量获取用户数据失败: {}", e))?;

        let mut info_map = HashMap::with_capacity(user_ids.len());
        for entity in entity_list {
            info_map.insert(entity.id, UserInfo::from(entity));
        }

        // 2. 🚀 【全局数据防御】上层 VO 想要的所有 uid，只要缺席，立刻用 UserInfo::default() 充满
        for &uid in user_ids {
            if uid > 0 {
                info_map.entry(uid).or_insert_with(UserInfo::default);
            }
        }

        Ok(info_map)
    }

    ////////

    /// # 4. [SERVICE] - 修改用户信息
    pub async fn update_user_info_by_id(
        user_id: i64,
        cmd: UserCommand,
    ) -> Result<UserInfo, anyhow::Error> {
        // TODO: 底层更新逻辑
        Ok(UserInfo::default())
    }

    ////////
    // 👇 以下两组静态函数保持原样，提供给你的关注/朋友动态流进行关系筛选

    pub async fn find_following_ids(_current_uid: i64) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }

    pub async fn find_friend_ids(_current_uid: i64) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }
}

//////// END
