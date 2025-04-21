
use std::fmt;

pub const VARIABLE : usize = 0x8000;
pub const TERMINAL : usize = 0x4000;
pub const ACTION : usize = 0x2000;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AbstractTAG<'a> {
    pub tag : usize,
    pub name : &'a str,
    pub num_att: u16,
}

impl<'a> AbstractTAG<'a> {
    pub fn new(tag: usize, name: &'a str, num_att: u16) -> Self {
        AbstractTAG {
            tag,
            name,
            num_att,
        }
    }

    pub fn vn(t: usize, name: &'static str, num_att: u16) -> AbstractTAG<'static> {
        AbstractTAG{ tag: t | VARIABLE, name, num_att }
    }
    pub fn vt(t: usize, name: &'static str, num_att: u16) -> AbstractTAG<'static> {
        AbstractTAG{ tag: t | TERMINAL, name, num_att }
    }
    pub fn action(t: usize, name: &'static str, num_att: u16) -> AbstractTAG<'static> {
        AbstractTAG{ tag: t | ACTION, name, num_att }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn to_int(&self) -> usize {
        self.tag & 0x0FFF
    }
    #[inline]
    pub fn is_variable(&self) -> bool {
        (self.tag & VARIABLE) == VARIABLE
    }
    #[inline]
    pub fn is_terminal(&self) -> bool {
        (self.tag & TERMINAL) == TERMINAL
    }
    #[inline]
    pub fn is_action(&self) -> bool {
        (self.tag & ACTION) == ACTION
    }
    #[inline]
    pub fn get_num_att(&self) -> u16 {
        self.num_att
    }

    pub fn to_string(&self) -> String {
        if self.is_variable() {
            format!("<{0}>", self.name)
        } else if self.is_terminal() {
            format!("\"{}\"", self.name)
        } else if self.is_action() {
            format!("@{}", self.name)
        } else {
            panic!("Unknown")
        }
    }
}

impl fmt::Display for AbstractTAG<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.to_string())
    }
}