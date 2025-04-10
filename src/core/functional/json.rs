use std::{fs::File, io::Read, path::PathBuf};



// JSON APIs
// readjson(filename) -> read a parsed JSON object, return jinja2 variable
// parsejson(string) -> parse a JSON object, return jinja2 variable
fn register_json_apis(env: &mut minijinja::Environment, working_dir: &PathBuf) {
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

    env.add_function("parsejson", |data: &str| -> Result<minijinja::Value, minijinja::Error> {
        let json: serde_json::Value = serde_json::from_str(&data).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))?;
        let res = minijinja::Value::from_serialize(&json);
        Ok(res)
    });
}

pub fn register(env: &mut minijinja::Environment, working_dir: &PathBuf) {
    register_json_apis(env, working_dir);
}