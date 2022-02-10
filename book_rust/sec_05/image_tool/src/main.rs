use image_tool::{filter_negapogi, make_ichimatu, make_thumb};

pub fn main() {
    make_ichimatu();
    let args: Vec<String> = std::env::args().collect();
    let fname = String::from(&args[1]);
    // make_thumb(fname);
    filter_negapogi(fname);
}
