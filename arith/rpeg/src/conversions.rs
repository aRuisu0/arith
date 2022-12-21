#![allow(unused)]
use csc411_image::Rgb;
use crate::structs::{FloatRgb,VideoComponents};

//probably could reconfigure code to make it so I just take each pixel and transform it from int rgb to component video in same function
//adding more time and moving entire vectors each time, same with the decompression vice versa


///Returns a vector of float rgb values which is produced by simply dividng each rgb value of each
/// pixel of the original image by 255. This will produce normalized values to be used further in compression
/// by allowing a smaller range of values to be represented and thus a lesser amount of bits needed to represent those values.
/// #Arguments:
/// * 'pixels' : vector of rgb pixels which are each in integer form and will each be converted to rgb float
/// * 'denominator' : the largest r g or b value represented in the image ---depreciated and will be used when proper quantization from floats to RGB is figured out
//compression
pub fn rgb_ints_conversion_to_floats(pixels: Vec<Rgb>, denominator: u16)->Vec<FloatRgb> {
    let mut pixels_in_float: Vec<FloatRgb> = Vec::new();
    //this function is fine
    for pixel in pixels.iter(){
        //set the FloatRgb struct with float values calculated with whatever original image denominator is
        let new_float_pixel = FloatRgb {
            // was dividing by denominator before
            red : pixel.red as f64 / 255  /*denominator*/ as f64, 
            blue: pixel.blue as f64 / 255  /*denominator*/ as f64,
            green: pixel.green as f64 / 255  /*denominator*/ as f64,
        };
        pixels_in_float.push(new_float_pixel);
    };
    pixels_in_float
}

///Returns a vector of Rgb pixels represented back in integer form. Is done with the intention
/// of mitigating quantization error by not allowing RGB color values to go negative by clamping 
/// within the normalized range of 0 to 1 and multiplying by 255 and rounding to quantize the data 
/// to specific colors within the 0 to 255 range
/// #Arguments:
/// *'float_pixels' : vector of rgb pixels represented in floating point form for their values (normalized values used for compression)

//decompression
pub fn rgb_floats_conversion_to_ints(float_pixels: Vec<FloatRgb>)-> Vec<Rgb> {
    let mut pixels_in_int: Vec<Rgb> = Vec::new();
    //have to figure how to limit the 255  chaarcteristics here from the float values
    for float_pixel in float_pixels.iter(){
        let new_int_pixel = Rgb{
            //recommended denominator in specs is magic number 255 on decompression
            red: (float_pixel.red.clamp(0.0,1.0) * 255.0).round() as u16,
            blue: (float_pixel.blue.clamp(0.0,1.0) *255.0).round() as u16,
            green: (float_pixel.green.clamp(0.0,1.0) *255.0).round() as u16,
        };
        pixels_in_int.push(new_int_pixel);
    };
    pixels_in_int
}

///Returns a vector of float rgb pixels by converting back to a close approximation of the orignal rgb float pixel
/// with a nonlinear conversion from the component video values passed through to the function. The y value is luminance (brightness),
/// the pb and pr values are color difference signals (chromacity) where pb decribes the  B-Y relationship and pr describes the R-Y relationship.
/// #Arguments:
/// * 'video_components_pixels': vector of video component structs meant to hold the y,pb,pr data utilized in this compression/decompression
//decompression
pub fn component_video_conversion_to_rgb_floats(video_components_pixels: Vec<VideoComponents>) -> Vec<FloatRgb>{
    let mut rgb_float_pixels:Vec<FloatRgb> = Vec::new();
    for video_component_pixel in video_components_pixels.iter(){
        let new_rgb_float = FloatRgb{
            red: 1.0 * video_component_pixel.y + 1.402 * video_component_pixel.pr,
            green: 1.0 * video_component_pixel.y - 0.344136 * video_component_pixel.pb - 0.714136 * video_component_pixel.pr,
            blue: 1.0 * video_component_pixel.y + 1.772 * video_component_pixel.pb, 
        } ;
        rgb_float_pixels.push(new_rgb_float);
        
    }
    rgb_float_pixels
}

//Returns a vector of component video values. The y value is luminance (brightness),
/// the pb and pr values are color difference signals (chromacity) where pb decribes the B-Y relationship and pr describes the R-Y relationship.
/// These are all calculated by the red,blue,and green rgb float values. These values are useful because they
/// help set the stage to more succinctly represent the difference in brightness which is much more important 
/// to human vision compared to the chromacity of an image. 
/// #Arguments: 
/// * 'floating_rgb_pixels' : a vector of floating rgb pixels which their red, green, and blue values will be used
/// in the nonlinear conversion to component video values
//compression
pub fn rgb_floats_conversion_to_component_video(floating_rgb_pixels: Vec<FloatRgb>) -> Vec<VideoComponents> {
    let mut video_components_pixels: Vec<VideoComponents> = Vec::new();
    for floating_rgb_pixel in floating_rgb_pixels.iter(){
        let new_video_components_pixel = VideoComponents{
            y: 0.299 * floating_rgb_pixel.red + 0.587 * floating_rgb_pixel.green + 0.114 * floating_rgb_pixel.blue,
            pb: -0.168736 * floating_rgb_pixel.red - 0.331264 * floating_rgb_pixel.green + 0.5 * floating_rgb_pixel.blue,
            pr: 0.5 * floating_rgb_pixel.red - 0.418688 * floating_rgb_pixel.green - 0.081312 * floating_rgb_pixel.blue,
        };
        video_components_pixels.push(new_video_components_pixel);
    };
    video_components_pixels
}