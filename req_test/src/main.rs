extern crate reqwest;

use std::io::stdout;
use std::io::copy;

use std::collections::HashMap;

fn get() {
    let mut response = reqwest::get("http://www.baidu.com").expect("Faild to send request");
    println!("{}", response.status());

    for header in response.headers().iter() {
        let (name, value) = header;
        println!("{}: {:?}", name, value);
    }

    copy(&mut response, &mut stdout()).expect("Failed to read response");
}

fn post() {
    let mut params = HashMap::new();
    params.insert("cinemaId","0d49c7dd117343a4bbb417a71907dd20");
    params.insert("cinemaName","测试影院");
    params.insert("unionId","oXTvQ1HiWhWD4rrNzA6tAI6mApzE");
    params.insert("movieId","932b978fa7824c8cb1ff035653c73f5e");
    params.insert("grade","3");
    params.insert("score","152");
    params.insert("token","dfe2d638bbc64592b1490482a64b48e5");

    let client = reqwest::Client::new();
    
    //let mut response = client.get("http://ote.ddspace.com.cn/wxs/wx/jssdk_params?token=dfe2d638bbc64592b1490482a64b48e5")
    let mut response = client.post("http://ote.ddspace.com.cn/wxs/wx/pushGameInfo?token=dfe2d638bbc64592b1490482a64b48e5")
    //let mut response = client.post("http://localhost:8080/wxs/wx/pushGameInfo?token=dfe2d638bbc64592b1490482a64b48e5")
        .json(&params)
        .send()
        .expect("Failed to send request");

    copy(&mut response, &mut stdout()).expect("Faild to read response");
}

fn main() {
    post();
}
