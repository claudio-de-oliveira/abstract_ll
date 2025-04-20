use std::sync::Arc;

use crate::abstract_tag::AbstractTAG;

pub struct Rule<'a> {
    pub lhs: AbstractTAG<'a>,
    pub rhs: Arc<[AbstractTAG<'a>]>,
    pub first: Arc<[AbstractTAG<'a>]>,
    pub follow: Arc<[AbstractTAG<'a>]>,
}

impl<'a> Rule<'a> {
    pub fn new(
        lhs: AbstractTAG<'a>, 
        rhs: Arc<[AbstractTAG<'a>]>, 
        first: Arc<[AbstractTAG<'a>]>, 
        follow: Arc<[AbstractTAG<'a>]>) -> Self {
        Rule { lhs, rhs, first, follow }
    }
}

impl<'a> Rule<'a> {
    pub fn get_lhs(&self) -> &AbstractTAG<'a> {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &[AbstractTAG<'a>] {
        self.rhs.as_ref()
    }

    pub fn get_first(&self) -> &[AbstractTAG<'a>] {
        self.first.as_ref()
    }

    pub fn get_follow(&self) -> &[AbstractTAG<'a>] {
        self.follow.as_ref()
    }
}