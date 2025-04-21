use std::collections::HashMap;

use crate::{abstract_parser::Scanner, abstract_token::AbstractToken, scanner_environment::ScannerEnv};

pub struct ConcreteScanner {
    reserved_words: HashMap<String, AbstractToken>, // Maps reserved words to their token types
}

impl Scanner for ConcreteScanner {
    fn next_token(&self, _env: &mut ScannerEnv) -> AbstractToken {
        // Implementation of the next_token method
        // This is where you would implement the logic to return the next token from the input stream
        // For now, we will just return a placeholder token
        // AbstractToken::new(RELOP) // Placeholder implementation
    }
    
    fn initialize(&mut self) {
        // Initialize the scanner with the environment
    }
}