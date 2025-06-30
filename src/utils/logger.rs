use actix_web::HttpRequest;
use chrono::Utc;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_request(req: &HttpRequest, route: &str, body: Option<&Value>) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
    let method = req.method().as_str();
    let uri = req.uri().to_string();

    let log_entry = format!(
        "\n=== REQUEST {} ===\nTimestamp: {}\nMethod: {}\nRoute: {}\nURI: {}\nHeaders: {:?}\nBody: {}\n",
        timestamp,
        timestamp,
        method,
        route,
        uri,
        req.headers(),
        body.map(|b| b.to_string()).unwrap_or_else(|| "None".to_string())
    );

    write_to_log(&log_entry);
}

pub fn log_response(route: &str, status_code: u16, response_body: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();

    let log_entry = format!(
        "=== RESPONSE {} ===\nTimestamp: {}\nRoute: {}\nStatus Code: {}\nResponse Body: {}\n==================\n\n",
        timestamp,
        timestamp,
        route,
        status_code,
        response_body
    );

    write_to_log(&log_entry);
}

fn write_to_log(content: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("api_logs.txt")
    {
        let _ = file.write_all(content.as_bytes());
        let _ = file.flush();
    }
}

pub fn log_startup() {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
    let log_entry = format!(
        "\n\n🚀 ================ SERVER STARTED {} ================\n\n",
        timestamp
    );
    write_to_log(&log_entry);
}
