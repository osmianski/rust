use std::collections::HashMap;

pub struct Env {
}

impl Env {
    pub fn new() -> Env {
        Env {
        }
    }

    pub fn load(&mut self, filename: &str) {
        panic!("not implemented");
    }

    /// Returns the value of the environment variable `key` if it exists, or `None` otherwise.
    /// 
    /// The value is loaded from these sources in order:
    /// 
    /// 1. OS environment variable
    /// 2. `.env` file
    /// 3. `.env.$APP_ENV` file
    pub fn get(&self, key: &str, default: Option<String>) -> Option<String> {
        match std::env::var(key) {
            Ok(env_value) => return Some(env_value),
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

        assert_eq!(env.get("FOO", None), Some("bar".to_string()));
        assert_eq!(env.get("BAR", None), None);
        assert_eq!(env.get("BAR", Some("default".to_string())), Some("default".to_string()));
    }
}

