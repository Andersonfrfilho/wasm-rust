use base64::{engine::general_purpose, Engine};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;
use std::str;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"GrayScale called".into());
    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    img = img.grayscale();
    log(&"Grayscale effectr applied".into());

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    img.write_to(&mut writer, Png).unwrap();
    log(&"color grayscale image applied".into());
    let encoded_img = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", &encoded_img);
    log(&data_url.clone().into());
    return data_url;
}
