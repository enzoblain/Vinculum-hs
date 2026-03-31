pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
            result.push_str(&ch.to_lowercase().to_string());
        } else {
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }
    }

    result
}
