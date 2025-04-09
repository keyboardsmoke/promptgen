use std::{fs::File, io::Read, path::PathBuf};

pub fn http_get_request(url: &str, useragent: Option<&str>) -> Result<String, minijinja::Error> {
    let mut request = ureq::get(url).header("User-Agent", useragent.unwrap_or("promptgen")).call().map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    let body = request.body_mut();
    let mut reader = body.as_reader();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    Ok(buf)
}

pub fn http_post_request(url: &str, body: &str, useragent: Option<&str>) -> Result<String, minijinja::Error> {
    let request = ureq::post(url).header("User-Agent", useragent.unwrap_or("promptgen")).send_json(body);
    let mut response = request.map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    let body = response.body_mut();
    let mut reader = body.as_reader();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
    Ok(buf)
}

pub fn register_functions(env: &mut minijinja::Environment, working_dir: &PathBuf)
{
    env.add_function("getenv", |key: &str| {
        std::env::var(key).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))
    });

    env.add_function("httpget", http_get_request);
    env.add_function("httppost", http_post_request);

    let read_file_wd = working_dir.clone();
    env.add_function("readfile", move |path: &str| {
        let file_with_path = read_file_wd.join(path);
        let mut file = File::open(file_with_path).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to open file")))?;
        let mut content = String::new();
        file.read_to_string(&mut content).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to read file")))?;
        Ok(content)
    });
    let read_json_wd = working_dir.clone();
    env.add_function("readjson", move |path: &str| -> Result<minijinja::Value, minijinja::Error> {
        let file_with_path = read_json_wd.join(path);
        let mut file = File::open(file_with_path).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to open file")))?;
        let mut content = String::new();
        file.read_to_string(&mut content).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to read file")))?;
        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
        let res = minijinja::Value::from_serialize(&json);
        Ok(res)
    });
}