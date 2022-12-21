#![allow(unused)]
use csc411_arith;
use std::convert::TryInto;
pub use crate::structs::{PixelBlock,CosineFloats,VideoComponents,WordStruct};
use bitpack::bitpack::{getu,gets,umax,smax,news,newu};

pub fn discrete_cosine_transform<T:std::clone::Clone>(vid_block: PixelBlock::<VideoComponents>) -> CosineFloats {
    let a = (vid_block.pixel_4.y + vid_block.pixel_3.y + vid_block.pixel_2.y + vid_block.pixel_1.y) as f64 / 4.0;
    let b = (vid_block.pixel_4.y + vid_block.pixel_3.y - vid_block.pixel_2.y - vid_block.pixel_1.y) as f64 / 4.0;
    let c = (vid_block.pixel_4.y - vid_block.pixel_3.y + vid_block.pixel_2.y - vid_block.pixel_1.y) as f64 / 4.0;
    let d = (vid_block.pixel_4.y - vid_block.pixel_3.y - vid_block.pixel_2.y + vid_block.pixel_1.y) as f64 / 4.0;
    let pb_avg = (vid_block.pixel_1.pb + vid_block.pixel_2.pb + vid_block.pixel_3.pb + vid_block.pixel_4.pb) as f32 / 4.0;
    let pr_avg = (vid_block.pixel_1.pr + vid_block.pixel_2.pr + vid_block.pixel_3.pr + vid_block.pixel_4.pr) as f32 / 4.0;

    CosineFloats { a, b, c, d, pb_avg, pr_avg }
}



pub fn scale_and_quantize(a : f64, b : f64, c: f64, d: f64, pb: f32, pr: f32, _word_struct: WordStruct)->(u64,i64,i64,i64,u64,u64){
    let a = (a * umax(_word_struct.a_width) as f64).round() as u64;
    let b = (b.clamp(-1.0 * _word_struct.b_c_d_quantize_range, _word_struct.b_c_d_quantize_range) 
    * (smax(_word_struct.b_width) as f64 /_word_struct.b_c_d_quantize_range)).round() as i64;
    let c = (c.clamp(-1.0 * _word_struct.b_c_d_quantize_range, _word_struct.b_c_d_quantize_range) 
    * (smax(_word_struct.c_width) as f64 /_word_struct.b_c_d_quantize_range)).round() as i64;
    let d = (d.clamp(-1.0 * _word_struct.b_c_d_quantize_range, _word_struct.b_c_d_quantize_range) 
    * (smax(_word_struct.d_width) as f64 /_word_struct.b_c_d_quantize_range)).round() as i64;
    let pb = csc411_arith::index_of_chroma(pb) as u64;
    let pr = csc411_arith::index_of_chroma(pr) as u64;
    return (a,b,c,d,pb,pr);
}


pub fn fill_word( a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64, word_struct: WordStruct) -> [u8;4]{
    let mut word = 0_u64;
    word = newu(word, word_struct.a_width, word_struct.a_lsb, a).unwrap();
    word = news(word, word_struct.b_width, word_struct.b_lsb, b).unwrap();
    word = news(word, word_struct.c_width, word_struct.c_lsb, c).unwrap();
    word = news(word, word_struct.d_width, word_struct.d_lsb, d).unwrap();
    word = newu(word, word_struct.pb_avg_index_width, word_struct.pb_avg_index_lsb, pb).unwrap();
    word = newu(word, word_struct.pr_avg_index_width, word_struct.pr_avg_index_lsb, pr).unwrap();
    let new_codeword = (word as u32).to_be_bytes();
    new_codeword
}



pub fn cosine_coefficients_to_float(a: u64, b: i64, c: i64, d:i64, pb: u64, pr: u64, word_conversion_container: WordStruct) -> (f64, f64, f64, f64, f32, f32) {
    let a = a as f64 / umax(word_conversion_container.a_width) as f64;
    let b = (b as f64 / ((smax(word_conversion_container.b_width)) as f64/ word_conversion_container.b_c_d_quantize_range)).clamp(-1.0 * word_conversion_container.b_c_d_quantize_range, word_conversion_container.b_c_d_quantize_range);
    let c = (c as f64 / ((smax(word_conversion_container.c_width)) as f64/ word_conversion_container.b_c_d_quantize_range)).clamp(-1.0 * word_conversion_container.b_c_d_quantize_range, word_conversion_container.b_c_d_quantize_range);
    let d = (d as f64 / ((smax(word_conversion_container.d_width)) as f64/ word_conversion_container.b_c_d_quantize_range)).clamp(-1.0 * word_conversion_container.b_c_d_quantize_range, word_conversion_container.b_c_d_quantize_range);
    let pb = csc411_arith::chroma_of_index(pb as usize);
    let pr = csc411_arith::chroma_of_index(pr as usize);
    return (a,b,c,d,pb,pr);
}


pub fn split_word(codeword: [u8; 4], word_conversion_container: WordStruct) -> (u64, i64, i64, i64, u64, u64) {
    let word = u32::from_be_bytes(codeword.try_into().unwrap());
    let a = getu(word as u64, word_conversion_container.a_width, word_conversion_container.a_lsb);
    let b = gets(word as u64, word_conversion_container.b_width, word_conversion_container.b_lsb);
    let c = gets(word as u64, word_conversion_container.c_width, word_conversion_container.c_lsb);
    let d = gets(word as u64, word_conversion_container.d_width, word_conversion_container.d_lsb);
    let pb = getu(word as u64, word_conversion_container.pb_avg_index_width, word_conversion_container.pb_avg_index_lsb);
    let pr = getu(word as u64, word_conversion_container.pr_avg_index_width, word_conversion_container.pr_avg_index_lsb); 
    return (a,b,c,d,pb,pr);
}

pub fn inverse_discrete_cosine_transform_to_components(cosine_coefficients : CosineFloats) -> PixelBlock<VideoComponents>{

    let y1 = cosine_coefficients.a - cosine_coefficients.b - cosine_coefficients.c + cosine_coefficients.d;
    let y2 = cosine_coefficients.a - cosine_coefficients.b + cosine_coefficients.c - cosine_coefficients.d;
    let y3 = cosine_coefficients.a + cosine_coefficients.b - cosine_coefficients.c - cosine_coefficients.d;
    let y4 = cosine_coefficients.a + cosine_coefficients.b + cosine_coefficients.c + cosine_coefficients.d;
    

    let pixel_1 = VideoComponents::create_video_components(y1,cosine_coefficients.pb_avg.into(), cosine_coefficients.pr_avg.into());
    let pixel_2 = VideoComponents::create_video_components(y2,cosine_coefficients.pb_avg.into(),cosine_coefficients.pr_avg.into());
    let pixel_3 = VideoComponents::create_video_components(y3,cosine_coefficients.pb_avg.into(),cosine_coefficients.pr_avg.into());
    let pixel_4 = VideoComponents::create_video_components(y4,cosine_coefficients.pb_avg.into(), cosine_coefficients.pr_avg.into());

    let pixel_block = PixelBlock::create_pixel_block(pixel_1,pixel_2,pixel_3,pixel_4);
    pixel_block
}
