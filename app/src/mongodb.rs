use mongodb::{options::ClientOptions, Client};
use tracing::{info};
use crate::config::Mongodb;

/// 初始化 MongoDB 连接
pub async fn mongodb_init(config: &Mongodb) -> Result<Client, mongodb::error::Error> {
    // 构建 MongoDB 连接字符串
    let mut uri = format!("mongodb://{}", config.host);
    if config.port != 0 {
        uri = format!("{}:{}", uri, config.port);
    }

    if let Some(user) = &config.user {
        if let Some(password) = &config.password {
            uri = format!("mongodb://{}:{}@{}:{}", user, password, config.host, config.port);
        }
    }

    // 解析配置
    let mut client_options = ClientOptions::parse(&uri).await?;
    client_options.app_name = Some("ShortVideoService".to_string());

    // 创建 MongoDB 客户端
    let client = Client::with_options(client_options)?;

    info!("MongoDB connected to {}:{}", config.host, config.port);

    Ok(client)
}
