#[link(wasm_import_module = "wasmedge_opencvmini")]
extern "C" {
    #[link_name = "wasmedge_opencvmini_imdecode"]
    pub fn imdecode(arg0: i32, arg1: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_imshow"]
    pub fn imshow(arg0: i32, arg1: i32, arg2: i32);
    #[link_name = "wasmedge_opencvmini_waitKey"]
    pub fn waitkey(arg0: i32);
    #[link_name = "wasmedge_opencvmini_blur"]
    pub fn blur(arg0: i32) -> i32;
    #[link_name = "wasmedge_opencvmini_imwrite"]
    pub fn imwrite(arg0: i32, arg1: i32, arg2: i32);
}

use std::fs;

fn main() {
    let v = fs::read("asset/35k.jpg").unwrap();

    println!("start");

    unsafe {
        let src = imdecode(v.as_ptr() as i32, v.len() as i32);
        let output = blur(src);
        let target_file = "output.jpg";
        imwrite(
            target_file.as_ptr() as i32,
            target_file.len() as i32,
            output,
        );
    }

    println!("done");
}
