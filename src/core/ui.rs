pub fn truncate_text(text: &String, max_len: usize) -> String{
    let mut result = text.clone();
    if text.len() > max_len{
        let _ = result.split_off(max_len);
        return format!("{result}...");
    }
    return result;
}