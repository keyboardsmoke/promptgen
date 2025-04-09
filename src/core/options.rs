
pub fn setup_options(env: &mut minijinja::Environment) {
    env.set_recursion_limit(500);   
    env.set_debug(true);
}