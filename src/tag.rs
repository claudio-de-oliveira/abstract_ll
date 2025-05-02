
use std::fmt;

pub const VARIABLE : usize = 0x8000;
pub const TERMINAL : usize = 0x4000;
pub const ACTION : usize = 0x2000;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Tag {
    pub tag : usize,
    pub name : &'static str,
    pub num_att: usize,
}

impl Tag {
    pub const fn vn(t: usize, name: &'static str, num_att: usize) -> Tag {
        Tag{ tag: t | VARIABLE, name, num_att }
    }
    pub const fn vt(t: usize, name: &'static str, num_att: usize) -> Tag {
        Tag{ tag: t | TERMINAL, name, num_att }
    }
    pub const fn action(t: usize, name: &'static str, num_att: usize) -> Tag {
        Tag{ tag: t | ACTION, name, num_att }
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn to_int(&self) -> usize {
        self.tag & 0x0FFF
    }
    #[inline]
    pub const fn is_variable(&self) -> bool {
        (self.tag & VARIABLE) == VARIABLE
    }
    #[inline]
    pub const fn is_terminal(&self) -> bool {
        (self.tag & TERMINAL) == TERMINAL
    }
    #[inline]
    pub const fn is_action(&self) -> bool {
        (self.tag & ACTION) == ACTION
    }
    #[inline]
    pub const fn get_num_att(&self) -> usize {
        self.num_att
    }

    // pub fn to_string(&self) -> String {
    //     if self.is_variable() {
    //         format!("<{0}>", self.name)
    //     } else if self.is_terminal() {
    //         format!("\"{}\"", self.name)
    //     } else if self.is_action() {
    //         format!("@{}", self.name)
    //     } else {
    //         panic!("Unknown")
    //     }
    // }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", 
            if self.is_variable() {
                format!("<{0}>", self.name)
            } else if self.is_terminal() {
                format!("\"{}\"", self.name)
            } else if self.is_action() {
                format!("@{}", self.name)
            } else {
                panic!("Unknown")
            }
        )
    }
}