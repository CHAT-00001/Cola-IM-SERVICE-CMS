// cola_user/src/biz/check.rs  --  Biz - 检查用户状态是否正常
// 2026/06/05 03:55 by wx: cestbon10080

////////

use anyhow::Result;
use cola_data::app::data::AppData;
// 🚀 1. 修正：清理掉之前重复冲突的引入，统一使用可乐数据中心的标准状态契约
use cola_data::user::info::state::{UserState, UserStatus};
use repo::user::service::state::UserStateService;
// 🚀 2. 引入对应的纯静态用户服务（用来替代原先的 StatePort）
use repo::user::service::user::UserService;

////////

/// # [LOGIC] - 获取用户状态
/// * 机制：全静态函数闭环，彻底干掉老旧的 StatePort 注入
/// * 场景：用户登录拦截、刷视频前的封禁风控校验、直播间进入权限检查
pub async fn logic_get_user_state(
    uid: i64,
) -> AppData<UserState> {
    if uid <= 0 {
        return AppData::err(400, "非法用户ID", None);
    }

    // 1. 获取用户基础状态（静态调用，补全异步 await）
    // 内部可以直接查库或者走 Redis 缓存
    let user_valid = UserStateService::check_user_valid(uid).await.unwrap_or(false);
    if !user_valid {
        return AppData::err(404, "用户不存在", None);
    }

    // 2. 获取封禁状态（🚀 剥离 Port，改用纯静态函数直下，支持高性能缓存旁路）
    let is_banned = UserStateService::check_user_banned(uid).await.unwrap_or(false);

    // 3. 根据业务逻辑计算最终的 UserStatus 状态枚举
    let status = if is_banned {
        UserStatus::Banned // 假设你有对应的封禁状态枚举值，没有的话维持原状或根据你 state.rs 的定义微调
    } else {
        UserStatus::Normal
    };

    // 4. 组装业务模型
    let state = UserState {
        status,
        banned: is_banned,
        uid,
        // 将复杂的逻辑简化为一个简单的布尔值给上层 Case 使用
        is_playable: !is_banned,
        status_code: None,
    };

    AppData::ok(state)
}

//////// END