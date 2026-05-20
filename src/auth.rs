use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tiny_http::{Header, Response, Server};

use crate::config::Config;

pub fn check_status(config: &Config) -> bool {
    let Some(token) = config.load_token() else {
        println!("未登录（无 token 文件）");
        return false;
    };
    let url = format!("{}/users/me", config.base_url);
    let client = reqwest::blocking::Client::new();
    match client
        .get(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
    {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(user) = resp.json::<crate::types::User>() {
                println!("已登录: {} ({})", user.full_name, user.email);
                true
            } else {
                println!("Token 有效，但解析用户信息失败");
                true
            }
        }
        Ok(resp) => {
            println!("Token 无效: HTTP {}", resp.status());
            false
        }
        Err(e) => {
            println!("检查登录状态失败: {}", e);
            false
        }
    }
}

pub fn login(config: &Config) -> bool {
    if config.load_token().is_some() {
        println!("已有 token，尝试验证...");
        if check_status(config) {
            println!("当前 token 有效，无需重新登录。");
            println!("如需重新登录，请先执行 `biolab logout`");
            return true;
        }
        println!("Token 已过期，开始重新认证...\n");
    }

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    let port = listener
        .local_addr()
        .expect("Failed to get local addr")
        .port();

    let callback_url = format!("http://localhost:{port}/callback");
    let encoded_callback =
        url::form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
    let auth_url = format!(
        "{}/feishu/authorize?redirect={}",
        config.base_url, encoded_callback
    );

    let received_token: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let config_clone = config.clone();
    let token_clone = Arc::clone(&received_token);

    let server = Server::from_listener(listener, None).expect("Failed to create server");

    println!("\n{}", "=".repeat(55));
    println!("  请在浏览器中打开以下链接完成飞书认证：");
    println!("\n    {auth_url}\n");
    println!("  等待认证完成（最多 2 分钟），每 5 秒打印 .");
    println!("{}\n", "=".repeat(55));

    let start = Instant::now();

    for request in server.incoming_requests() {
        let url_str = request.url().to_string();
        if url_str.starts_with("/callback") {
            if let Some(query) = url_str.split('?').nth(1) {
                for pair in query.split('&') {
                    if let Some((key, value)) = pair.split_once('=') {
                        if key == "token" {
                            *token_clone.lock().unwrap() = Some(value.to_string());
                            break;
                        }
                    }
                }
            }

            let response = Response::from_string(
                "<html><body style='font-family:sans-serif;text-align:center;padding-top:3em'>\
                 <h2>Login Successful</h2>\
                 <p>Token has been saved. You may close this window.</p>\
                 </body></html>",
            )
            .with_header(Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap());
            let _ = request.respond(response);
        } else {
            let _ = request.respond(Response::empty(204));
        }

        if token_clone.lock().unwrap().is_some() {
            break;
        }

        if start.elapsed() > Duration::from_secs(120) {
            break;
        }

        if start.elapsed().as_secs() % 5 == 0 {
            print!(".");
            use std::io::Write;
            let _ = std::io::stdout().flush();
        }
    }

    println!();

    let token = received_token.lock().unwrap().take();
    if let Some(token) = token {
        if let Err(e) = config_clone.save_token(&token) {
            eprintln!("保存 token 失败: {}", e);
            return false;
        }
        println!("认证成功！Token 已保存到 ~/.biolab_token");
        check_status(&config_clone)
    } else {
        println!("\n认证超时（超过 2 分钟未收到回调）。");
        println!("请重新运行 `biolab login` 并在浏览器中打开授权链接。");
        false
    }
}

pub fn logout(config: &Config) {
    if config.remove_token().is_ok() {
        println!("已登出，Token 已删除。");
    } else {
        println!("未登录。");
    }
}
