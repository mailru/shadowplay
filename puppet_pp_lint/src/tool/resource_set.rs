pub fn is_valid_metaparameter_value<EXTRA>(
    name: &str,
    value: &puppet_lang::expression::Expression<EXTRA>,
) -> Option<bool> {
    match name {
        "alias" => Some(puppet_tool::expression::string_constant_value(value).is_some()),
        "loglevel" => match puppet_tool::expression::string_constant_value(value) {
            None => Some(true),
            Some(v) => match v.as_str() {
                "emerg" | "alert" | "crit" | "err" | "warning" | "notice" | "info" | "verbose"
                | "debug" => Some(true),
                _ => Some(false),
            },
        },
        "audit" | "before" | "noop" | "notify" | "require" | "schedule" | "stage" | "subscribe"
        | "tag" => Some(true),
        _ => None,
    }
}
