// src/middleware/auth_check.rs - jwt 验证
// 2026-01-16 13:45:00

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;
use serde::{Deserialize, Serialize};

/// JWT 载荷结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,      // 用户 ID (uid)
    pub exp: u64,      // 过期时间 (秒级时间戳)
}

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            // 1. 获取 Authorization Header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];

                        // 2. 校验 JWT
                        // 建议：实际开发中密钥应该从环境配置（config）中读取
                        let secret = "your_super_secret_key_123";
                        let decoding_key = DecodingKey::from_secret(secret.as_ref());
                        let validation = Validation::new(Algorithm::HS256);

                        if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                            // 3. 【核心】验证成功，将 uid 注入 extensions
                            // 后续 ApiRequest 就能通过 req.extensions().get::<i64>() 拿到它
                            req.extensions_mut().insert(token_data.claims.sub);
                        } else {
                            // 这里可以选择直接拦截返回 401，也可以放行让业务层判断是否为游客
                            // tracing::warn!("JWT 验证失败: {}", token);
                        }
                    }
                }
            }

            // 继续执行后续逻辑（即使没 Token 也放行，因为 ApiRequest 会处理 Option<uid>）
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}