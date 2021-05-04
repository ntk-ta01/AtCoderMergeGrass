use actix_web::{cookie::Cookie, HttpRequest};
use std::env;

/// 特定のキーで環境変数から値を取得するための関数
///
/// # Arguments
/// * `key` - 環境変数から取り出したい値のキー
///
/// # Return value
/// * String - 環境変数の値を文字列として取得する
///
pub fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => return value,
        Err(e) => println!("ENV: ERR {:?}", e),
    }
    String::new()
}

/// 環境変数に設定された HTTPS の値が 1 か判定する
/// Cookie の属性に Secure を付与するか判定するのに使用する
///
/// # Return value
/// * bool - Secure 属性を付与するか判定するための真偽値
///
pub fn is_https() -> bool {
    get_env("HTTPS") == "1"
}

/// 存在していれば、特定のキーで Cookie に設定された値を取得するための関数
///
/// # Arguments
/// * `key` - Cookie から取り出したい値のキー
/// * `cookie_string` - get_cookie_string_from_header 関数で取得した Cookie の文字列
///
/// # Return value
/// * Option<String> - Cookie に設定されている値を取得する
///
pub fn get_cookie_value(key: &str, cookie_string: String) -> Option<String> {
    // 取得した Cookie 文字列を ; で分割してループで回す
    let kv: Vec<&str> = cookie_string.split(';').collect();
    for c in kv {
        // Cookie 文字列をパースして key で指定した値とマッチしたキーが存在するかチェックする
        match Cookie::parse(c) {
            Ok(kv) => {
                if key == kv.name() {
                    // key で指定した値とマッチしたキーが存在していたら、その値を取得する
                    return Some(String::from(kv.value()));
                }
            }
            Err(e) => {
                println!("cookie parse error. -> {}", e);
            }
        }
    }
    None
}

/// 存在していれば、HTTP Request ヘッダーから Cookie 文字列を取得する関数
///
/// # Arguments
/// * `req` - actix_web::HttpRequest
///
/// # Return value
/// * Option<String> - key=value; key1=value1;~ のような Cookie の文字列
///
pub fn get_cookie_string_from_header(req: HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        let cookie_string = v.to_str().unwrap();
        return Some(String::from(cookie_string));
    }
    None
}
