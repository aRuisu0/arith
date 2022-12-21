mod codec;
mod wordfill;
mod blockdata;
mod conversions;
mod structs;
use clap::{Parser};

//literally don't know why I wouldn't just use clap for command line stuff
#[derive(Parser, Debug)]
#[clap(author ="Justin W. <justin_watkins@uri.edu>", version ="1.0", about ="just a guy struggling with Rust", long_about = None)]
struct Args {
    //file pathway
    path : Option<String>,
    //compress flag
    #[clap(short = 'c',takes_value = false)]
    compress: bool,
    //decompress flag
    #[clap(short = 'd', takes_value = false)]
    decompress: bool,
}
fn main() {
        let args = Args::parse();
        let compress = args.compress;
        let decompress = args.decompress;
        //if both flags are present or neither is then panic (this should cover all cases of incorrect input and incorrect
        //number of arguments along with clap's default)
        if !compress && !decompress || (compress && decompress) {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]");
        }else{
            //selection on compress or decompress, no default here as both can't be present
            if compress {
                codec::compress(args.path);
            }else{
                codec::decompress(args.path);
            }
        }
}
