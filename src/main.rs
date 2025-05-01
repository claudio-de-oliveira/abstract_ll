mod tag;
mod test;
pub mod token;
pub mod abstract_parser;
pub mod rule;
pub mod scanner_environment;
pub mod concrete_scanner;
mod ce_tag;
mod ce_token;
mod scanner;

use scanner::{CeScanner, Scanner};

#[warn(non_snake_case)]
use tag::Tag;

fn main() {
    // let vn = Tag::vn(1, "variable", 1);
    // let vt = Tag::vt(1, "terminal", 1);
    // let ac = Tag::action(1, "action", 1);
    // println!("{:?}", *ce_tag::VT_ALT);
    // println!("{:?}", *ce_tag::VT_ANSWERED);
    // println!("{:?}", *ce_token::ALT);
    // println!("{:?}", *ce_token::ANSWERED);
    // let mut env = scanner_environment::ScannerEnv::new(vec!["Claudio de Oliveira"]);
    let scanner = CeScanner::new();
    println!("{:#?}", scanner);

    // println!("{}", vn);
    // println!("{}", vt);
    // println!("{}", ac);
    // println!("{}", vn.to_string());
    // println!("{}", vt.to_string());
    // println!("{}", ac.to_string());
    // println!("{:#?}", vn);
    // println!("{:#?}", vt);
    // println!("{:#?}", ac);
}