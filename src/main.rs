use std::{collections::{BTreeMap, HashMap}, fs::File, io::Read, path::PathBuf};

use clap::Parser;
use minijinja::{Environment, context};

pub mod core;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    script_dir: Option<PathBuf>,

    #[clap(short, long)]
    script: String,

    // Here are multiple ways to supply arguments to the script

    // Supply arguments as a json file
    #[clap(long)]
    json_file: Option<PathBuf>,

    // JSON file base64 encoded for programmatic use
    #[clap(long)]
    base64: Option<String>,

    // JSON string, not as elegant but possible to use in command line
    #[clap(long)]
    json_string: Option<String>,
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

fn js_to_mj_value(value: serde_json::Value) -> anyhow::Result<minijinja::Value> {
    if value.is_string() {
        Ok(minijinja::Value::from(value.as_str().unwrap()))
    } else if value.is_number() {
        Ok(minijinja::Value::from(value.as_f64().unwrap()))
    } else if value.is_boolean() {
        Ok(minijinja::Value::from(value.as_bool().unwrap()))
    } else if value.is_array() {
        let arr = value.as_array().unwrap();
        let mut rv = Vec::new();
        for i in 0..arr.len() {
            rv.push(js_to_mj_value(arr[i].clone())?);
        }
        Ok(minijinja::Value::from(rv))
    } else if value.is_object() {
        return Err(anyhow::anyhow!("Object is not supported as a global variable"));
    } else {
        Err(anyhow::anyhow!("Unsupported value type"))
    }
}

fn setup_env(working_dir: &PathBuf, env: &mut Environment, script_args: Option<serde_json::Value>) -> anyhow::Result<()> {
    let function_dir = working_dir.clone();

    // Register globals
    if let Some(script_args) = script_args {
        for (key, value) in script_args.as_object().unwrap().iter() {
            let mj_value = js_to_mj_value(value.clone())?;
            env.add_global(key.clone(), mj_value);
        }
    }

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

fn execute_script(working_dir: &PathBuf, script_path: &PathBuf, script_filename: &str, script_args: Option<serde_json::Value>) -> anyhow::Result<()> {
    let mut script = File::open(&script_path)?;
    let mut script_content = String::new();
    script.read_to_string(&mut script_content)?;

    let mut env = Environment::new();

    setup_env(working_dir, &mut env, script_args)?;

    env.add_template(&script_filename, &script_content)?;

    let prompt = env.get_template(&script_filename)?;
    let context = context!();
    let (rv, _) = prompt.render_and_return_state(context)?;
    print!("{}", rv);
    Ok(())
}

fn get_script_arguments(args: &Args) -> Option<serde_json::Value> {
    if args.json_file.is_some() {
        let json_file = File::open(args.json_file.as_ref().unwrap());
        if json_file.is_err() {
            panic!("A JSON file was provided but could not be opened");
        }
        let mut json_file = json_file.unwrap();
        let mut json_content = String::new();
        let result = json_file.read_to_string(&mut json_content);
        if result.is_err() {
            panic!("A JSON file was provided but could not be read");
        }
        let json_value = serde_json::from_str(&json_content);
        if json_value.is_err() {
            panic!("A JSON file was provided but could not be parsed");
        }
        Some(json_value.unwrap())
    } else if args.base64.is_some() {
        let base64 = args.base64.as_ref().unwrap();
        let json_value = serde_json::from_str(&base64);
        if json_value.is_err() {
            panic!("A base64 encoded JSON string was provided but could not be parsed");
        }
        Some(json_value.unwrap())
    } else if args.json_string.is_some() {
        let json_value = serde_json::from_str(&args.json_string.as_ref().unwrap());
        if json_value.is_err() {
            panic!("A JSON string was provided but could not be parsed");
        }
        Some(json_value.unwrap())
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let script_base = get_script_dir(args.script_dir.clone())?;
    let script_path = get_script_path(&script_base, &args.script.clone());
    let script_filename = get_pathbuf_filename(&script_path)?;
    let script_parent_dir = script_path.parent();
    if script_parent_dir.is_none() {
        return Err(anyhow::anyhow!("Failed to get script parent directory"));
    }
    let script_parent_dir = std::path::absolute(script_parent_dir.unwrap())?;
    let script_args = get_script_arguments(&args);
    execute_script(&script_parent_dir, &script_path, &script_filename, script_args)?;
    Ok(())
}
