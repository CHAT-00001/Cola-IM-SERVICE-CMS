// cola_data/src/user/port/active  -- 数据中心 - USER - 端口层 ADD
// 2026/6/10 07:31

////////

use crate::cola_user::command::user::add::UserCommand;
use crate::cola_user::command::user::update::UpdateUserCommand;
use crate::cola_user::info::user::UserInfo;

////////

#[async_trait::async_trait]
pub trait AddPort : Send + Sync + 'static {


    ////////

    /// # 1. [SERVICE] - 保存
    async fn save_user(
        &self,
        cmd: UserCommand,
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 2. [SERVICE] - 更新
    async fn update_user(
        &self,
        cmd: UpdateUserCommand,
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 3. [SERVICE] - 删除一个
    async fn del_one_user(
        &self,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [SERVICE] - 删除多个
    async fn del_many_user(
        &self,
        user_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}

//////// END