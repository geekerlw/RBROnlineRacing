pub mod store;
pub mod time;

macro_rules! meters_to_kilometers {
    ($meters:expr) => {
        format!("{:.1} Km", $meters as f64 / 1000.0)
    };
}