mod tag;
mod test;
pub mod token;
pub mod parser;
pub mod rule;
pub mod scanner_environment;
pub mod semantic_environment;
mod scanner;
mod grammar;
mod variable;
mod literal;


use std::collections::HashMap;

use scanner::{CeScanner, Scanner};
use token::Token;
use variable::Variable;
use literal::Literal;

fn main() {
    // let vn = Tag::vn(1, "variable", 1);
    // let vt = Tag::vt(1, "terminal", 1);
    // let ac = Tag::action(1, "action", 1);
    // println!("{:?}", *ce_tag::VT_ALT);
    // println!("{:?}", *ce_tag::VT_ANSWERED);
    // println!("{:?}", *ce_token::ALT);
    // println!("{:?}", *ce_token::ANSWERED);
    // let mut env = scanner_environment::ScannerEnv::new(vec!["Claudio de answered Oliveira"]);
    // let scanner: CeScanner = CeScanner::new();
    // let mut symbol_table = HashMap::<String, Box<dyn Token>>::new();


    // let mut variables = HashMap::<String, Variable>::new();
    // variables.insert(
    //     "de".to_string(), 
    //     Variable {
    //         input_method: None, 
    //         name: None, 
    //         data_type: None, 
    //         field_only: None, 
    //         occurs_order: None, 
    //         prompt: None, 
    //         selections: None, 
    //         repeat: None, 
    //         repeats:2, 
    //         definition: None, 
    //         logic: None, 
    //         default_format: None, 
    //         original_format: None, 
    //         depth: None, 
    //         relevant: false, 
    //         visible: None, 
    //         value: None, 
    //     }
    // );

    // let token = scanner.next_token(&mut env, &symbol_table, &variables);
    // let token = scanner.next_token(&mut env, &symbol_table, &variables);
    // let token = scanner.next_token(&mut env, &symbol_table, &variables);
    // println!("{:#?}", scanner);

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