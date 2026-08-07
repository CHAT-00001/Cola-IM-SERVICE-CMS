// repo_adapter/src/user/black/get.rs
// 🔌 适配器 - USER - 黑名单 - 获取
// 2026/8/6 解耦: 获取黑名单IDs

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::black::get::BlackGetPort;
use repository::user::pg::black::get::UserBlackGetRepo;
use tracing::{error, info};

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `用户黑名单获取服务适配器`
pub struct BlackGetAdapter;

#[async_trait]
impl BlackGetPort for BlackGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 获取我的黑名单IDs列表
    async fn get_my_black_ids(
        &self,
        uid: i64,
        id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        // 目标ID转换
        let target_uid = id;

        // 1. Call The Repo ..
        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!("[🔌 ADAPTER] - ✅️ 获取我的黑名单IDs成功: uid = {}, count = {}", target_uid, ids.len());
                Ok(ids)
            }
            Err(e) => {
                error!("[🔌 ADAPTER] - ❌️ 获取我的黑名单IDs失败: uid = {}, err = {:?}", target_uid, e);
                Err(e)
            }
        }
    }

    ////////

    /// # [ADAPTER] - 获取她的黑名单IDs列表
    async fn get_he_black_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        let target_uid = id;

        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!("[🔌 ADAPTER] - ✅️ 获取她的黑名单IDs成功: uid = {}, count = {}", target_uid, ids.len());
                Ok(ids)
            }
            Err(e) => {
                error!("[🔌 ADAPTER] - ❌️ 获取她的黑名单IDs失败: uid = {}, err = {:?}", target_uid, e);
                Err(e)
            }
        }
    }

    ////////

    /// # [ADAPTER] - 获取黑名单IDs列表
    async fn get_black_me_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        let target_uid = id;

        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!("[🔌 ADAPTER] - ✅️ 获取拉黑我的黑名单IDs成功: uid = {}, count = {}", target_uid, ids.len());
                Ok(ids)
            }
            Err(e) => {
                error!("[🔌 ADAPTER] - ❌️ 获取拉黑我的黑名单IDs失败: uid = {}, err = {:?}", target_uid, e);
                Err(e)
            }
        }
    }

    ////////

    /// # [ADAPTER] - 获取黑名单IDs列表
    async fn get_black_he_ids(
        &self,
        uid: i64,
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        let target_uid = id;

        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!("[🔌 ADAPTER] - ✅️ 获取拉黑TA的黑名单IDs成功: uid = {}, count = {}", target_uid, ids.len());
                Ok(ids)
            }
            Err(e) => {
                error!("[🔌 ADAPTER] - ❌️ 获取拉黑TA的黑名单IDs失败: uid = {}, err = {:?}", target_uid, e);
                Err(e)
            }
        }
    }
}

//////// END