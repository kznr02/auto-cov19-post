use chrono::prelude::*;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::value::Value;
use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time;

async fn json_read(path: &str) -> serde_json::Value {
    let file = std::fs::File::open(path).unwrap();
    let buffer = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(buffer).unwrap();
    json
}

async fn post(
    client: &Client,
    username: &String,
    password: &String,
) -> Result<HashMap<String, Value>, reqwest::Error> {
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
    let ret = client
        .post("https://app.xaut.edu.cn/uc/wap/login/check")
        .headers(headers)
        .form(&load)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(ret)
}

async fn sign(
    client: &Client,
    username: &String,
    password: &String,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("zgfxdq", "0");
    form.insert("mjry", "0");
    form.insert("csmjry", "0");
    form.insert("tw", "2");
    form.insert("sfcxtz", "0");
    form.insert("sfjcbh", "0");
    form.insert("sfcxzysx", "0");
    form.insert("qksm", "");
    form.insert("sfyyjc", "0");
    form.insert("jcjgqr", "0");
    form.insert("remark", "");
    form.insert("address", "陕西省西安市碑林区太乙路街道交大街兰蒂斯城1期");
    form.insert("geo_api_info", "{\"type\":\"complete\",\"position\":{\"Q\":34.244535861546,\"R\":108.993039279514,\"lng\":108.993039,\"lat\":34.244536},\"location_type\":\"html5\",\"message\":\"Get ipLocation failed.Get geolocation success.Convert Success.Get address success.\",\"accuracy\":1571,\"isConverted\":true,\"status\":1,\"addressComponent\":{\"citycode\":\"029\",\"adcode\":\"610103\",\"businessAreas\":[{\"name\":\"等驾坡\",\"id\":\"610113\",\"location\":{\"Q\":34.234888,\"R\":109.00945300000001,\"lng\":109.009453,\"lat\":34.234888}}],\"neighborhoodType\":\"\",\"neighborhood\":\"\",\"building\":\"\",\"buildingType\":\"\",\"street\":\"交大商场街\",\"streetNumber\":\"1幢\",\"country\":\"中国\",\"province\":\"陕西省\",\"city\":\"西安市\",\"district\":\"碑林区\",\"towncode\":\"610103005000\",\"township\":\"太乙路街道\"},\"formattedAddress\":\"陕西省西安市碑林区太乙路街道交大街兰蒂斯城1期\",\"roads\":[],\"crosses\":[],\"pois\":[],\"info\":\"SUCCESS\"}");
    form.insert("area", "陕西省+西安市+碑林区");
    form.insert("province", "陕西省");
    form.insert("city", "西安市");
    form.insert("sfzx", "1");
    form.insert("sfjcwhry", "0");
    form.insert("sfjchbry", "0");
    form.insert("sfcyglq", "0");
    form.insert("gllx", "");
    form.insert("glksrq", "");
    form.insert("jcbhlx", "");
    form.insert("jcbhrq", "");
    form.insert("bztcyy", "");
    form.insert("sftjhb", "0");
    form.insert("sftjwh", "0");
    form.insert("jcjg", "");

    let date: DateTime<Utc> = Utc::now();
    let fmt_date = "%Y%m%d";
    let date_str = date.format(fmt_date).to_string();

    form.insert("date", &date_str.as_str());
    form.insert("uid", "63455");

    let millis = date.timestamp_millis() / 1000;
    let millis_str = millis.to_string();

    form.insert("created", &millis_str.as_str());

    form.insert("jcqzrq", "");
    form.insert("sfjcqz", "");
    form.insert("szsqsfybl", "0");
    form.insert("sfsqhzjkk", "0");
    form.insert("sqhzjkkys", "");
    form.insert("sfygtjzzfj", "0");
    form.insert("gtjzzfjsj", "");
    form.insert("fxyy", "在");
    form.insert("id", "20879237");
    form.insert("gwszdd", "");
    form.insert("sfyqjzgc", "");
    form.insert("jrsfqzys", "");
    form.insert("sfyqjzgc", "");
    form.insert("jrsfqzys", "");
    form.insert("jrsfqzfy", "");
    form.insert("ismoved", "0");

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded; charset=UTF-8"
            .parse()
            .unwrap(),
    );

    let mut count = 0;
    for it in form.iter() {
        count += it.0.len() + it.1.len() + 1;
    }
    count += form.len() - 1;

    headers.insert(
        "Content-Length",
        fmt::format(format_args!("{}", count)).parse().unwrap(),
    );
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36 Edg/106.0.1370.34".parse().unwrap());

    Ok(client
        .post("https://app.xaut.edu.cn/ncov/wap/default/save")
        .headers(headers)
        .form(&form)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
}
#[tokio::main]
async fn main() {
    println!("欢迎使用疫情通自动填报系统！初次使用请向json添加账号密码");

    let json = json_read("account.json").await;
    let c = json.as_array().unwrap();

    let mut userName = String::new();
    let mut passWord = String::new();

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    for i in c.iter() {
        if i.is_object() {
            userName = i["username"].to_string();
            passWord = i["password"].to_string().trim_matches('"').to_string();

            println!("正在签到用户: {} \n\r密码为: {}", userName, passWord);

            if let Ok(resp) = post(&client, &userName, &passWord).await {
                // println!("{:?}", resp);

                match resp.get("e").unwrap().as_i64().unwrap() {
                    1     =>    println!("账号或密码错误，请核对后输入"),
                    10002 =>    println!("表头传递参数有误"),
                    10016 =>    println!("错误次数已达最大上限,请稍后再试"),
                    _ | 0 => {
                        println!("登陆成功，正在打卡");
                        let mut resp = sign(&client, &userName, &passWord).await.unwrap();
                        loop {
                            if resp.get("m").unwrap() == "定位信息不能为空" {
                                println!("打卡失败正在重试");
                                std::thread::sleep(time::Duration::from_secs(1));
                                resp = sign(&client, &userName, &passWord).await.unwrap();
                            }
                            else {
                                break;
                            }
                        }
                        println!("打卡成功");
                    }
                }
            }
        }
    }
}
