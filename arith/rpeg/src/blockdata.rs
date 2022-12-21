#![allow(unused)]
use crate::structs::PixelBlock;
pub use array2::Array2;


pub fn block_pix<T: std::clone::Clone>(pixels: Vec<T>, width: u32, height: u32) -> Vec<PixelBlock<T>> {
    let mut pixel_blocks: Vec<PixelBlock<T>> = Vec::with_capacity((width * height / 4) as usize);
    let pixels_array = Array2::from_row_major(width.try_into().unwrap(), height.try_into().unwrap(), pixels);
    // Iterate over 2x2 blocks of pixels
    for row  in (0..height).step_by(2) {
        for col  in (0..width).step_by(2) {
            let new_block = PixelBlock {
                pixel_1: pixels_array.get_element(row.try_into().unwrap(), col.try_into().unwrap()).unwrap().clone(),
                pixel_2: pixels_array.get_element(row.try_into().unwrap(), (col+1).try_into().unwrap()).unwrap().clone(),
                pixel_3: pixels_array.get_element((row+1).try_into().unwrap(), col.try_into().unwrap()).unwrap().clone(),
                pixel_4: pixels_array.get_element((row+1).try_into().unwrap(), (col+1).try_into().unwrap()).unwrap().clone(),
            };
            pixel_blocks.push(new_block);
        }
    }
    pixel_blocks // Return vector of pixel blocks
}


pub fn pixel_vec<T: std::clone::Clone>(blocks: Vec<PixelBlock<T>>, width: usize, height: usize) -> Vec<T> {
    let mut pixels = Vec::with_capacity(width * height);
    for block in blocks {
        pixels.push(block.pixel_1);
        pixels.push(block.pixel_2);
        pixels.push(block.pixel_3);
        pixels.push(block.pixel_4);
    }
    pixels
}

