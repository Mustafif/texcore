use crate::{Any, Element};


pub trait ExtraOptions {
    fn modify_element(&mut self, options: Vec<Option>);
}

#[derive(Debug, Clone)]
pub enum Option {
    Curly(String),
    Square(String),
}

impl Option {
    pub fn modify(&self, latex: &str) -> String {
        let mut latex = latex.to_string();
        match &self {
            Option::Curly(option) => {
                let option = format!("{{{option}}}");
                latex.push_str(&option)
            }
            Option::Square(option) => {
                let option = format!("[{option}]");
                latex.push_str(&option);
            }
        }
        latex
    }
}

impl ExtraOptions for Element<Any> {
    fn modify_element(&mut self, options: Vec<Option>) {
        for option in options {
            self.latex = option.modify(&self.latex);
        }
    }
}

