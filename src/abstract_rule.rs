use std::sync::Arc;

use crate::abstract_tag::AbstractTAG;

pub struct Rule {
    pub lhs: AbstractTAG,
    pub rhs: Arc<[AbstractTAG]>,
    pub first: Arc<[AbstractTAG]>,
    pub follow: Arc<[AbstractTAG]>,
}

impl Rule {
    pub fn new(
        lhs: AbstractTAG, 
        rhs: Arc<[AbstractTAG]>, 
        first: Arc<[AbstractTAG]>, 
        follow: Arc<[AbstractTAG]>) -> Self {
        Rule { lhs, rhs, first, follow }
    }
}

impl Rule {
    pub fn get_lhs(&self) -> &AbstractTAG {
        &self.lhs
    }

    pub fn get_rhs(&self) -> &[AbstractTAG] {
        self.rhs.as_ref()
    }

    pub fn get_first(&self) -> &[AbstractTAG] {
        self.first.as_ref()
    }

    pub fn get_follow(&self) -> &[AbstractTAG] {
        self.follow.as_ref()
    }
}