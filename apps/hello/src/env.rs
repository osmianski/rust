use std::collections::HashMap;

pub struct Env {
}

impl Env {
    pub fn new() -> Env {
        Env {
        }
    }

    pub fn load_file(&mut self, filename: &str) {
        panic!("not implemented");
    }

    pub fn load_string(&mut self, contents: &str) {
        // TODO: continue here
        panic!("not implemented");
    }

    /// Returns the value of the environment variable `key` if it exists, or the `default` value otherwise.
    pub fn get(&self, key: &str, default: Option<String>) -> Option<String> {
        match std::env::var(key) {
            Ok(env_value) => Some(env_value),
            Err(_) => default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_just_reads_environment_variables_if_no_files_are_loaded() {
        let env = Env::new();

        std::env::set_var("FOO", "bar");

        assert_eq!(env.get("FOO", None), Some(String::from("bar")));
        assert_eq!(env.get("BAR", None), None);
        assert_eq!(env.get("BAR", Some(String::from("default"))), Some(String::from("default")));
    }

    #[test]
    fn it_prioritizes_environment_variable_value_over_loaded_one() {
        let mut env = Env::new();

        env.load_string("FOO=loaded");
        assert_eq!(env.get("FOO", None), Some(String::from("loaded")));

        std::env::set_var("FOO", "bar");
        assert_eq!(env.get("FOO", None), Some(String::from("bar")));
    }
}

