use std::fmt;

use crate::abstract_tag::AbstractTAG;

#[derive(Clone)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Attribute {
    pub name: String
}


#[derive(Debug)]
pub struct AbstractToken {
    tag: AbstractTAG<'static>,
    inherited : Option<Vec<Option<Attribute>>>,
}

impl AbstractToken {
    pub fn new(tag: AbstractTAG<'static>) -> Self {
        AbstractToken {
            tag: tag.clone(),
            inherited: if tag.get_num_att() > 0 { Some(vec![Option::<Attribute>::None; tag.get_num_att() as usize]) } else { None },
        }
    }

    #[inline]
    pub fn get_tag(&self) -> &AbstractTAG<'static> {
        &self.tag
    }

    pub fn exist_attribute(&self, i: usize) -> bool {
        self.inherited.is_some() && i < self.inherited.as_ref().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn get_attribute(&self, i: usize) -> &Attribute {

        if !self.exist_attribute(i) {
            panic!("{} deve herdar, pelo menos, {} atributo(s)!", self.to_string(), i + 1);
        }
        if self.inherited.as_ref().unwrap()[i].is_none() {
            panic!("{}[{}] não deve ser nulo!", self.to_string(), i);
        }

        self.inherited.as_ref().unwrap()[i].as_ref().unwrap()
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

    pub fn has_complement(&self) -> bool {
        match self.inherited {
            Some(_) => true,
            None => false,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{0} {1:?}", self.tag.to_string(), self.inherited)
    }

}

impl fmt::Display for AbstractToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.to_string())
    }
}