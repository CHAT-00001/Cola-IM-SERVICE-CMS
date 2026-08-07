// repo_adapter/src/cola_im/contact_request.rs  -- 适配器 - IM - 联系人请求
// 2026-07-07

use async_trait::async_trait;
use cola_data::cola_im::port::contact_request::ContactRequestRepo;
use cola_data::cola_im::command::contact_request::ContactRequestCommand;
use cola_data::cola_im::info::contact_request::ContactRequestInfo;

pub struct ContactRequestPortAdapter;

#[async_trait]
impl ContactRequestRepo for ContactRequestPortAdapter {
    async fn send_request(&self, _uid: i64, _cmd: ContactRequestCommand) -> anyhow::Result<()> {
        Ok(())
    }

    async fn accept_request(&self, _uid: i64, _request_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn refuse_request(&self, _uid: i64, _request_id: i64, _replay: Option<String>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn list_requests(&self, _uid: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<ContactRequestInfo>> {
        Ok(vec![])
    }
}