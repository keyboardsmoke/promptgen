use minijinja::Environment;

pub fn register_filters(env: &mut Environment)
{
    env.add_filter("repeat", |s: &str, n: usize| s.repeat(n));  
}