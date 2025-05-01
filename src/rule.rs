use std::sync::Arc;

use crate::tag::Tag;

pub struct Rule {
    pub lhs: Tag,
    pub rhs: Arc<[Tag]>,
    pub first: Arc<[Tag]>,
    pub follow: Arc<[Tag]>,
}

impl Rule {
    pub fn new(
        lhs: Tag, 
        rhs: Arc<[Tag]>, 
        first: Arc<[Tag]>, 
        follow: Arc<[Tag]>) -> Self {
        Rule { lhs, rhs, first, follow }
    }
}

impl Rule {
    pub fn get_lhs(&self) -> &Tag {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &[Tag] {
        self.rhs.as_ref()
    }

    pub fn get_first(&self) -> &[Tag] {
        self.first.as_ref()
    }

    pub fn get_follow(&self) -> &[Tag] {
        self.follow.as_ref()
    }
}