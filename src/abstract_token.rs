use std::fmt;

use crate::abstract_tag::AbstractTAG;

pub struct AbstractToken {
    tag: AbstractTAG<'static>,
}

impl AbstractToken {
    pub fn new(tag: AbstractTAG<'static>) -> Self {
        AbstractToken { tag }
    }

    pub fn get_tag(&self) -> &AbstractTAG<'static> {
        &self.tag
    }

    pub fn has_complement(&self) -> bool {
        self.tag.has_complement()
    }
}

impl fmt::Display for AbstractToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.tag.to_string())
    }
}