use std::{env, fs};
use std::io::stdin;
use std::path::Path;
use std::str::FromStr;

use colors_transform::Rgb;
use regex::{Captures, Regex, Replacer};

const REGEX: &str = r"rgba\((?<red> ?\d+(\.\d+)?),(?<green> ?\d+(\.\d+)?),(?<blue> ?\d+(\.\d+)?),(?<alpha> ?\d+(\.\d+)?)\)";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String;
    let file: String = if args.len() == 2 {
        let path = Path::new(args.get(1).unwrap());
        file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        fs::read_to_string(path).expect("Invalid file")
    } else {
        println!("Type the file location: ");
        let mut ans = String::new();
        stdin().read_line(&mut ans).expect("Invalid input");
        let path = Path::new(ans.trim());
        file_name = path.file_name().expect("Invalid input").to_str().unwrap().to_owned();
        fs::read_to_string(path).expect("Invalid file")
    };
    let new_file = find_and_replace_rgba(&file);
    let new_path = "fixed_".to_owned() + &file_name;
    fs::write(&new_path, new_file).expect("Error while writing new file");
    println!("Success, written as {new_path}, press any button to close this window...");
    stdin().read_line(&mut String::new()).expect("");
}

fn find_and_replace_rgba(file: &str) -> String {
    let regex = Regex::from_str(REGEX).unwrap();
    regex.replace_all(file, SvgRgbaToHexReplacer).to_string()
}

struct SvgRgbaToHexReplacer;
impl SvgRgbaToHexReplacer {
    fn convert_to_hex(red: f32, green: f32, blue: f32, alpha: f32) -> String {
        Rgb::from(red, green, blue).to_css_hex_string() + format!("\" fill-opacity=\"{alpha}").as_str()
    }
}
impl Replacer for SvgRgbaToHexReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let red: f32 = caps["red"].trim().parse().expect("Error while parsing color");
        let green: f32 = caps["green"].trim().parse().expect("Error while parsing color");
        let blue: f32 = caps["blue"].trim().parse().expect("Error while parsing color");
        let alpha: f32 = caps["alpha"].trim().parse().expect("Error while parsing color");
        dst.push_str(&SvgRgbaToHexReplacer::convert_to_hex(red, green, blue, alpha))
    }
}
