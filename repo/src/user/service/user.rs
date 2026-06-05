// repo/src/user/service/user.rs  -- 仓储 - USER - service - user
// 2026/06/05 03:10 by wx: cestbon10080

////////

use std::collections::HashMap;
use anyhow::Result;
use cola_data::user::command::user::UserCommand;
use cola_data::user::info::user::UserInfo;
use crate::user::pg::user::UserRepo;

////////

pub struct UserService;

impl UserService {

    /// # 1. [SERVICE] - 保存用户信息
    pub async fn save_user_info(
        cmd: UserCommand,
    ) -> Result<UserInfo, anyhow::Error> {
        // TODO: 底层落库逻辑
        Ok(UserInfo::default())
    }

    ////////

    /// # 2. [SERVICE] - 查找一个用户信息
    pub async fn find_user_info_by_id(
        user_id: i64,
    ) -> Result<UserInfo, anyhow::Error> {
        let option_entity = UserRepo::find_user_by_id(user_id)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 底层查询用户失败: {}", e))?;

        match option_entity {
            Some(entity) => {
                Ok(UserInfo::from(entity))
            }
            // 🚀 单条兜底：Repo 返回 None，直接用构造函数吐出空的 UserInfo 扔给上层 VO
            None => Ok(UserInfo::default()),
        }
    }

    ////////

    /// # 3. [SERVICE] - 查找一组用户信息
    /// * 核心：专门提供给各大业务层的装配器（VideoVo, CommentVo 等）调用
    /// * 机制：不管数据库有没有，上层要多少 UID，这里就必然喂饱多少个 UserInfo，空的主动用构造函数垫后
    pub async fn find_user_info_by_uids(
        user_ids: &[i64],
    ) -> Result<HashMap<i64, UserInfo>, anyhow::Error> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }

        // 1. 物理层击中多少抓多少
        let entity_list = UserRepo::find_many_users_by_ids(user_ids)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 批量获取用户数据失败: {}", e))?;

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

    pub async fn find_following_ids(
        _current_uid: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }

    pub async fn find_friend_ids(
        _current_uid: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        Ok(vec![])
    }
}

//////// END