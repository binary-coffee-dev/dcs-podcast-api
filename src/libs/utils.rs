pub fn get_env(env_name: &str, default: &str) -> String {
    for (key, value) in std::env::vars() {
        if key == String::from(env_name) {
            return value;
        }
    }
    String::from(default)
}
