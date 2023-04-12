#[link(wasm_import_module = "wasmedge_opencvmini")]
extern "C" {
    pub fn wasmedge_opencvmini_imdecode(arg0: i32, arg1: i32) -> i32;
    pub fn wasmedge_opencvmini_imshow(arg0: i32, arg1: i32, arg2: i32);
    pub fn wasmedge_opencvmini_waitkey(arg0: i32);
}

use std::fs;

fn main() {
    let v = fs::read("asset/35k.jpg").unwrap();

    unsafe {
        let mat_key = wasmedge_opencvmini_imdecode(v.as_ptr() as i32, v.len() as i32);
        let name = "hello";
        wasmedge_opencvmini_imshow(name.as_ptr() as i32, name.len() as i32, mat_key);
        wasmedge_opencvmini_waitkey(0);
    }
}
