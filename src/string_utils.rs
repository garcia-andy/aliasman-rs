
/// Remove the pattern from the init
pub fn remove_init(content: &str, pattern: &str) -> String 
{ replace_init(content, pattern, "") }

/// Replace the pattern from the init for to
pub fn replace_init(content: &str, from: &str, to: &str) -> String {
    if content.starts_with(from)
        { content.replacen(from, to, 1) }
    else 
        { String::from(content) }
}

/// Remove the pattern from the end
pub fn remove_end(content: &str, pattern: &str) -> String 
{ replace_end(content, pattern, "") }

/// Replace the pattern from the end for to
pub fn replace_end(content: &str, from: &str, to: &str) -> String {
    let mut result = String::from(content);
    
    if let Some(idx) = content.find(from){
        result = String::from(&content[..idx]);
        result.push_str(to);
    }

    result
}

/// Remove the pattern from the end and the init
pub fn remove_both(content: &str, pattern: &str) -> String 
{ replace_both(content, pattern, "") }

/// Replace the pattern from the end and init for to
pub fn replace_both(content: &str, from: &str, to: &str) -> String {
    replace_end(
        replace_init(content, from, to).as_str(), 
        from, 
        to
    )
}