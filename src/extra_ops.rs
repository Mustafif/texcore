use crate::{
    options_implement, Any, Chapter, Custom, Element, Environment, Header, Input, List, Package,
    Paragraph, Part, Tex, Text,
};

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

/// # Extra Options
/// This trait provides a way to modify an element, the reason of using a trait
/// instead of adding a method to `Element<Any>` is because of allowing users to implement this
/// for any element type, and the possibility of adding more methods to this trait.
///
/// The basic idea of `ExtraOptions` to allow users to add extra arguments on an element, consider putting an image
/// with a scale of `0.75`,we can do the following:
/// ```rust
/// use std::path::PathBuf;
/// use texcore::{Custom, Element, Level};
/// use texcore::bundle::graphicx::GraphicInclude;
/// use texcore::extra_ops::{ExtraOptions, Options};
///
/// let image = GraphicInclude::new(PathBuf::from("foo.png"));
/// let mut image_el = Element::from(image);
/// image_el.modify_element(
///     vec![Options::Square("scale = 0.75".to_string())]
/// );
/// ```
/// This results in the follow LaTeX code:
/// ```latex
/// \includegraphics{foo.png}[scale = 0.75]
/// ```
impl ExtraOptions for Element<Any> {
    fn modify_element(&mut self, options: Vec<Options>) {
        for option in options {
            self.latex = option.modify(&self.latex);
        }
    }
}

options_implement!(
    Part,
    Any,
    Environment,
    Custom,
    Input,
    Package,
    Chapter,
    Header,
    Text,
    Paragraph,
    List
);
