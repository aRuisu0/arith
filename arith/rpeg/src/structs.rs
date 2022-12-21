#![allow(unused)]


///Will be a struct which will simply hold floating point representations of 
/// red, green, and blue values which make up an RGB pixel in an image
#[derive(Clone)]
pub struct FloatRgb {
    pub red : f64,
    pub blue : f64,
    pub green: f64,
}

#[derive(Clone)]
pub struct VideoComponents {
    pub y: f64,
    pub pb: f64,
    pub pr: f64,
}

///Returns a VideoComponents struct which will hold the component videos value y. pb, pr
/// These values are calculated from the corresponding RgbFloats values of an image.
/// #Arguments:
/// * 'y' : luminance of a pixel (brightness)
/// * 'pb' : difference between the blue signals of a pixel and it's brightness
/// * 'pr' : difference between the red signals of a pixel and it's brightness

impl VideoComponents{
    pub fn create_video_components(y : f64, pb : f64, pr :f64) -> VideoComponents{
        let video_components = VideoComponents{
            y : y,
            pb : pb,
            pr : pr,
        };
        video_components
    }
}

#[derive(Clone)]
pub struct PixelBlock<T:Clone> {
    pub pixel_1: T,
    pub pixel_2: T,
    pub pixel_3: T,
    pub pixel_4: T,
}
///Returns a PixelBlock struct which will store any type of pixel but will be specifically
/// for storing component video struct values within
/// #Arguments:
/// * 'pixel_1': first pixel of the 2x2 block you're storing 
///  * 'pixel_2': second pixel of the 2x2 block you're storing
/// * 'pixel_3': third pixel of the 2x2 block you're storing
/// * 'pixel_4': fourth pixel of the 2x2 block you're storing 
impl <T:Clone> PixelBlock<T>{
pub fn create_pixel_block(pixel_1: T, pixel_2: T, pixel_3: T, pixel_4: T)->PixelBlock<T>{
    let pixel_block = PixelBlock::<T> {
        pixel_1 : pixel_1,
        pixel_2 : pixel_2,
        pixel_3 : pixel_3,
        pixel_4 : pixel_4,
    };
    pixel_block
}
}

//this will actually give you structure of word too
//just fill in when creating struct, then all you have to do is change one line of code
//a is umax
//b,c,d is smax *
//one line
#[derive(Clone)]
pub struct  WordStruct {
    pub a_width : u64,
    pub a_lsb : u64,
    pub b_c_d_quantize_range : f64,
    pub b_width : u64,
    pub b_lsb : u64,
    pub c_width : u64,
    pub c_lsb : u64,
    pub d_width : u64,
    pub d_lsb : u64,
    pub pb_avg_index_width : u64,
    pub pb_avg_index_lsb : u64,
    pub pr_avg_index_width : u64,
    pub pr_avg_index_lsb : u64,
}

///is a no arguments constructor signifying all the hardcoded values for
/// the compression/decompression outlined in the URI CSC 411 arith assignment.
/// These values compose the fundamental specifications on the range quantization
/// is limited to for b,c,d values and also for a,b,c,d by providing the width
/// values which are used further to scale and quantize the floating point cosine coefficients
/// to allow for each representation of their brightness or color difference of the image
/// fit within the specified bit field width from the position of the least siginificant bit
/// they will be placed into the 32-bit codeword.
///  all width values are the bit field lengths (the number of bits each value is allowed to exist within)
/// all lsb values are the least significant bit position where the corresponding bit field will start for 
/// each value
/// quantize range is the specific brightness range for b,c,d values which the values will be quantized from
//mor ecustomizable compression algorithm is possible here
//compression or decompression
impl WordStruct{
    pub fn set_word_struct() -> WordStruct{
        //customization as needed
        let current_compression_word_struct = WordStruct{
            a_width : 9,
            a_lsb : 23,
            b_c_d_quantize_range : 0.3,
            b_width : 5,
            b_lsb : 18,
            c_width : 5,
            c_lsb : 13,
            d_width : 5,
            d_lsb : 8,
            pb_avg_index_width : 4,
            pb_avg_index_lsb : 4,
            pr_avg_index_width : 4,
            pr_avg_index_lsb : 0,
        };
        current_compression_word_struct
    }
}


#[derive(Clone)]
pub struct CosineFloats { //we don't want to have this much info in one thing
    pub a : f64,
    pub b : f64,
    pub c : f64,
    pub d : f64,
    pub pb_avg: f32,
    pub pr_avg : f32,
}

///Returns a CosineCoerfficientsFloats struct instantiated with all a,b,c,d,pb.pr values passed through
///. Used in transition from one compression/decompression step to the next
/// * 'a' : average brightness of the image
/// * 'b' :  the degree to which the image gets brighter as we move from top to bottom 
/// * 'c' : the degree to which the image gets brighter as we move from left to right
/// * 'd' :  the degreeto which pixels on one diagonal are brighter than the pixels on the other diagonal
/// * 'pb_avg' :  the average color difference signals of B-Y 
/// * 'pr_avg' :  the average color difference signals of R-Y  

impl CosineFloats{
    pub fn create_cosine_coeff_struct(a: f64, b: f64, c: f64, d: f64, pb: f32, pr: f32)->CosineFloats{
        let new_cosine_coefficients = CosineFloats{
            a : a,
            b : b,
            c : c,
            d : d,
            pb_avg : pb,
            pr_avg : pr,
        };
        new_cosine_coefficients
    }
}
