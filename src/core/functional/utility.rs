use std::path::PathBuf;

// Utility APIs
// uuid()
// now(format="%Y-%m-%d %H:%M:%S")
pub fn register(env: &mut minijinja::Environment, _working_dir: &PathBuf) {
    env.add_function("now", |format: &str| {
        let now = chrono::Local::now();
        now.format(format).to_string()
    });

    env.add_function("uuid", || {
        uuid::Uuid::new_v4().to_string()
    });
}