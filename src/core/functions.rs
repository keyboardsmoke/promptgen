use std::path::PathBuf;

use super::functional;

pub fn register_functions(env: &mut minijinja::Environment, working_dir: &PathBuf) {
    functional::os::register(env, working_dir);
    functional::net::register(env, working_dir);
    functional::json::register(env, working_dir);
    functional::utility::register(env, working_dir);
}