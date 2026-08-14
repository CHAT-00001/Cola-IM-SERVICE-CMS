use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

// # [ROUTER] - 用户 - 路由器
pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // # router dispatcher
        web::scope("/user")
            .route("", web::get().to(get_users))
            .route("/{id}", web::get().to(get_user))
            .route("", web::post().to(create_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            // 可以继续添加更多子路由，比如：
            .service(
                web::scope("/profile")
                    .route("", web::get().to(get_profile))
                    .route("", web::put().to(update_profile))
            )
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// 用户列表
pub async fn get_users() -> HttpResponse {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    HttpResponse::Ok().json(users)
}

// 获取单个用户
pub async fn get_user(user_id: web::Path<u64>) -> HttpResponse {
    let user = User {
        id: *user_id,
        name: format!("User {}", user_id),
        email: format!("router{}@example.com", user_id),
    };

    HttpResponse::Ok().json(user)
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// 创建用户
pub async fn create_user(user_req: web::Json<CreateUserRequest>) -> HttpResponse {
    let new_user = User {
        id: 100, // 模拟ID生成
        name: user_req.name.clone(),
        email: user_req.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}

// 更新用户
pub async fn update_user(user_id: web::Path<u64>, user_req: web::Json<CreateUserRequest>) -> HttpResponse {
    let updated_user = User {
        id: *user_id,
        name: user_req.name.clone(),
        email: user_req.email.clone(),
    };

    HttpResponse::Ok().json(updated_user)
}

// 删除用户
pub async fn delete_user(user_id: web::Path<u64>) -> HttpResponse {
    HttpResponse::NoContent().finish()
}

// 用户个人资料相关
#[derive(Debug, Serialize)]
pub struct Profile {
    pub user_id: u64,
    pub bio: String,
    pub avatar: String,
}

// 获取用户资料
pub async fn get_profile() -> HttpResponse {
    let profile = Profile {
        user_id: 1,
        bio: "This is a sample bio".to_string(),
        avatar: "https://example.com/avatar.jpg".to_string(),
    };

    HttpResponse::Ok().json(profile)
}

// 更新用户资料
pub async fn update_profile() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Profile updated"}))
}