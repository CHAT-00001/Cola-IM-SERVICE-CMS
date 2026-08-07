// repo_adapter/src/user/view/add.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 发布服务
// 2026/8/6 04:18 Created.

////////

// /del.rs
//
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::command::new::UserCommand;
use cola_data::user::port::view::add::ViewAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `用户浏览发布服务`
pub struct ViewAddService;

// 构造实现
#[async_trait]
impl ViewAddPort for ViewAddService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn save_view(&self, cmd: UserCommand) -> Result<()> {
        todo!()
    }

    async fn del_view(&self, cmd: UserCommand) -> Result<()> {
        todo!()
    }

    async fn del_one_user(&self, user_id: i64) -> Result<()> {
        todo!()
    }

    async fn del_many_user(&self, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END
