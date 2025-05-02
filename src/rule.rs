use crate::tag::Tag;

pub struct Rule {
    pub lhs: &'static Tag,
    pub rhs: Vec<&'static Tag>,
    pub first: Vec<&'static Tag>,
    pub follow: Vec<&'static Tag>,
}

impl Rule {
    pub fn new(
        lhs: &'static Tag,
        rhs: Vec<&'static Tag>,
        first: Vec<&'static Tag>,
        follow: Vec<&'static Tag>) -> Self {
        Rule { lhs, rhs, first, follow }
    }
}

impl Rule {
    #[inline]
    pub const fn get_lhs(&self) -> &'static Tag {
        &self.lhs
    }

    #[inline]
    pub const fn get_rhs(&self) -> &Vec<&'static Tag> {
        &self.rhs
    }

    #[inline]
    pub const fn get_first(&self) -> &Vec<&'static Tag> {
        &self.first
    }

    #[inline]
    pub const fn get_follow(&self) -> &Vec<&'static Tag> {
        &self.follow
    }
}