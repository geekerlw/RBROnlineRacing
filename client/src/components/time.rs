pub fn format_duration(duration: f32) -> String {
    let minutes = (duration / 60.0) as u32;
    let seconds = (duration % 60.0) as u32;
    let milliseconds = ((duration % 1.0) * 1000.0) as u32;

    format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
}