use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row, Type};
use std::collections::{HashMap, HashSet};


/// # 用户信息
/// * 给别的UCG模型嵌套的用户基础信息.
/// * age 根据生日自动计算.
/// * 强社交关系还需要计算与操作者的关系例如：是否关注.
#[derive(Serialize, Deserialize, Clone, FromRow, Type, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub nickname: String,
    pub avatar: Option<String>,
    pub bg_img: Option<String>,
    pub email: Option<String>,
    pub age: Option<String>,
}


/// # 获取用户基础信息
/// 根据 user_id 查询用户信息，如果不存在返回默认信息
pub async fn get_user_info<UserRow>(
    pool: &PgPool,
    user_id: Option<i64>
) -> Result<UserInfo, sqlx::Error> {

  //  let pool =  &state.db.pg_pool;

    let row = sqlx::query("SELECT id, nickname, avatar FROM \"cola_user\" WHERE id=$1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(match row {
        Some(r) => UserInfo {
            id: r.get("id"),
            nickname: r.get("nickname"),
            avatar: r.get("avatar"),
            bg_img: None,
            email: None,
            age: None,
        },
        None => UserInfo {
            id: user_id.expect("REASON"),
            nickname: "用户不存在".to_string(),
            avatar: None,
            bg_img: None,
            email: None,
            age: None,
        },
    })
}

/// 通用函数：根据 `user_ids` 获取用户信息
pub async fn get_user_info2(
    pool: &PgPool,
    user_ids: &HashSet<i64>
) -> Result<HashMap<i64, UserInfo>, sqlx::Error> {

    // 在调用 get_user_info2 前后添加调试信息
    //println!("需要查询的用户ID: {:?}", user_ids);


    // 将 HashSet 转换为 Vec
    let ids_vec: Vec<i64> = user_ids.iter().cloned().collect();


    let users = sqlx::query_as::<_, UserInfo>(
        r#"
        SELECT id, nickname, email
        FROM cola_user
        WHERE id = ANY($1)
    "#
    )
        .bind(&ids_vec)
        .fetch_all(pool)
        .await?;

    //
    println!("{:?}", users);

    // 将查询结果转换成 HashMap，id -> UserInfo
    let user_map = users.into_iter().map(|user| (user.id, user)).collect();
    Ok(user_map)
}
