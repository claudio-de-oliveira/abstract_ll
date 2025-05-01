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
    use crate::{tag::Tag, token::Token, token::Attribute};

    #[test]
    fn token_new() {
        let tag = Tag::vn(1, "variable", 1);
        let token = Token::new(tag);
        assert_eq!(token.get_tag().to_int(), 1);
        assert_eq!(token.has_complement(), true);
    }

    #[test]
    fn tag_get_attribute() {
        let tag = Tag::vn(1, "variable", 3);
        let token = &mut Token::new(tag);

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

    #[test]
    fn token_format() {
        let tag = Tag::vn(1, "variable", 3);
        let token = &mut Token::new(tag);

        token.set_attribute(0, &Attribute {
            name: "attribute1".to_string(),
        });
        token.set_attribute(1, &Attribute {
            name: "attribute2".to_string(),
        });

        println!("{:#?}", token);

        assert_eq!(format!("{:#?}", token).to_string(), String::from("Token {
    tag: Tag {
        tag: 32769,
        name: \"variable\",
        num_att: 3,
    },
    inherited: Some(
        [
            Some(
                Attribute {
                    name: \"attribute1\",
                },
            ),
            Some(
                Attribute {
                    name: \"attribute2\",
                },
            ),
            None,
        ],
    ),
}"));

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
}
