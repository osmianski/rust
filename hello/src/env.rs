use std::collections::HashMap;

pub struct Env {
    /// Parsed `.env.$APP_ENV` file, or `None` if it's not parsed yet
    dot_app_env_file: Option<HashMap<String, String>>,

    /// Parsed `.env` file, or `None` if it's not parsed yet
    dot_file: Option<HashMap<String, String>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            dot_app_env_file: None, 
            dot_file: None,
        }
    }

    /// Returns the value of the environment variable `key` if it exists, or `None` otherwise.
    /// 
    /// The value is loaded from these sources in order:
    /// 
    /// 1. OS environment variable
    /// 2. `.env` file
    /// 3. `.env.$APP_ENV` file
    pub fn get(&mut self, key: &str) -> Option<String> {
        let env_value = std::env::var(key);

        if let Ok(env_value) = env_value {
            return Some(env_value);
        }

        let default_value = self.get_default().get(key);

        if let Some(default_value) = default_value {
            return Some(default_value.clone());
        }

        let current_value = self.get_current().get(key);

        if let Some(current_value) = current_value {
            return Some(current_value.clone());
        }

        None
    }

    /// Returns parsed `.env.$APP_ENV` file. Parses it on the first call.
    fn get_current(&mut self) -> &HashMap<String, String> {
        if let None = self.dot_app_env_file {
            self.dot_app_env_file = Some(Env::parse_file(".env"));
        }

        self.dot_app_env_file.as_ref().unwrap()
    }

    /// Returns parsed `.env` file. Parses it on the first call.
    fn get_default(&mut self) -> &HashMap<String, String> {
        if let None = self.dot_file {
            self.dot_file = Some(Env::parse_file(".env"));
        }

        self.dot_file.as_ref().unwrap()
    }

    fn parse_file(filename: &str) -> HashMap<String, String> {
        panic!("not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_just_reads_environment_variables_if_no_files_are_loaded() {
        let mut env = Env::new();

        std::env::set_var("FOO", "bar");

        assert_eq!(env.get("FOO"), Some("bar".to_string()));
        assert_eq!(env.get("BAR"), None);
    }
}

