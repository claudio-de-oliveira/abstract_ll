#[cfg(test)]
mod tests {
    use crate::abstract_tag::AbstractTAG;

    #[test]
    fn tag_new() {
        let vn = AbstractTAG::vn(1, "variable", 1);
        let vt = AbstractTAG::vt(1, "terminal", 1);
        let ac = AbstractTAG::action(1, "action", 1);
        assert_eq!(vn.to_int(), 1);
        assert_eq!(vt.to_int(), 1);
        assert_eq!(ac.to_int(), 1);
    }

    #[test]
    fn tag_is() {
        let vn = AbstractTAG::vn(1, "variable", 1);
        let vt = AbstractTAG::vt(1, "terminal", 1);
        let ac = AbstractTAG::action(1, "action", 1);
        assert_eq!(vn.is_variable(), true);
        assert_eq!(vt.is_terminal(), true);
        assert_eq!(ac.is_action(), true);
    }
}
