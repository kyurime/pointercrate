use pointercrate_core::util::from_env_or_default;

pub fn list_size() -> i16 {
    from_env_or_default("LIST_SIZE", 75)
}

pub fn extended_list_size() -> i16 {
    from_env_or_default("EXTENDED_LIST_SIZE", 150)
}
