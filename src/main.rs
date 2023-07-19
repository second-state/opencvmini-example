#[link(wasm_import_module = "wasmedge_opencvmini")]
extern "C" {
    #[link_name = "wasmedge_opencvmini_imdecode"]
    pub fn imdecode(arg0: i32, arg1: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_imencode"]
    pub fn imencode(arg0: i32, arg1: i32, arg2: i32);

    #[link_name = "wasmedge_opencvmini_imwrite"]
    pub fn imwrite(arg0: i32, arg1: i32, arg2: i32);

    #[link_name = "wasmedge_opencvmini_blur"]
    pub fn blur(arg0: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_normalize"]
    pub fn normalize(arg0: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_bilinear_sampling"]
    pub fn bilinear_sampling(arg0: i32, w: i32, h: i32) -> i32;
}

use std::fs;
use wasmedge_tensorflow_interface::{self};

fn main() {
    let v = fs::read("asset/35k.jpg").expect("failed to open image");

    let mod_buf = fs::read("asset/lite-model_ssd_mobilenet_v1_1_metadata_2.tflite")
        .expect("failed to open model");
    // The mod_buf is a vec<u8> which contains the model data.
    let mut session = wasmedge_tensorflow_interface::TFLiteSession::new(&mod_buf);

    println!("start");

    let mut buf: Vec<f32> = vec![];
    buf.resize(v.len(), 0.);

    unsafe {
        let img = imdecode(v.as_ptr() as i32, v.len() as i32);
        let img = normalize(img);

        // encode back to instance's buffer
        imencode(img, buf.as_ptr() as i32, buf.len() as i32);
    }

    println!("add input");
    session
        .add_input("normalized_input_image_tensor", &buf)
        .run();
    let res_vec: Vec<u8> = session.get_output("tensorflow/serving/classify");
    println!("{:?}", res_vec);

    println!("done");
}
