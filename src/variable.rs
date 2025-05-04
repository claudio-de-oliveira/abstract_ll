pub struct Object {}

pub struct Variable {
    pub input_method : Option<String>,
    pub name : Option<String>,
    pub data_type : Option<String>,
    pub field_only : Option<String>,
    pub occurs_order : Option<String>,
    pub prompt : Option<String>,
    pub selections : Option<Vec<String>>,
    pub repeat: Option<Object>,
    pub repeats: usize,
    pub definition : Option<String>,
    pub logic: Option<Object>,
    pub default_format : Option<String>,
    pub original_format : Option<String>,
    pub depth : Option<String>,
    pub relevant: bool,
    pub visible: Option<Object>,
    pub value: Option<Object>,
}