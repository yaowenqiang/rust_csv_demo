// #![deny(warnings)]
// use std::env;
extern crate reqwest;
// use reqwest::Client;
// use reqwest::blocking;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::result::Result;
// use std::path::Path;
use std::process;
// use std::collections::HashMap;

#[tokio::main]
async fn main() -> std::result::Result<(), reqwest::Error> {
    // if let Ok(lines) = read_lines("./hosts") {
    //     for line in lines {
    //         if let Ok(ip) = line {
    //             println!("{}", ip);
    //         }
    //     }
    // }

    // let filename = "src/main.rs";
    let filename = "src/test.csv";
    // let filename = "E:\\2020-07-24_112628.csv";
    let filename = "E:\\url8.csv";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut i = 0;
    // let client =  Client::new();

    for (index, line) in reader.lines().enumerate() {
        // if i >= 10 {
        //     process::exit(0);
        // }
        let line = line.unwrap();
        // let strings : Vec<&str> = line.split(",").collect();

        // println!("{:?}", strings);
        // println!("{}", strings[1]);
        // async {
        //     let body = reqwest::get("https://www.baidu.com")
        //     .await?
        //     .text()
        //     .await?;
        //     println!("{:?}", body);
        // };

        // let resp = reqwest::get("https://httpbin.org/ip").await?;
        // println!("response.status: {}", resp.status());


        i = i + 1;
        let proxy = reqwest::Proxy::http("http://127.0.0.1:8888")?;
        // let client = reqwest::Client::new();
        let client = reqwest::Client::builder()
            .proxy(proxy)
            .build()?;
        let request_url = format!("http://localhost:8889/", line) ;

        let resp = client.get(&request_url)
            // .body(request_body)
            .send()
            .await?
            .text()
            .await?;
        
        // println!("response.status: {}", resp.status());

        // println!("response.body: {}", resp);
        println!("{}", resp);

        // foo().unwrap();

    }
    Ok(())
}


// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename);
//     Ok(io::BufReader::new(file).lines())
// }


// fn foo() -> Result<(), reqwest::Error> {
//     // let body = reqwest::get("https://httpbin.org/status/418")?
//     //     .text()?;
//     //     println!("{}", body);
//     //     Ok(())
// }