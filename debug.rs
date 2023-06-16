use std::fs::File;
use std::io::Write;

fn debug(file_name: &str, slice: &mut [f32], num: u32) {
    let bytes: &[u8] = bytemuck::cast_slice(slice);

    let mut file = File::create(file_name).expect("Failed to create file");
    file.write_all(bytes).expect("Failed to write to file");
}
