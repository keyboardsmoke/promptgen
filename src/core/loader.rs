use std::{fs::File, io::Read, path::PathBuf};


pub fn register_loader(env: &mut minijinja::Environment, working_dir: &PathBuf) {
    let working_dir = working_dir.clone();
    env.set_loader(move |f| {
        let file_with_path = working_dir.join(f);
        let mut file = File::open(file_with_path).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to open file")))?;
        let mut content = String::new();
        file.read_to_string(&mut content).or_else(|_| Err(minijinja::Error::new(minijinja::ErrorKind::TemplateNotFound, "Failed to read file")))?;
        Ok(Some(content))
    });
}