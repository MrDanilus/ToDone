pub fn truncate_text_adaptive(text: &str, window_width: u32, base_width: u32, base_chars: usize) -> String {
    // Вычисляем отношение текущей ширины к базовой
    let mut scale = window_width as f32 / base_width as f32;

    // Нелинейная корректировка: при маленьких окнах скейл уменьшается сильнее
    scale = scale.powf(1.2); // Чем меньше степень, тем сильнее сжатие на маленьких окнах

    let allowed_chars = ((base_chars as f32) * scale).floor() as usize;

    if text.chars().count() > allowed_chars.saturating_sub(3) {
        let truncated = text
            .chars()
            .take(allowed_chars.saturating_sub(3))
            .collect::<String>();
        format!("{}...", truncated)
    } else {
        text.to_string()
    }
}