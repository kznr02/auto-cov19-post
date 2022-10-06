use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::collections::HashMap;
use std::{fmt, io};

async fn get() -> Result<HashMap<String, String>, reqwest::Error> {
    Ok(reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?)
}

async fn post(
    username: &String,
    password: &String,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let cilent = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();

    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded; charset=UTF-8"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "Content-Length",
        fmt::format(format_args!("{}", username.len() + password.len() + 19))
            .parse()
            .unwrap(),
    );
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36 Edg/106.0.1370.34".parse().unwrap());

    let mut load = HashMap::new();
    load.insert("username", username);
    load.insert("password", password);

    Ok(cilent
        .post("https://app.xaut.edu.cn/uc/wap/login/check")
        .headers(headers)
        .form(&load)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}

#[tokio::main]
async fn main() {
    println!("欢迎使用疫情通自动填报系统！初次使用请输入学号以及服务门户密码");
    println!("请输入你的学号:");
    let mut userName = String::new();
    match io::stdin().read_line(&mut userName) {
        Ok(n) => {
            userName = userName.trim().to_string();
            println!("您输入的学号是：{:?}", userName);
        }
        Err(error) => println!("输入错误！"),
    }

    println!("请输入服务门户密码:");
    let mut passWord = String::new();
    match io::stdin().read_line(&mut passWord) {
        Ok(n) => {
            passWord = passWord.trim().to_string();
            println!("您输入的密码是：{:?}", passWord);
        }
        Err(error) => println!("输入错误！"),
    }

    if let Ok(resp) = post(&userName, &passWord).await {
        println!("{:?}", resp);

        match resp.get("e").unwrap().as_i64().unwrap() {
            1 => println!("账号或密码错误，请核对后输入"),
            10002 => println!("表头传递参数有误"),
            10016 => println!("错误次数已达最大上限,请稍后再试"),
            _ | 0 => println!("登陆成功，正在打卡"),
        }
    }
}
