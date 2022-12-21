mod blockdata;
mod structs;
mod conversions;
mod codec;
mod wordfill;

#[cfg(test)]
mod tests{
    use crate::conversions;
    use csc411_image::Rgb;
    use crate::structs::{FloatRgb,VideoComponents};
    use approx_eq::assert_approx_eq;

    #[test]
    fn check_floats_conversion(){
        let mut rgb_pixels : Vec<Rgb> = Vec::new();
        rgb_pixels.push(Rgb{red:112,green:100,blue:255});
        let rgb_pixels_float : Vec<FloatRgb> = conversions::rgb_ints_conversion_to_floats(rgb_pixels,255);
        eprintln!("{:?}", rgb_pixels_float[0].red);
    
        assert_eq!(rgb_pixels_float[0].red, 112.0/255.0);
    }

    #[test]
    fn check_round_trip_conversion_rgb_int_to_floats_to_int(){
        let mut rgb_pixels : Vec<Rgb> = Vec::new();
        rgb_pixels.push(Rgb{red:112,green:100,blue:255});
        let rgb_pixels_float : Vec<FloatRgb> = conversions::rgb_ints_conversion_to_floats(rgb_pixels,255);
        let rgb_pixels_int : Vec<Rgb> = conversions::rgb_floats_conversion_to_ints(rgb_pixels_float);
        eprintln!("wtf {}", rgb_pixels_int[0].red);
        assert_eq!(rgb_pixels_int[0].red, 112);

    }
    #[test]
    fn check_rgb_floats_to_video_components_conversion(){
        let mut rgb_pixels : Vec<Rgb> = Vec::new();
        rgb_pixels.push(Rgb{red:100,green:0,blue:100});
        let rgb_pixels_float : Vec<FloatRgb> = conversions::rgb_ints_conversion_to_floats(rgb_pixels,255/* 100*/);
        let rgb_vid_components : Vec<VideoComponents> = conversions::rgb_floats_conversion_to_component_video(rgb_pixels_float);
        assert_eq!(rgb_vid_components[0].y,0.413);
        //assert_approx_eq!(rgb_vid_components[0].y,0.16196,0.001);
    }
    
    #[test]
    fn check_round_trip_conversion_rgb_float_to_components_to_floats(){
        let mut rgb_pixels : Vec<Rgb> = Vec::new();
        rgb_pixels.push(Rgb{red:100,green:0,blue:100});
        let rgb_pixels_float : Vec<FloatRgb> = conversions::rgb_ints_conversion_to_floats(rgb_pixels,255/* 100*/);
        let rgb_pixels_float1 = rgb_pixels_float.clone();
        let rgb_vid_components : Vec<VideoComponents> = conversions::rgb_floats_conversion_to_component_video(rgb_pixels_float);
        let rgb_pixels_float_2 : Vec<FloatRgb> = conversions::component_video_conversion_to_rgb_floats(rgb_vid_components);
        
        assert_approx_eq!(rgb_pixels_float1[0].red, rgb_pixels_float_2[0].red,0.001);
    }
    
    #[test]
    fn check_round_trip_conversion_rgb_float_to_components_to_floats_to_ints(){
        let mut rgb_pixels : Vec<Rgb> = Vec::new();
        rgb_pixels.push(Rgb{red:100,green:0,blue:100});
        let mut rgb_pixels1 = rgb_pixels.clone();
        let rgb_pixels_float : Vec<FloatRgb> = conversions::rgb_ints_conversion_to_floats(rgb_pixels,255/* 100*/);
        let rgb_pixels_float1 = rgb_pixels_float.clone();
        let rgb_vid_components : Vec<VideoComponents> = conversions::rgb_floats_conversion_to_component_video(rgb_pixels_float);
        let rgb_pixels_float_2 : Vec<FloatRgb> = conversions::component_video_conversion_to_rgb_floats(rgb_vid_components);
        let rgb_pixels2 : Vec<Rgb> = conversions::rgb_floats_conversion_to_ints(rgb_pixels_float_2);
        eprintln!("{}",rgb_pixels2[0].red);
        assert_eq!(rgb_pixels1[0].red, rgb_pixels2[0].red); //fails miserably

    }

    


}
