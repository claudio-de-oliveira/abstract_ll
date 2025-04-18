
use std::{fmt, vec::Vec};

const VARIABLE : u16 = 0x8000;
const TERMINAL : u16 = 0x4000;
const ACTION : u16 = 0x2000;


#[derive(Clone)]
#[derive(Debug)]
pub struct Attribute {
    pub name: String
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AbstractTAG<'a> {
    tag : u16,
    name : &'a str,
    inherited : Option<Vec<Option<Attribute>>>,
}

impl<'a> AbstractTAG<'a> {
    pub fn new(tag: u16, name: &'a str, num_att: u16) -> Self {
        AbstractTAG {
            tag,
            name,
            inherited: if num_att > 0 { Some(vec![Option::<Attribute>::None; num_att as usize]) } else { None }
        }
    }

    pub fn vn(tag: u16, name: &str, num_att: u16) -> AbstractTAG {
        AbstractTAG::new(tag | VARIABLE, name, num_att)
    }
    pub fn vt(t: u16, name: &str, num_att: u16) -> AbstractTAG {
        AbstractTAG::new(t | TERMINAL, name, num_att)
    }
    pub fn action(t: u16, name: &str, num_att: u16) -> AbstractTAG {
        AbstractTAG::new(t | ACTION, name, num_att)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn to_int(&self) -> u16 {
        self.tag & 0x0FFFu16
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

    pub fn exist_attribute(&self, i: usize) -> bool {
        self.inherited.is_some() && i < self.inherited.as_ref().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn get_attribute(&self, i: usize) -> Option<&Attribute> {

        if !self.exist_attribute(i) {
            panic!("{} deve herdar, pelo menos, {} atributo(s)!", self.to_string(), i + 1);
            // return None;
        }
        if self.inherited.as_ref().unwrap()[i].is_none() {
            panic!("{}[{}] não deve ser nulo!", self.to_string(), i);
            // return None;
        }

        self.inherited.as_ref().unwrap()[i].as_ref()
    }

    #[allow(dead_code)]
    pub fn set_attribute(&mut self, i: usize, v: &Attribute) {

        if !self.exist_attribute(i) {
            panic!("{} deve herdar, pelo menos, {} atributo(s)!", self.to_string(), i + 1);
        }

        if let Some(inherited) = &mut self.inherited {
            inherited[i] = Some(v.clone());
        }
    }

}

impl fmt::Display for AbstractTAG<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.to_string())
    }
}