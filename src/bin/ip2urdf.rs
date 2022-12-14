use std::fs;

use ip2urdf::*;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Directly read from arguments
    #[structopt(short)]
    direct: Option<String>,

    /// Input file
    #[structopt(short, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(short, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut text = String::new();
    if opt.direct.is_some() {
        text += opt.direct.unwrap().as_str();
    } else if opt.input.is_some() {
        text += fs::read_to_string(opt.input.clone().unwrap())
            .unwrap_or_else(|_| panic!("File {:?} Not found", opt.input.unwrap()))
            .as_str();
    } else {
        panic!(
            "You must either paste texts with the -d option or specify a file with the -i option."
        );
    }

    let p = parser::parse_properties(text);
    let output = urdf_link::generate(&p);

    if opt.output.is_some() {
        fs::write(opt.output.clone().unwrap(), output)
            .unwrap_or_else(|_| panic!("Writing into {:?} failed", opt.output.unwrap()));
    } else {
        println!("{}", output);
    }
}
