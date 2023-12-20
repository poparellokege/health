use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use url::Url;

fn healthcheck(url: &str) -> Result<(), String> {
    let parsed_url = Url::parse(url).map_err(|e| format!("URL parsing error: {}", e))?;
    let client = Client::new();

    loop {
        let response = client.get(parsed_url.as_str()).send().map_err(|e| format!("HTTP request error: {}", e))?;
        match response.status().as_u16() {
            200 => println!("OK(200)"),
            code => println!("ERR({})", code),
        }
        thread::sleep(Duration::from_secs(10)); // Интервал между проверками (в данном случае 10 секунд)
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let url = match args.next() {
        Some(url) => url,
        None => {
            eprintln!("Usage: healthcheck URL");
            return;
        }
    };

    if let Err(e) = healthcheck(&url) {
        eprintln!("{}", e);
    }
}