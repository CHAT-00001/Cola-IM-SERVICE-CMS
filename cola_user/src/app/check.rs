// cola_user/src/api/check.rs -- 可乐用户 - 应用层 - 检查用户状态
// 2026/06/05 04:25

////////

use cola_data::app::data::AppData;
use crate::biz::check::logic_get_user_state; // 👈 引入本地 case 里的纯静态逻辑函数

////////

/// # [CASE] - 检查用户状态
pub async fn case_check_user_playable(
    uid: i64,
) -> AppData<()> {
    // 1. 在异步块中处理逻辑，允许使用 ?
    let result: Result<(), AppData<()>> = async {
        // 🚀 剥离 Port：不再传递 port 入参，纯静态单向直通
        let state_data = logic_get_user_state(uid).await;

        // 解析出内部的 UserState 业务模型（根据你的 AppData 解包设计调整，通常是用 check() 或 rebind()）
        let state = state_data.check().map_err(|e| e.rebind::<()>())?;

        // 🚀 根据年龄/状态等业务核心指标，直接使用刚才洗干净的 state 布尔值防御
        if !state.is_playable {
            return Err(AppData::err(4002, "用户状态异常", None));
        }

        if state.banned {
            return Err(AppData::err(4003, "用户已被封禁", None));
        }

        Ok(())
    }.await;

    // 2. 将 Result 还原回 AppData
    match result {
        Ok(_) => AppData::ok(()),
        Err(err_app_data) => err_app_data,
    }
}

////////

/* /// [CASE]- ensure 用户隐私权限守卫
/// * (专职双向黑名单与对方隐私检索)
/// * 💡 备忘：后续改造该函数时，也直接采用静态服务调用，千万别再塞 Port 了！
pub async fn ensure_user_permission<T>(
    current_user_id: i64,
    target_user_id: i64,
) -> Result<(), AppData<T>> {
    Ok(())
} */

//////// END