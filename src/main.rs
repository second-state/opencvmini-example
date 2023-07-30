#[link(wasm_import_module = "wasmedge_opencvmini")]
extern "C" {
    #[link_name = "wasmedge_opencvmini_imdecode"]
    pub fn imdecode(arg0: i32, arg1: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_imencode"]
    pub fn imencode(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32);

    #[link_name = "wasmedge_opencvmini_imwrite"]
    pub fn imwrite(arg0: i32, arg1: i32, arg2: i32);

    #[link_name = "wasmedge_opencvmini_normalize"]
    pub fn normalize(arg0: i32) -> i32;
}

use std::fs;
use wasmedge_tensorflow_interface::*;

fn main() {
    let mut image = fs::read("asset/image_1_thumb.jpg").expect("failed to open image");

    let mod_buf = fs::read("asset/lite-model_ssd_mobilenet_v1_1_metadata_2.tflite")
        .expect("failed to open model");
    // The mod_buf is a vec<u8> which contains the model data.
    let mut session = TFLiteSession::new(&mod_buf);

    println!("start");

    let mut buf: Vec<u8> = vec![];
    buf.resize(270000, 0);
    image.resize(270000, 0);

    unsafe {
        let img = imdecode(image.as_ptr() as i32, image.len() as i32);
        let img = normalize(img);

        // encode back to instance's buffer
        let ext = ".jpg";
        imencode(
            ext.as_ptr() as i32,
            ext.len() as i32,
            img,
            buf.as_ptr() as i32,
            buf.len() as i32,
        );
    }

    println!("add input");
    session
        .add_input("normalized_input_image_tensor", &image)
        .run();
    println!("input added");

    // Described in https://www.tensorflow.org/lite/examples/object_detection/overview#output_signature
    // 0 Locations: Multidimensional array of [N][4] floating point values between 0 and 1, the inner arrays representing bounding boxes in the form [top, left, bottom, right]
    let locations: Vec<f32> = session.get_output("TFLite_Detection_PostProcess");
    for i in 0..locations.len() / 4 {
        println!("{}: {:?}", i, &locations[i * 4..i * 4 + 4]);
    }
    // 1 Classes: Array of N integers (output as floating point values) each indicating the index of a class label from the labels file
    let classes: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:1");
    println!("Classes: {:?}", classes);
    // 2 Scores: Array of N floating point values between 0 and 1 representing probability that a class was detected
    let scores: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:2");
    println!("Scores: {:?}", scores);
    // 3 Number of detections: Integer value of N
    let number_of_detections: Vec<u8> = session.get_output("TFLite_Detection_PostProcess:3");
    let num = u32::from_ne_bytes(number_of_detections[0..4].try_into().unwrap());
    println!("Number of detections: {}", num);

    println!("done");
}
