#[cfg(test)]
mod abstract_tag_tests {
    use std::fmt::Debug;

    use crate::abstract_tag::{AbstractTAG, Attribute};

    #[test]
    fn abstract_tag_new() {
        let vn = AbstractTAG::vn(1, "variable", 1);
        let vt = AbstractTAG::vt(1, "terminal", 1);
        let ac = AbstractTAG::action(1, "action", 1);
        assert_eq!(vn.to_int(), 1);
        assert_eq!(vt.to_int(), 1);
        assert_eq!(ac.to_int(), 1);
    }

    #[test]
    fn abstract_tag_is() {
        let vn = AbstractTAG::vn(1, "variable", 1);
        let vt = AbstractTAG::vt(1, "terminal", 1);
        let ac = AbstractTAG::action(1, "action", 1);
        assert_eq!(vn.is_variable(), true);
        assert_eq!(vt.is_terminal(), true);
        assert_eq!(ac.is_action(), true);
    }

    #[test]
    fn abstract_tag_get_attribute() {
        let vn = &mut AbstractTAG::vn(1, "variable", 3);

        vn.set_attribute(0, &Attribute {
            name: "attribute1".to_string(),
        });
        vn.set_attribute(1, &Attribute {
            name: "attribute2".to_string(),
        });
        vn.set_attribute(2, &Attribute {
            name: "attribute3".to_string(),
        });

        assert_eq!(vn.get_attribute(0).name, "attribute1");
        assert_eq!(vn.get_attribute(1).name, "attribute2");
        assert_eq!(vn.get_attribute(2).name, "attribute3");
    }

    #[test]
    fn abstract_tag_to_string() {
        let vn = AbstractTAG::vn(1, "variable", 1);
        let vt = AbstractTAG::vt(1, "terminal", 1);
        let ac = AbstractTAG::action(1, "action", 1);
        assert_eq!(vn.to_string(), "<variable>");
        assert_eq!(vt.to_string(), "\"terminal\"");
        assert_eq!(ac.to_string(), "@action");
    }

}

#[cfg(test)]
mod abstract_token_tests {
    use crate::{abstract_tag::AbstractTAG, abstract_token::AbstractToken};

    #[test]
    fn abstract_token_new() {
        let tag = AbstractTAG::vn(1, "variable", 1);
        let token = AbstractToken::new(tag);
        assert_eq!(token.get_tag().to_int(), 1);
        assert_eq!(token.has_complement(), true);
    }

    #[test]
    fn abstract_token_format() {
        let tag = &mut AbstractTAG::vn(1, "variable", 3);
        let token = AbstractToken::new(tag);

        tag.set_attribute(0, &Attribute {
            name: "attribute1".to_string(),
        });
        tag.set_attribute(1, &Attribute {
            name: "attribute2".to_string(),
        });

        assert_eq!(format!("{:#?}", token).to_string(), String::from("AbstractTAG {
    tag: 32769,
    name: \"variable\",
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

