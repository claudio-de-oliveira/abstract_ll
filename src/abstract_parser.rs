use crate::{abstract_tag::AbstractTAG, abstract_token::AbstractToken};
use array2d::Array2D;

pub trait Scanner {
    fn next_token(&self, text : &str) -> AbstractToken;
    fn initialize(&mut self);
}

pub trait Semantic {
    fn execute(&self, action : &AbstractToken);
    fn initialize(&mut self);
}

#[allow(dead_code)]
pub struct AbstractParser<'a> {
    scanner: Box<dyn Scanner>,
    semantic: Box<dyn Semantic>,

    rhs: Box<Array2D<AbstractTAG<'a>>>, // Array2D storing AbstractTAG elements
    m: Box<Array2D<i32>>, // Array2D storing AbstractTAG elements
    end_mark: AbstractTAG<'a>,
}

impl<'a> AbstractParser<'a> {
    pub fn new(
        scanner: Box<dyn Scanner>,
        semantic: Box<dyn Semantic>,
        rhs: Box<Array2D<AbstractTAG<'a>>>,
        m: Box<Array2D<i32>>,
        end_mark: AbstractTAG<'a>
    ) -> Self {
        AbstractParser {
            scanner,
            semantic,
            rhs,
            m,
            end_mark,
        }
    }

    pub fn get_end_mark(&'a self) -> &'a AbstractTAG<'a> {
        &self.end_mark
    }
}