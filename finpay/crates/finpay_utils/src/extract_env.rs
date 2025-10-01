use std::{env, str::FromStr};

pub fn extract_env<T: FromStr>(env_key: &str) -> T
where
    T::Err: std::fmt::Display,
{
    let value = env::var(env_key).unwrap_or_else(|_| panic!("{env_key} is not set"));

    value
        .parse::<T>()
        .unwrap_or_else(|e| panic!("Failed to parse {env_key}='{value}' into target type: {e}"))
}
