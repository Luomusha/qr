mod util;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: u8);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(a: f32);

}

#[wasm_bindgen]
pub fn process_image_data(image_data: &[u8], width: usize, height: usize) -> ImageData {
    // 计算图像的平均灰度值
    let gray = util::gray_data(&image_data, width, height);

    // 高斯滤波，去掉噪声
    let size = 3;
    let filtered_gray = util::filter_image_data(&gray, width, height, size, 10.0);

    let pixel_count: usize = width * height;
    let mut processed_data = Vec::with_capacity(image_data.len());
    for i in 0..pixel_count {
        processed_data.push(filtered_gray[i]);
        processed_data.push(filtered_gray[i]);
        processed_data.push(filtered_gray[i]);
        processed_data.push(255); // 假设 alpha 通道始终为 255
    }
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&processed_data),
        width as u32,
        height as u32,
    )
    .unwrap()
}
