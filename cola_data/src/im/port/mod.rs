// cola_data/src/im/port/mod.rs  -- 数据中心 - IM - port
// 2026-07-07

//////

use crate::im::port::contact::ContactRepo;
use crate::im::port::contact_request::ContactRequestRepo;
use crate::im::port::card::CardRepo;
use crate::im::port::message::MessageRepo;
use crate::im::port::chat::ChatRepo;
use std::sync::Arc;

//////

pub mod contact;
pub mod contact_request;
pub mod card;
pub mod message;
pub mod chat;

//////

/// # [SERVICE PORT] - IM ServicePort
#[derive(Clone)]
pub struct ColaImPort {
    pub contact: Arc<dyn ContactRepo + Send + Sync + 'static>,
    pub contact_request: Arc<dyn ContactRequestRepo + Send + Sync + 'static>,
    pub card: Arc<dyn CardRepo + Send + Sync + 'static>,
    pub message: Arc<dyn MessageRepo + Send + Sync + 'static>,
    pub chat: Arc<dyn ChatRepo + Send + Sync + 'static>,
}