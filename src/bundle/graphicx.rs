use crate::extra_ops::{ExtraOptions, Options};
use crate::Type::T_Bundle;
use crate::{Any, Element, Level, Package, Tex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Returns the `graphicx` package
pub fn graphicx_pkg() -> Element<Any> {
    let pkg = Package::new("graphicx");
    Element::from(pkg)
}

/// Used to declare a path for all images.
/// Returns the latex string `\graphicspath{ {./foo/} }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicPath {
    path: PathBuf,
}

impl GraphicPath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Tex for GraphicPath {
    fn to_latex_string(&self) -> String {
        let path = match self.path.to_str() {
            None => "".to_string(),
            Some(s) => s.to_string(),
        };
        format!(r"\graphicspath{{ {{{path}}} }}")
    }
}

impl From<GraphicPath> for Element<Any> {
    fn from(path: GraphicPath) -> Self {
        let latex = path.to_latex_string();
        let any = Any {
            value: path.path.to_str().unwrap_or("").to_string(),
            latex: latex.to_string(),
            type_: T_Bundle,
            level: Level::Meta,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
        };
        Element::new_any(any)
    }
}

/// Used to declare the a path to include an image.
/// Returns the latex string `\includegraphics{foo.png}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicInclude {
    path: PathBuf,
}

impl GraphicInclude {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Tex for GraphicInclude {
    fn to_latex_string(&self) -> String {
        let path = match self.path.to_str() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        format!(r"\includegraphics{{{path}}}")
    }
}

impl From<GraphicInclude> for Element<Any> {
    fn from(incl: GraphicInclude) -> Self {
        let latex = incl.to_latex_string();
        let any = Any {
            value: incl.path.to_str().unwrap_or("").to_string(),
            latex: latex.to_string(),
            type_: T_Bundle,
            level: Level::Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
        };
        Element::new_any(any)
    }
}

/// A bundle that will return an array of all images, the graphics path and the package `graphicx`.
pub fn graphics_bundle(
    graphic_path: PathBuf,
    image_paths: Vec<PathBuf>,
    scales: Vec<f32>,
) -> Vec<Element<Any>> {
    let scales: Vec<Options> = {
        let mut options = Vec::new();
        for s in scales {
            let s = format!("scale = {s}");
            options.push(Options::Square(s))
        }
        options
    };
    let mut elements = Vec::new();
    elements.push(graphicx_pkg());
    let gp = GraphicPath::new(graphic_path);
    elements.push(Element::from(gp));

    let mut gi_elements = Vec::new();

    for ip in image_paths {
        let gi = GraphicInclude::new(ip);
        gi_elements.push(Element::from(gi));
    }

    for (i, op) in scales.iter().enumerate() {
        gi_elements[i].modify_element(vec![op.clone()]);
        elements.push(gi_elements[i].clone())
    }

    elements
}

#[cfg(test)]
mod tests {
    use crate::bundle::graphicx::GraphicPath;
    use crate::Tex;
    use std::path::PathBuf;

    #[test]
    fn test_path() {
        let gp = GraphicPath::new(PathBuf::from("./images/"));
        let s = r"\graphicspath{ {./images/} }";
        assert_eq!(s.to_string(), gp.to_latex_string())
    }
}
