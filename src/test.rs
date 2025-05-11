#[cfg(test)]
mod tag_tests {

    use crate::tag::Tag;

    #[test]
    fn tag_new() {
        let vn = Tag::vn(1, "variable", 1);
        let vt = Tag::vt(1, "terminal", 1);
        let ac = Tag::action(1, "action", 1);
        assert_eq!(vn.to_int(), 1);
        assert_eq!(vt.to_int(), 1);
        assert_eq!(ac.to_int(), 1);
    }

    #[test]
    fn tag_is() {
        let vn = Tag::vn(1, "variable", 1);
        let vt = Tag::vt(1, "terminal", 1);
        let ac = Tag::action(1, "action", 1);
        assert_eq!(vn.is_variable(), true);
        assert_eq!(vt.is_terminal(), true);
        assert_eq!(ac.is_action(), true);
    }

    #[test]
    fn tag_to_string() {
        let vn = Tag::vn(1, "variable", 1);
        let vt = Tag::vt(1, "terminal", 1);
        let ac = Tag::action(1, "action", 1);
        assert_eq!(vn.to_string(), "<variable>");
        assert_eq!(vt.to_string(), "\"terminal\"");
        assert_eq!(ac.to_string(), "@action");
    }

}

#[cfg(test)]
mod token_tests {
    use crate::{tag::Tag, token::{Attribute, SimpleToken, Token}};

    #[test]
    fn tag_get_attribute() {
        let tag = Tag::vn(1, "teste", 3);
        let token = &mut SimpleToken::from_tag(tag);

        token.set_attribute(0, &Attribute {
            name: "attribute1".to_string(),
        });
        token.set_attribute(1, &Attribute {
            name: "attribute2".to_string(),
        });
        token.set_attribute(2, &Attribute {
            name: "attribute3".to_string(),
        });

        assert_eq!(token.get_attribute(0).name, "attribute1");
        assert_eq!(token.get_attribute(1).name, "attribute2");
        assert_eq!(token.get_attribute(2).name, "attribute3");
    }
}


#[cfg(test)]
mod environment {
    use crate::scanner_environment::ScannerEnv;

    fn create_env() -> ScannerEnv<'static> {
        let text_ = "int {\na = 0;\n#";

        let vtext_ = text_.split('\n').collect::<Vec<_>>();

        ScannerEnv::new(vtext_)
    }

    #[test]
    fn environment_next_char() {
        let mut env_ = create_env();
        assert_eq!(env_.next_char(), 'i');
        assert_eq!(env_.next_char(), 'n');
        assert_eq!(env_.next_char(), 't');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.next_char(), 'a');
        assert_ne!(env_.next_char(), 'x');
    }

    #[test]
    fn environment_current_row() {
        let mut env_ = create_env();
        assert_eq!(env_.next_char(), 'i');
        assert_eq!(env_.next_char(), 'n');
        assert_eq!(env_.next_char(), 't');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.get_current_row(), 0);
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.get_current_row(), 1);
    }

    #[test]
    fn environment_current_col() {
        let mut env_ = create_env();
        assert_eq!(env_.get_current_col(), 0);
        assert_eq!(env_.next_char(), 'i');
        assert_eq!(env_.get_current_col(), 1);
        assert_eq!(env_.next_char(), 'n');
        assert_eq!(env_.get_current_col(), 2);
        assert_eq!(env_.next_char(), 't');
        assert_eq!(env_.get_current_col(), 3);
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.get_current_col(), 4);
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.get_current_col(), 5);
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.get_current_col(), 0);
    }

    #[test]
    fn environment_retract() {
        let mut env_ = create_env();
        assert_eq!(env_.next_char(), 'i');
        assert_eq!(env_.next_char(), 'n');
        assert_eq!(env_.next_char(), 't');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.get_current_row(), 1);
        env_.retract();
        assert_eq!(env_.get_current_row(), 0);
        assert_eq!(env_.get_current_col(), 4);
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.get_current_col(), 0);
    }

    #[test]
    fn environment_end_of_text() {
        let mut env_ = create_env();
        assert_eq!(env_.next_char(), 'i');
        assert_eq!(env_.next_char(), 'n');
        assert_eq!(env_.next_char(), 't');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '{');
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.next_char(), 'a');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '=');
        assert_eq!(env_.next_char(), ' ');
        assert_eq!(env_.next_char(), '0');
        assert_eq!(env_.next_char(), ';');
        assert_eq!(env_.next_char(), '\n');
        assert_eq!(env_.next_char(), '#');
        assert!(env_.end_of_text());
    }
}

#[cfg(test)]
mod scanner {
    use std::collections::HashMap;

    use crate::{grammar::{self}, literal::Literal, scanner::{CeScanner, Scanner}, scanner_environment::ScannerEnv, token::{self, ComplementTrait, Token}, variable::Variable};

    fn create_scanner_env(text: &str) -> ScannerEnv {
        let vtext_ = text.split('\n').collect::<Vec<_>>();

        ScannerEnv::new(vtext_)
    }

    fn create_symbol_table() -> HashMap::<String, Box<dyn Token>> {
        HashMap::<String, Box<dyn Token>>::new()
    }

    fn create_variables() -> HashMap::<String, Variable> {
        HashMap::<String, Variable>::new()
    }

    #[test]
    fn scanner_next_add_operators() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let mut scanner_env = create_scanner_env(" + - ");
        let scanner = CeScanner::new();

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_ADDOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::AddOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::AddOpType::Plus);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_ADDOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::AddOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::AddOpType::Minus);
    }

    #[test]
    fn scanner_next_mul_operators() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let mut scanner_env = create_scanner_env(" * / ");
        let scanner = CeScanner::new();

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_MULOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::MulOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::MulOpType::Multiply);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_MULOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::MulOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::MulOpType::Divide);
    }

    #[test]
    fn scanner_next_rel_operators() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let mut scanner_env = create_scanner_env("is isatleast isatmost islessthan ismorethan ");
        let scanner = CeScanner::new();

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_RELOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::RelOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::RelOpType::Is);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_RELOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::RelOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::RelOpType::IsAtLeast);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_RELOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::RelOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::RelOpType::IsAtMost);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_RELOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::RelOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::RelOpType::IsLessThan);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_RELOP);
        let token = token.as_any().downcast_ref::<token::ValuedToken<grammar::RelOpType>>().unwrap();
        assert_eq!(token.get_complement(), grammar::RelOpType::IsMoreThan);
    }

    #[test]
    fn scanner_next_single_quoting_variable_names() {
        let symbol_table = create_symbol_table();
        let mut variables = create_variables();
        let mut scanner_env = create_scanner_env("'Tax Regulations Apply' ");
        variables.insert( "'Tax Regulations Apply'".to_string(), Variable { repeats: 1, relevant:false, input_method: None, name: Some("'Tax Regulations Apply'".to_string()), data_type: None, field_only: None, occurs_order: None, prompt: None, selections: None, repeat: None, definition: None, logic: None, default_format: None, original_format: None, depth: None, visible: None, value: None } );
        let scanner = CeScanner::new();

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_VARIABLE);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<Variable>>().unwrap();
        let variable = valued_token.get_complement();
        assert_eq!(variable.name.unwrap(), "'Tax Regulations Apply'".to_string());
    }

    #[test]
    fn scanner_next_double_quoting_variable_names() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let mut scanner_env = create_scanner_env("\"North America\" ");
        let scanner = CeScanner::new();

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_LITERAL);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<Literal>>().unwrap();
        let literal = valued_token.get_complement();
        assert_eq!(literal.value, "North America".to_string());
    }

    #[test]
    fn scanner_next_reserved_words() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let scanner = CeScanner::new();
        let text = scanner.reserved_words.keys().cloned().collect::<Vec<&str>>().join(" ") + "#";
        let mut scanner_env = create_scanner_env(&text);

        for word in scanner.reserved_words.keys().into_iter() {
            let tag = scanner.reserved_words.get(word);
            let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
            assert_eq!(token.get_tag(), *tag.unwrap());
        }
    }

    #[test]
    fn scanner_next_numbers() {
        let symbol_table = create_symbol_table();
        let variables = create_variables();
        let scanner = CeScanner::new();
        let mut scanner_env = create_scanner_env("1 23 5678 12.34 0.45 17.0 ");

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_INTEGER);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<isize>>().unwrap();
        assert_eq!(valued_token.get_complement(), 1_isize);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_INTEGER);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<isize>>().unwrap();
        assert_eq!(valued_token.get_complement(), 23_isize);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_INTEGER);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<isize>>().unwrap();
        assert_eq!(valued_token.get_complement(), 5678_isize);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_DECIMAL);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<f64>>().unwrap();
        assert_eq!(valued_token.get_complement(), 12.34_f64);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_DECIMAL);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<f64>>().unwrap();
        assert_eq!(valued_token.get_complement(), 0.45_f64);

        let token = scanner.next_token(&mut scanner_env, &symbol_table, &variables);
        assert_eq!(token.get_tag(), grammar::VT_DECIMAL);
        let valued_token = token.as_any().downcast_ref::<token::ValuedToken<f64>>().unwrap();
        assert_eq!(valued_token.get_complement(), 17.0_f64);
    }

}
