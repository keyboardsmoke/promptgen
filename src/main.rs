use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use minijinja::{Environment, context};

pub mod core;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    script_dir: Option<PathBuf>,

    #[clap(short, long)]
    script: String,
}

fn get_script_dir(script_dir: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if script_dir.is_some() {
        Ok(script_dir.unwrap())
    } else {
        std::env::current_dir().or_else(|_| Err(anyhow::anyhow!("Failed to get current directory")))
    }
}

fn get_script_path(script_base: &PathBuf, script: &str) -> PathBuf {
    let mut script_path = script_base.clone();
    script_path.push(script);
    script_path
}

fn get_pathbuf_filename(pathbuf: &PathBuf) -> anyhow::Result<String> {
    let filename = pathbuf.file_name();
    if filename.is_none() {
        return Err(anyhow::anyhow!("Failed to get file name"));
    }
    let filename = filename.unwrap().to_str();
    if filename.is_none() {
        return Err(anyhow::anyhow!("Failed to get file name"));
    }
    let filename = filename.unwrap().to_string();
    Ok(filename)
}

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

fn execute_script(working_dir: &PathBuf, script_path: &PathBuf, script_filename: &str) -> anyhow::Result<()> {
    let mut script = File::open(&script_path)?;
    let mut script_content = String::new();
    script.read_to_string(&mut script_content)?;

    let mut env = Environment::new();

    setup_env(working_dir, &mut env)?;

    env.add_template(&script_filename, &script_content)?;

    let prompt = env.get_template(&script_filename)?;
    let context = context!();
    let (rv, _) = prompt.render_and_return_state(context)?;
    print!("{}", rv);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let script_base = get_script_dir(args.script_dir)?;
    let script_path = get_script_path(&script_base, &args.script);
    let script_filename = get_pathbuf_filename(&script_path)?;
    let script_parent_dir = script_path.parent();
    if script_parent_dir.is_none() {
        return Err(anyhow::anyhow!("Failed to get script parent directory"));
    }
    let script_parent_dir = std::path::absolute(script_parent_dir.unwrap())?;
    execute_script(&script_parent_dir, &script_path, &script_filename)?;
    Ok(())
}
