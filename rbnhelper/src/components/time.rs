#[allow(dead_code)]
pub fn format_seconds(second: f32) -> String {
    let minutes = (second / 60.0) as u32;
    let seconds = (second % 60.0) as u32;
    let milliseconds = ((second % 1.0) * 1000.0) as u32;

    format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
}