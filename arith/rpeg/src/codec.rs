#![allow(unused)]
use csc411_image::{Read,Write,RgbImage};
use csc411_rpegio;
pub use array2::Array2;
use crate::structs::{PixelBlock,CosineFloats,VideoComponents,WordStruct};
use crate::conversions;
use crate::wordfill;
use crate::blockdata;
use std::convert::TryInto;


pub fn compress(filename: Option<String>) {
    // Read in the image data
    let image = RgbImage::read(filename.as_deref()).unwrap();
    let mut width = image.width;
    let mut height = image.height;
    let denominator = image.denominator;

    // Trim the height and width if necessary
    trim_dimensions(&mut width, &mut height);

    // Set the word struct for compression and decompression
    let curr_word = WordStruct::set_word_struct();

    // Convert the RGB int pixel values to RGB float values
    let pixels = conversions::rgb_ints_conversion_to_floats(image.pixels, denominator);

    // Convert the RGB float values to component video pixels
    let vid_pixels = conversions::rgb_floats_conversion_to_component_video(pixels);

    // Divide the component video pixels into blocks of 2x2 pixels
    let vec_pixel_blocks = blockdata::block_pix(vid_pixels, width, height);

    // Convert the blocks of component video pixels to cosine coefficients
    let img_cosine = components_to_coefficients(vec_pixel_blocks);

    // Scale and quantize the cosine coefficients, then create and fill codewords from them
    let all_codewords = scale_quantize_fill_all_words(curr_word, img_cosine);

    // Output the codewords to stdout
    csc411_rpegio::output_rpeg_data(&all_codewords, width, height);
}

//  ensure that both the width and height of the image are even
pub fn trim_dimensions(width: &mut u32, height: &mut u32) {
    if *width % 2 != 0 {
        *width -= 1;
    }
    if *height % 2 != 0 {
        *height -= 1;
    }
}


pub fn components_to_coefficients(blocks_of_video_components: Vec<PixelBlock<VideoComponents>>) -> Vec<CosineFloats>{
    let mut image_cosine_coefficients : Vec<CosineFloats> = Vec::new();
    for block_of_pixels in blocks_of_video_components{
        let new_cosine_coefficients = wordfill::discrete_cosine_transform::<PixelBlock<VideoComponents>>(block_of_pixels.clone());
        image_cosine_coefficients.push(new_cosine_coefficients.clone());
    }
    image_cosine_coefficients
}


pub fn scale_quantize_fill_all_words(word_struct: WordStruct, cosine_coefficients_vec: Vec<CosineFloats>)-> Vec<[u8;4]> { //maybe put return type and do the rpeg at the end
    let mut all_codewords = Vec::new();
    for cosine_coefficient_set in cosine_coefficients_vec.iter(){
        //scale and quantize coefficients
        let (a,b,c,d,pb,pr) = wordfill::scale_and_quantize(cosine_coefficient_set.a.clone(), cosine_coefficient_set.b.clone(), cosine_coefficient_set.c.clone(), cosine_coefficient_set.d.clone(),cosine_coefficient_set.pb_avg.clone(), cosine_coefficient_set.pr_avg.clone(), word_struct.clone());
        //insert these values bitwise into a single 32 bit word
        let new_codeword = wordfill::fill_word(a,b,c,d,pb,pr, word_struct.clone()); //big endian
        all_codewords.push(new_codeword.clone());
    }
    all_codewords
}


pub fn decompress(filename: Option<String>) {
    
    //extract codewords
    let (all_codewords, width, height) = csc411_rpegio::read_in_rpeg_data(filename.as_deref()).unwrap();

    //set wordstruct for decompression
    let word_conversion_container = WordStruct::set_word_struct();

    //convert codewords to cosine coefficients in float form by descaling and dequantizing
    let vec_of_cosine_coefficients = retrieve_word_descale_float(all_codewords,word_conversion_container.clone());

    //convert the floating point cosine coefficients back to component video pixel blocks
    let vec_pixel_block_of_components = coefficients_to_components(vec_of_cosine_coefficients);

    //unpack pixel blocks into a single vector of video components
    let vec_sep_pixels = blockdata::pixel_vec(vec_pixel_block_of_components, width as usize, height as usize);

    //convert component video values back to floating point rgb
    let rgb_float_pixels = conversions::component_video_conversion_to_rgb_floats(vec_sep_pixels);
    //convert back to a vector of rgb pixels

    let rgb_pixels_in_int = conversions::rgb_floats_conversion_to_ints(rgb_float_pixels);
    
    //store vector of rgb pixels back into an rgbimage struct
    //ALLOCATE A   2D array of pixels of the given height and width (create_and_fill)
    let decompressed_image = RgbImage{
        pixels : rgb_pixels_in_int,
        width : width,
        height : height,
        denominator : 255, //magic number
    };
    //utilize csc411 image crate to write the rgbimage to stdout
    decompressed_image.write(None);
    
}


pub fn coefficients_to_components(cosine_coefficients_vec: Vec<CosineFloats>) -> Vec<PixelBlock<VideoComponents>>{
    //utilize the inverse_discrete_cosine_transform single call to create the video component blocks
    let mut vec_pixel_block_of_components : Vec<PixelBlock<VideoComponents>> = Vec::new();
    for cosine_coefficient_set in cosine_coefficients_vec {
        let new_block_of_pixels = wordfill::inverse_discrete_cosine_transform_to_components(cosine_coefficient_set);
        vec_pixel_block_of_components.push(new_block_of_pixels.clone());
    }
    vec_pixel_block_of_components
}

pub fn retrieve_word_descale_float(all_codewords: Vec<[u8;4]>,word_conversion_container: WordStruct) -> Vec<CosineFloats>{

    let mut image_cosine_coefficients: Vec<CosineFloats> = Vec::new();
    for codeword in all_codewords{
        //retrieve each bitmask within codeword
        let (a,b,c,d,pb,pr) = wordfill::split_word(codeword.clone(),word_conversion_container.clone());
        //convert values back to floats
        let (a,b,c,d,pb,pr) = wordfill::cosine_coefficients_to_float(a,b,c,d,pb,pr,word_conversion_container.clone());
        let new_cosine_coefficients = CosineFloats::create_cosine_coeff_struct(a,b,c,d,pb,pr);
        image_cosine_coefficients.push(new_cosine_coefficients);
    }
    image_cosine_coefficients
}





