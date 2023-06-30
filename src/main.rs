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
}

use std::fs;

fn main() {
    let v = fs::read("asset/35k.jpg").unwrap();

    println!("start");

    unsafe {
        let src = imdecode(v.as_ptr() as i32, v.len() as i32);
        let output = blur(src);
        let output = normalize(output);

        let mut buf: Vec<f32> = vec![];
        buf.resize(v.len(), 0.);
        imencode(output, buf.as_ptr() as i32, buf.len() as i32);

        println!("{:?}", buf);
    }

    println!("done");
}
