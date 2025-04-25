use std::{collections::HashMap, ffi::CStr, fs::File, io::Read, path::PathBuf};
use minijinja::{Environment, context};

pub mod core;

fn setup_env(working_dir: &PathBuf, env: &mut Environment) -> anyhow::Result<()> {
    let function_dir = working_dir.clone();

    // Register all core functions
    core::functions::register_functions(env, &function_dir);

    // Register all core filters
    core::filters::register_filters(env);

    let loader_dir = working_dir.clone();

    // Register the loader
    core::loader::register_loader(env, &loader_dir);

    // Set any remaining options
    core::options::setup_options(env);

    Ok(())
}

pub fn run_script(working_dir: &PathBuf, script_path: &PathBuf, script_filename: &str, arguments: Option<HashMap<String, String>>) -> anyhow::Result<String> {
    let script = File::open(&script_path);
    if script.is_err() {
        return Err(anyhow::anyhow!("Failed to open script file: {}", script_path.to_string_lossy()));
    }
    let mut script = script.unwrap();
    let mut script_content = String::new();
    let script_result = script.read_to_string(&mut script_content);
    if script_result.is_err() {
        return Err(anyhow::anyhow!("Failed to read script file: {}", script_path.to_string_lossy()));
    }

    let mut env = Environment::new();

    setup_env(working_dir, &mut env)?;

    env.add_template(&script_filename, &script_content)?;

    if let Some(arguments) = arguments {
        for (key, value) in arguments {
            env.add_global(key, value);
        }
    }

    let prompt = env.get_template(&script_filename)?;
    let context = context!();
    let (rv, _) = prompt.render_and_return_state(context)?;
    Ok(rv)
}

pub fn execute_script(working_dir: &PathBuf, script_path: &PathBuf, script_filename: &str) -> anyhow::Result<()> {
    let rv = run_script(working_dir, script_path, script_filename, None)?;
    println!("{}", rv);
    Ok(())
}

#[unsafe(no_mangle)]
pub extern "system" fn execute(working_directory: *const i8, script_path: *const i8, script_filename: *const i8) -> bool {
    unsafe {
        let working_directory = CStr::from_ptr(working_directory).to_string_lossy();
        let script_path = CStr::from_ptr(script_path).to_string_lossy();
        let script_filename = CStr::from_ptr(script_filename).to_string_lossy();

        let working_dir = PathBuf::from(working_directory.to_string());
        let script_path = PathBuf::from(script_path.to_string());

        let res = execute_script(&working_dir, &script_path, &script_filename);
        if res.is_err() {
            return false;
        }
        true
    }
}