/// Returns true iff the signed value `n` fits into `width` signed bits.
///
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    n == ((n << (64 - width)) as i64) >> (64 - width)
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
///
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    n == (n << (64 - width)) >> (64 - width)
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
	// shift relevant bits to be left aligned
	// shift to be right aligned (fills left side with 1's if negative, 0's if positive)
	// return (0x value if positive) (1x value if negative)
    ((word << (64 - width - lsb)) as i64) >> (64 - width)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// /// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
	// shift relevant bits to be left aligned (deletes bits we don't want on the left)
	// shift relevant bits to be right aligned (deletes bits to the right)

	// return 0x value
    (word << (64 - width - lsb)) >> (64 - width)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    //word_size = word.count_zeros() + word.count_ones();
    //let word_width = (word.count_ones() + word.count_zeros()) as u64;
    let word_width = word.count_zeros() + word.count_ones();
    //eprintln!("WORD USING {:b}",word);
    // rust says this is useless width < 0 &&
    if width > word_width.into()  || width + lsb >  word_width.into() {
        panic!("You cannot retrieve that value from the bit field and/or word of that length");
    }else{
        if fitsu(value,width){
            let left = (word >> (lsb+width)) <<(lsb+width); // clears everything on right
            let right = shr(shl(word,word_width as u64 - lsb),word_width as u64 - lsb); //clears everything on left
            let val = value <<lsb; //moves value to position of lsb with trailing 0s to the right
            return Some(left | right as u64 | val);
        } else {
            return None;
        }
    }  
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    let word_width = word.count_zeros() + word.count_ones();
    // rust says this is useless width < 0 &&
    if width > word_width.into()  || width + lsb >  word_width.into() {
        panic!("You cannot retrieve that value from the bit field and/or word of that length");
    }else{
        if fitss(value,width){
            //let left = (word >> (lsb+width)) << (lsb+width);
            let left = shl(shr(word,lsb+width),lsb+width);
            //eprintln!("left {:b}",left);
            //let right = (word << (64-lsb-width)) >> (64-width);
            //let right = ((word) << (word_width as u64-lsb )) >> (word_width as u64-(lsb));
            let right = shr(shl(word,word_width as u64-lsb),word_width as u64-(lsb));

            let mask = ((1_i64 << word_width) - 1) << lsb;

            //eprintln!("right {:b}",right);
            let val = value << lsb;

            let u_val = (val & mask) as u64;

            //eprintln!("val {:b}",val);
            return Some(left | right | u_val);
        } else{
            return None;
        }
    }
    //return newu(word, width, lsb, value);
}

//Beginning attempts to mask shift left and shift right operators 
// and to try and include a proper reaction to shifting by word length
// does not include the difference in shift right for signed and shift right
// for unsigned and the difference between the logical and arthimetic versions
// respectively where one would be shr0 where it's for unsigned integers to only have 
// 0s placed to the left when shifting right and shrs operator where the signed bit value
// would be perserved when shifting right
// would just require proper casting of the word
// 
// a reasonable approach for shifting by 64 or more would be to set the result to be all zeroes/all ones?
#[inline]
pub fn shl(word: u64, shift: u64)->u64{
        return word << shift;
}
#[inline]
pub fn shr(word:u64,shift:u64)->u64{
        return word >> shift;
}

///Returns the largest possible unsigned integer value you can achieve with a set amount of bits 
/// by simply taking 1 over to the right bits+1 places over and then subtrcating 1 to get all ones
/// however, if the field of bits is destined as larger than 64, limitations on the function would
/// yield only the largest number for 64 bits or u64::MAX or 1 64 times
/// #Arguments:
/// * 'bits' : how many bits you're using to make a number (bit width)

#[inline]
pub fn umax (bits: u64)-> u64 {
    if bits >= 64{
        return u64::MAX;
    }else{
        return (1_u64<<bits)-1 as u64; //this should simply arithmetic right shift the signed value over the number of 
    }
        //specified bits while keeping the signed bit (the one) the entire time of shifting keeping all 
    //positions filled with one to get the largest unsigned value possible when casted back to u64
}

///Returns the largest possible signed integer value you can achieve with a set amount of bits 
/// by simply taking  1 over to the right bits+1 places over and dividing by 2 to eliminate a 
/// power of two placeholder as signed bits are split between having half their values positive 
/// and half negative but the number 0 is included also so you have to subtract by 1
/// however, if the field of bits is destined as larger than 64, limitations on the function would
/// yield only the largest number for 64 bits or u64::MAX
/// #Arguments:
/// * 'bits' : how many bits you're using to make a number (bit width)
/// 
#[inline]
pub fn smax (bits: u64) -> i64{
    //eprintln!("{}",(1_i64<<bits)/2-1);
    if bits >= 64{
        return i64::MAX;
    }else {
        return (1_i64<<bits)/2-1;
    } //takes 10000000, divides by 2 and subtracts 1
    //eliminates one power of 2 position and subtracts 1 to get 01111111 giving positive max value for a signed number
    
}

///Returns the smallest possible signed integer value you cna achieve 
/// #Arguments:
/// * 'bits' :  how many bits you're using to make a number (bit width)
#[inline]
pub fn smin (bits: u64) -> i64 {
    //eprintln!("{}",!(1_i64<<bits)/2);
    if bits >= 64{
        return i64::MIN;
    }else{
        return !(1_i64<<bits)/2;
    }
     //takes 1000000 and just eliminates one power of two position giving negative minimum value for a signed number
}

#[cfg(test)]
mod tests {
	use crate::bitpack::*;
	// 8 bit width tests
	
	// fitss
	// n bounded by {-128, 127}
	#[test]
	fn fitss_lower_bound() {
		assert!(fitss(-128, 8));
		assert!(!fitss(-129, 8));
	}
	// n bounded by {0, 255}
	#[test]
	fn fitss_upper_bound() {
		assert!(fitss(127, 8));
		assert!(!fitss(128, 8));
	}
	
	//fitsu
	// n bounded by {0, 255}
	#[test]
	fn fitsu_lower_wound() {
		assert!(fitsu(0, 8));
	}
	#[test]
	fn fitsu_upper_bound() {
		assert!(fitsu(255, 8));
		assert!(!fitsu(256, 8));
	}
	
	// build_word and get_word are general tests
	#[test]
	fn build_word() {
		// original word
		// 0 x 32 ... 1 x 32
		// numbers to input (left to right)
		// -3 4 1 15 2 6 -8 -1
		// result
		// 0xD41F286
		let mut word: u64 = !0_u32 as u64;
		// news and newu should delete the 1's in their way
		word = news(word, 4, 28, -3).unwrap();
		word = newu(word, 4, 24, 4 ).unwrap();
		word = newu(word, 4, 20, 1 ).unwrap();
		word = newu(word, 4, 16, 15).unwrap();
		word = newu(word, 4, 12, 2 ).unwrap();
		word = newu(word, 4, 8, 6  ).unwrap();
		word = news(word, 4, 4, -8 ).unwrap();
		word = news(word, 4, 0, -1 ).unwrap();
		
		assert_eq!(word, 0xD41F268F);
	}
	
	#[test]
	fn get_word() {
		let word: u64 = 0xD41F268F;
		assert_eq!(gets(word, 4, 28), -3);
		assert_eq!(getu(word, 4, 24),  4);
		assert_eq!(getu(word, 4, 20),  1);
		assert_eq!(getu(word, 4, 16), 15);
		assert_eq!(getu(word, 4, 12),  2);
		assert_eq!(getu(word, 4,  8),  6);
		assert_eq!(gets(word, 4,  4), -8);
		assert_eq!(gets(word, 4,  0), -1);
	}
	
	// follwing tests check if user gives maximum width of 64
	#[test]
	fn newu_bounds() {
		let mut word: u64 = 0;
		word = newu(word, 64, 0, !0_u64).unwrap();
		assert_eq!(word, !0_u64);
	}
	#[test]
	fn news_bounds() {
		let mut word: u64 = 0;
		word = news(word, 64, 0, -1_i64).unwrap();
		assert_eq!(word, !0_u64);
	}
	
	#[test]
	fn getu_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(getu(word, 64, 0), !0_u64);
	}
	#[test]
	fn gets_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(gets(word, 64, 0), -1_i64);
	}
	
}
