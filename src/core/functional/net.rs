use std::{io::Read, path::PathBuf};

fn http_get_request(url: &str, useragent: Option<&str>) -> Result<String, minijinja::Error> {
    let mut request = ureq::get(url).header("User-Agent", useragent.unwrap_or("promptgen")).call().map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    let body = request.body_mut();
    let mut reader = body.as_reader();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    Ok(buf)
}

fn http_post_request(url: &str, body: &str, useragent: Option<&str>) -> Result<String, minijinja::Error> {
    let request = ureq::post(url).header("User-Agent", useragent.unwrap_or("promptgen")).send_json(body);
    let mut response = request.map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    let body = response.body_mut();
    let mut reader = body.as_reader();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    Ok(buf)
}

// Networking APIs
// httpget(url, useragent) -> send a HTTP GET request
// httppost(url, body, useragent) -> send a HTTP POST request
pub fn register(env: &mut minijinja::Environment, _working_dir: &PathBuf) {
    env.add_function("webget", http_get_request);
    env.add_function("webpost", http_post_request);
}