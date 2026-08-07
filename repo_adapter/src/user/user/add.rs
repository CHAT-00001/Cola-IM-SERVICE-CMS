// /add.rs
//
// 2026/8/6 04:18 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::command::new::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::port::user::add::UserAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `用户发布服务`
pub struct UserAddAdapter;

// 构造实现
#[async_trait]
impl UserAddPort for UserAddAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 保存
    /// * `desc`: `保存新用户记录`
    async fn save_user(
        &self,
        cmd: UserCommand, // 命令
    ) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn update_user(&self, cmd: UpdateUserCommand) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn update_avatar(&self, uid: i64, media_id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn update_bg(&self, uid: i64, media_id: i64) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
