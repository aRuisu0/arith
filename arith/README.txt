– Identify you and your programming partner by name

Justin Watkins and Rojeen Sindi

– Acknowledge help you may have received from or collaborative work
you may have undertaken with others

Most work was done solely on our own, with only one or two quick pop ins to Connor's TA discord hours just to ensure we were in the right direction. 

– Identify what has been correctly implemented and what has not

Compression and decompression both work properly but the degree of difference in utilizing the ppmdiff module wasn't ascertained.  

There are some limitations on our bitpack module however as the extensive testing that was supposed to be performed with the 
bitpack testing lab wasn't able to be completed. But there is still a somewhat extensive test suite but nothing near exhaustive testing requirements and we were not able 
to ensure all proper algebraic laws for bitpack. 64 bit shift does work when utilizing the shl and shr commands however, 
due to retraints in time a full implementation of them wasn't properly conducted.

Thoughtful quantization back to Rgb 0 to 255 values and properly dividing by the denominator of the RgbImage wasn't conducted either. Only a simple calmp procedure was utilized to try and 
abate the spec details hinting towards issues in this conversion being done continually. 

Too much reliance on for loops with regular .iter() uses as this rough implementation led to many .clone() procedures and aversion for utilizing array2. Optimization, referencing, 
and utilization of Rust language characteristics were lackluster at best. 

– Explain the architecture of your solution

The archtitecture our program follows is mainly around the transferring of converted vectors of data from one stage of the compression or decompression to the next. Our modules 
are centered around this approach and while some rethinking was done during the actual implementation it mainly follows this central idea. There are functions which are 
implemented without resultant vectors produced but in most cases conversions between vectors of structures meant specifically to hold each stage of the compression or 
decompression are utilized. The following modules are included:

	-The conversions module which translates vectors of individual pixels to the next stage of compression or decompression (vectors of rgb pixels to vectors of rgb float pixels 
	and then RgbFloat pixels to component video pixel parameters and the reverse of each of those conversions)
	
	-The blockdata module which implemented vectors of blocks to vectors of pixels conversion and vice versa. This is utilized for grouping together the component video values into 2x2 blocks
	and taking apart those 2x2 blocks. This was one of the better implemented modules as a proper utilization of array2 was used and a strong generality with very little dependence on the
	rest of the modules occured. 

	-The wordfill module which implemented the discrete cosine transform functions and the scaling and quantization of those cosine coefficients. The idea was because these coefficients
	and their direct transformations were going to make up the codeword, they would be included directly alongside the functions for filling the codewords. All inverse transformations and 
	conversions were done here for decompression also. 
		
		-If more time was allowed we would probably further breakdown the module here into a cosinetransform one also, separate from the wordfill
	
	-The bitpack module which was responsible for bitwise and integer operations to allow for bitmasking of the compressed data which can be later decompressed. 
		
		-In hindsight some functions (like umax and smax) could've probably gone into the wordfill module, especially if the cosinetransform module was properly separated. But, original 
		implementations of bitpacking utilized them also. 

	-The structs module which defines all the structures to hold the converted values from each stage of the compression and decompression. There includes constructors for most of these structs
	The structs inlcuded are the RgbFloats which hold the floating point rgb pixels, the VideoComponents struct which hold the Y/Pb/Pr values, the PixelBlock which holds 4 pixels at a time 
	(mainly used for component video pixels blocks but generalizable), the CosineCoefficientsFloats which hold the direct conversion from component video to the cosine transform values or the 
	conversion from codewords quantized values. Finally the WordStruct struct is also present in this module which is described below. 

In general, another guiding principle the entire function is centered around is the idea of needing to change as little code as possible 
to allow for a different compression and decompression scheme to be utilized. This is done by having a WordStruct structure which will specify how the codeword is formatted 
and ranges for quantization can be specified. A custom constructor wasn't implemented but you can simply change the values in the no argument constructor set_word_struc to allow 
for the different compression/decompression scheme within the confines of the u64 limit bitpacking allows for. 

– Say approximately how many hours you have spent analyzing the problems posed in the assignment

30 hours minimum.

– Say approximately how many hours you have spent solving the problems after your analysis

At least another 30 hours. It's been a very rigorous assignment and bitpacking really did require extensive testing and thought.

 

