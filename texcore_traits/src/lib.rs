pub use texcore_derive::ExtraOps;

/// A trait to modify an element to add extra options to it.
pub trait ExtraOptions {
    fn modify_element(&mut self, options: Vec<Options>);
}

/// Options of either adding extra arguments using `{}` (curly) or `[]` (square).
#[derive(Debug, Clone)]
pub enum Options {
    Curly(String),
    Square(String),
}

impl Options {
    pub fn modify(&self, latex: &str) -> String {
        let mut latex = latex.to_string();
        match &self {
            Options::Curly(option) => {
                let option = format!("{{{option}}}");
                latex.push_str(&option)
            }
            Options::Square(option) => {
                let option = format!("[{option}]");
                latex.push_str(&option);
            }
        }
        latex
    }
}