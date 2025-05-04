use std::collections::HashMap;

use crate::{token::Token, variable::Variable};


pub struct SemanticEnv {
    symbol_table : HashMap<String, Box<dyn Token>>,
    variables : HashMap<String, Variable>,
}