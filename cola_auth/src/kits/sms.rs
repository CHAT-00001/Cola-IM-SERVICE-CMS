// cola_auth/src/kits/sms.rs  -- AUTH - 工具包 - 构建短信验证码
// 2026/4/13 02:10 by wx: cestbon10080

////////

use rand::Rng;

////////

/// # [KITS] - 验证码生成器
/// 生成一个 6 位数字验证码，范围：000000 ~ 999999
pub fn kit_generate_6digit_code() -> String {
    let num = rand::thread_rng().gen_range(0..1_000_000);
    format!("{:06}", num)
}

/// # KITS - 构建验证短信内容
/// 返回：(验证码, 完整短信内容)
pub fn kit_make_auth_sms_content() -> (String, String) {
    let minute = 5;
    let auth_code = kit_generate_6digit_code();
    let app_name = "可乐APP";

    // Rust 的 format! 必须这样写
    let sms = format!(
        "【{}】你好，你正在登录验证，验证码是 {}，请在 {} 分钟内完成验证。若非本人操作请忽略。",
        app_name, auth_code, minute
    );

    (auth_code, sms)
}


//////// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sms_generation() {
        let (code, content) = kit_make_auth_sms_content();
        assert_eq!(code.len(), 6);
        println!("Code: {}, Content: {}", code, content);
    }
}

//////// END