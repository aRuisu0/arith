pub mod bitpack;
//pub use crate::bitpack;
//cargo test -- --nocapture
#[cfg(test)]
mod tests{

    use crate::bitpack;

    #[test]
    fn check_fitts(){
        let num = 3;
        let width = 5;

        let fitss = bitpack::fitss(num, width);
        //println!("{:?}", fitss);
        assert_eq!(fitss, true);
    }

    // #[test]
    // fn check_word_width(){
    //     let word = 1_i64;
    //     assert_eq!(bitpack::get_word_width(word as u64),64);
    // }
    #[test]
    fn check_fitss_fail(){
        let num = 30;
        let width = 5;

        let fitss = bitpack::fitss(num, width);
        //println!("{:?}", fitss);
        assert_eq!(fitss, false);
    }

    #[test]
    fn check_fitsu(){
        let num = 3;
        let width = 5;

        let fitsu = bitpack::fitsu(num, width);
        //println!("{:?}", fitsu);
        assert_eq!(fitsu, true);
    }

    #[test]
    fn check_fitsu_fail(){
        let num = 500;
        let width = 5;

        let fitsu = bitpack::fitsu(num, width);
        //println!("{:?}", fitsu);
        assert_eq!(fitsu, false);
    }

    #[test]
    fn check_smax(){
        let bits = 3;

        let smax = bitpack::smax(bits);
        //println!("{:?}", smax);
        assert_eq!(smax, 3);
    }

    #[test]
        fn check_smin(){
            let bits = 3;
            let smin = bitpack::smin(bits);
            assert_eq!(smin, -4)
        }
    

    #[test]
    fn check_umax(){
        let bits = 3;

        let umax = bitpack::umax(bits);
        //println!("{:?}", umax);
        assert_eq!(umax, 7);
    }

    #[test]
    fn check_getu(){
        let getu = bitpack::getu(0x3f4,6,2);
        //println!("{:?}", getu);
        assert_eq!(getu,61);
    }

    #[test]
    fn check_gets(){
        let gets = bitpack::gets(0x3f4,6,2);
        println!("gets result {:04b}", gets);
        assert_eq!(gets,-3);
    }

    #[test]
    fn test_64(){
        let gets = bitpack::news(1_u64 -1, 64, 0, 1);
        //assert_eq!()
    }

    #[test]
    fn round_trip_new_get_s(){
        let word = 0b0111110001010100;
        let w = 6;
        let lsb = 2;
        let val = -7;
        assert_eq!(bitpack::gets(bitpack::news(word, w, lsb, val).unwrap(), w, lsb), val);
        
    }

    #[test]
    fn round_trip_new_get_u(){
        let word = 0b0111110001010110;
        let w = 4;
        let lsb = 5;
        let val = 11;
        println!("value we have at first is {:04b}",11);
        let newu = bitpack::newu(word, w, lsb, val).unwrap();
        println!("original word {:04b}",word);
        println!(" test on newu i guess {:04b}",newu);
        let getu = bitpack::getu(newu, w,lsb);
        println!("lets see {:04b}", getu);
        //println!()
        assert_eq!(bitpack::getu(bitpack::newu(word, w, lsb, val).unwrap(), w, lsb), val);
    }

    #[test]
    fn check_newu(){
        let word = 0b101001;
        let width = 3;
        let lsb = 2;
        let value = 0b111; 
        

        let newu = bitpack::newu(word,width,lsb, value);
        println!("NEWU TEST DEFINITELY{:b}", newu.unwrap());
        assert_eq!(newu.unwrap(), 61);
    }

    #[test]
    fn check_news(){
        let word = 0x0000;
        let width = 6;
        let lsb = 1;
        let value = 0b10111 as i64;//-9
        //adds one 0 to the end of the number inserted x2
        //should be 101110 which should be -18 but we return a u64 
        //so it will be 46


        let news = bitpack::news(word,width,lsb, value);
        //println!("{:?}", news);
        assert_eq!(news.unwrap(), 46);
    }
}
