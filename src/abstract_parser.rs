use crate::{abstract_tag::AbstractTAG, abstract_token::AbstractToken, abstract_rule::Rule};
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
    scanner: &'a mut Box<dyn Scanner>,
    semantic: &'a mut Box<dyn Semantic>,

    rules: &'a [Rule<'a>], // Array2D storing AbstractTAG elements
    m: Box<Array2D<i32>>, // Array2D storing AbstractTAG elements
    end_mark: AbstractTAG<'a>,
}

impl<'a> AbstractParser<'a> {
    pub fn new(
        rules: &'a [Rule<'a>],
        number_of_terminals: usize,
        number_of_variables: usize,
        scanner_: &'a mut Box<dyn Scanner>,
        semantic_: &'a mut Box<dyn Semantic>,
        end_mark: AbstractTAG<'a>
    ) -> Self {

        let mut m_ = Box::new(Array2D::filled_with(-1, number_of_terminals, number_of_variables));

        // Initialize the parsing table with -1
        for i in 0..number_of_terminals {
            for j in 0..number_of_variables {
                m_[(i, j)] = -1; // Set all entries to -1
            }
        }

        // Fill the parsing table with the rules
        for rule in 0..rules.len() {
            let row = rules[rule].lhs.to_int();
            for first in rules[rule].first.iter() {
                let col = first.to_int();
                if m_[(row, col)] >= 0 {
                    panic!("Conflict in parsing table at row {}, col {}", row, col);
                };
                m_[(row, col)] = rule as i32; // Set the rule index
            }
            for follow in rules[rule].follow.iter() {
                let col = follow.to_int();
                if m_[(row, col)] == -1 {
                    m_[(row, col)] = -2; // Set to -2 for follow
                }
            }
        }

        AbstractParser {
            scanner: scanner_,
            semantic: semantic_,
            rules,
            m: m_,
            end_mark,
        }
    }

    #[inline]
    pub fn get_end_mark(&'a self) -> &'a AbstractTAG<'a> {
        &self.end_mark
    }

    #[allow(dead_code)]
    fn production(&self, a: AbstractTAG<'a>, token: &AbstractToken) -> i32 {
        let row = a.to_int() as usize;
        let col = token.get_tag().to_int() as usize;

        if row < self.m.row_len() && col < self.m.column_len() {
            return self.m[(row, col)];
        } else {
            return -1;
        }
    }

    fn push_rhs(&self, stk: &mut Vec<AbstractTAG<'a>>, p: usize) {
        // Push the right-hand side of the production rule onto the stack in reverse order
        for i in (0..self.rules[p].rhs.len()).rev() {
            stk.push(self.rules[p].rhs[i].clone());
        }
    }

   
    pub fn parse(&mut self, text: &str) -> bool {
        let stk = &mut Vec::<AbstractTAG<'a>>::new();
        let mut token = self.scanner.next_token(text);

        self.push_rhs(stk, 0);

        self.scanner.initialize();
        self.semantic.initialize();

        true
    }
}