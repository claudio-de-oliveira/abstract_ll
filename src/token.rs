use std::{any::Any, fmt};
use crate::tag::Tag;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Attribute {
    pub name: String
}

pub trait Token: Any {
    fn get_tag(&self) -> Tag;
    fn exist_attribute(&self, i: usize) -> bool;
    fn get_attribute(&self, i: usize) -> &Attribute;
    fn set_attribute(&mut self, i: usize, v: &Attribute);
    fn has_complement(&self) -> bool;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}


#[derive(Debug, Clone)]
pub struct SimpleToken {
    tag: Tag,
    inherited : Option<Vec<Option<Attribute>>>,
}

impl SimpleToken {
    pub fn from_tag(tag: Tag) -> impl Token {
        if tag.get_num_att() > 0 {
            SimpleToken {
                tag: tag.clone(),
                inherited: Some(vec![Option::<Attribute>::None; tag.get_num_att() as usize]),
            }
        } else {
            SimpleToken {
                tag: tag.clone(),
                inherited: None,
            }
        }
    }

    fn to_string(&self) -> String {
        format!("{0} {1:?}", self.tag.to_string(), self.inherited)
    }
}

impl fmt::Display for SimpleToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.to_string())
    }
}

impl Token for SimpleToken {
    #[inline]
    fn get_tag(&self) -> Tag {
        self.tag.clone()
    }

    fn exist_attribute(&self, i: usize) -> bool {
        self.inherited.is_some() && i < self.tag.get_num_att()
    }

    #[allow(dead_code)]
    fn get_attribute(&self, i: usize) -> &Attribute {

        if !self.exist_attribute(i) {
            panic!("{} deve herdar, pelo menos, {} atributo(s)!", self.to_string(), i + 1);
        }
        if self.inherited.as_ref().unwrap()[i].is_none() {
            panic!("{}[{}] não deve ser nulo!", self.to_string(), i);
        }

        self.inherited.as_ref().unwrap()[i].as_ref().unwrap()
    }

    #[allow(dead_code)]
    fn set_attribute(&mut self, i: usize, v: &Attribute) {

        if !self.exist_attribute(i) {
            panic!("{} deve herdar, pelo menos, {} atributo(s)!", self.to_string(), i + 1);
        }

        if let Some(inherited) = &mut self.inherited {
            inherited[i] = Some(v.clone());
        }
    }

    fn has_complement(&self) -> bool {
        match self.inherited {
            Some(_) => true,
            None => false,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ValuedToken<C> where C: Clone {
    tag: Tag,
    inherited : Option<Vec<Option<Attribute>>>,
    complement: C,
}

pub trait ComplementTrait<C> where C: Clone {
    fn create(tag: Tag, complement: C) -> ValuedToken<C>;
    fn get_complement(&self) -> C;
}

impl<C> ComplementTrait<C> for ValuedToken<C> where C: Clone {
     fn create(tag: Tag, complement: C) -> ValuedToken<C> {
        if tag.get_num_att() > 0 { 
            ValuedToken {
                tag: tag.clone(),
                inherited: Some(vec![Option::<Attribute>::None; tag.get_num_att() as usize]),
                complement: complement.clone(),
            }
        } else {
            ValuedToken {
                tag,
                inherited: None,
                complement: complement.clone(),
            }
        }
    }

    fn get_complement(&self) -> C {
        self.complement.clone()
    }
}

impl<C: Clone + PartialEq + 'static> Token for ValuedToken<C> {
    #[inline]
    fn get_tag(&self) -> Tag {
        self.tag.clone()
    }

    fn exist_attribute(&self, i: usize) -> bool {
        self.inherited.is_some() && i < self.tag.get_num_att()
    }

    fn get_attribute(&self, i: usize) -> &Attribute {
        self.inherited.as_ref().unwrap()[i].as_ref().unwrap()
    }

    fn set_attribute(&mut self, i: usize, v: &Attribute) {
        self.inherited.as_mut().unwrap()[i] = Some(v.clone());
    }

    fn has_complement(&self) -> bool {
        false
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
