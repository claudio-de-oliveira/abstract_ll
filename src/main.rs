mod abstract_tag;
mod test;
pub mod abstract_token;
pub mod abstract_parser;
pub mod abstract_rule;
pub mod scanner_environment;
pub mod concrete_scanner;
mod tag;

#[warn(non_snake_case)]
use abstract_tag::AbstractTAG;

fn main() {
    let vn = AbstractTAG::vn(1, "variable", 1);
    let vt = AbstractTAG::vt(1, "terminal", 1);
    let ac = AbstractTAG::action(1, "action", 1);
    println!("{}", vn);
    println!("{}", vt);
    println!("{}", ac);
    println!("{}", vn.to_string());
    println!("{}", vt.to_string());
    println!("{}", ac.to_string());
    println!("{:#?}", vn);
    println!("{:#?}", vt);
    println!("{:#?}", ac);
}