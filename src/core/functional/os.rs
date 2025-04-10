use std::{fs::File, io::Read, path::PathBuf};

// Environment/OS APIs
// getenv(key) -> get an environment variable as a string
// readfile(filename) -> read file into a string
pub fn register(env: &mut minijinja::Environment, working_dir: &PathBuf) {
    env.add_function("getenv", |key: &str| {
        std::env::var(key).map_err(|e| minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))
    });

    let read_file_wd = working_dir.clone();
    env.add_function("readfile", move |path: &str| {
        let file_with_path = read_file_wd.join(path);
        let mut file = File::open(file_with_path).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to open file")))?;
        let mut content = String::new();
        file.read_to_string(&mut content).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to read file")))?;
        Ok(content)
    });
}