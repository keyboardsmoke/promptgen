
pub fn setup_options(env: &mut minijinja::Environment) {
    // Set recursion limit
    env.set_recursion_limit(500);
    // Debug mode
    env.set_debug(true);
    // Remove leading spaces and tabs from the start of a line to a block.
    env.set_lstrip_blocks(true);
    // Remove the first newline after a block.
    env.set_trim_blocks(true);
    // Complains very quickly about undefined values.
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
}
