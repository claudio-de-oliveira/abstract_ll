// use crate::{grammar::Grammar, scanner_environment::ScannerEnv, token::Token};

// pub trait Scanner {
//     fn next_token(&self, env : &mut ScannerEnv) -> impl Token;
//     fn initialize(&mut self);
// }

// pub trait Semantic {
//     fn execute(&self, action : &impl Token);
//     fn initialize(&mut self);
// }

// #[allow(dead_code)]
// pub struct Parser<S : Scanner, T: Semantic> {
//     grammar: Grammar,
//     scanner: S, //&'a mut Box<dyn Scanner>,
//     semantic: T,
// }

// impl<S : Scanner, T: Semantic> Parser<S, T> {
//     pub fn new(
//         grammar: Grammar,
//         scanner: S,
//         semantic: T,
//     ) -> Self {
//         Parser {
//             grammar,
//             scanner,
//             semantic,
//         }
//     }

//     // Push the right-hand side of the production rule onto the stack in reverse order
//     fn push_rhs(&self, stk: &mut Vec<T>, p: usize) where T: Token {
//         for i in (0..self.grammar.get_rhs(p).len()).rev() {
//             stk.push(T::from_tag(self.grammar.get_rhs(p)[i]));
//         }
//     }


//     pub fn parse(&mut self, vtext: Vec<&str>) -> bool {

//         let stk = &mut Vec::<Token>::new();

//         let scanner_env = &mut ScannerEnv::new(vtext);

//         self.scanner.initialize();
//         self.semantic.initialize();

//         self.push_rhs(stk, 0);

//         let mut token: Token = self.scanner.next_token(scanner_env);

//         loop {
//             let a = stk.pop().unwrap_or(Token::from_tag(self.grammar.get_end_mark()));

//             if a.get_tag().is_terminal() {

//                 if a.get_tag().to_int() == self.grammar.get_end_mark().to_int() {
//                     // END
//                     break;
//                 } else if a.get_tag().to_int() == token.get_tag().to_int() {
//                     // POP
//                     token = self.scanner.next_token(scanner_env);
//                 } else {
//                     // ERROR
//                     panic!("Error: Expected {}, but found {}", a.to_string(), token.to_string());
//                 }

//             } else if a.get_tag().is_variable() {

//                 let p = self.grammar.get_production(a.get_tag(), token.get_tag());

//                 if p == -1 {
//                     token = self.scanner.next_token(scanner_env);
//                 } else if p == -2 {
//                     // Follow set, do nothing
//                 } else {
//                     self.push_rhs(stk, p as usize);
//                 }

//             } else {

//                 self.semantic.execute(&token);

//             }
//         }
//         true
//     }
// }