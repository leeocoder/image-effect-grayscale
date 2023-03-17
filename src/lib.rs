use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{encode, decode};
use image::{load_from_memory, ImageOutputFormat::Png};
use std::{io::Cursor, fmt::format};

#[wasm_bindgen]
pub fn grayscale (encoded: &str) -> String {
  log(&"Grayscale called".into());
  let base64_to_vector = decode(encoded).unwrap();
  log(&"Image decode".into());
  
  let mut image = load_from_memory(&base64_to_vector).unwrap();
  log(&"Image load".into());
  
  image = image.grayscale();
  log(&"Grayscale effect applied".into());
  
  let mut buffer = Cursor::new(vec![]);
  image.write_to(&mut buffer, Png).unwrap();
  log(&"New Image Written".into());
  let png_bytes = buffer.into_inner();

  let encoded_image = encode(&png_bytes);
  let data_url = format!("data:image/png;base64,{}", encoded_image);
  data_url
}
