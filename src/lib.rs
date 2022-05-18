use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{ encode , decode };
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file:&str) -> String{

     let base64_to_vector=decode(encoded_file).unwrap();
     log(&"Emage Decoded".into());
     let mut image=load_from_memory(&base64_to_vector).unwrap();
     log(&"Emage sent to vector".into());

        image=image.grayscale();
        log(&"Gray effect applied".into());

        let mut buffer=vec![];
        image.write_to(& mut buffer, Png).unwrap();
        log(&"new image written".into());

        let encoded_img =encode(&buffer);
        let data_url= format!(
            "data:image/png;base64,{}",encoded_img
        );
    return data_url;
}

