use crate::{Element, Level, Tex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use texcore_traits::*;

#[allow(non_camel_case_types)]
/// Represents the types of latex elements
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub enum Type {
    T_Input,
    T_Package,
    T_Part,
    T_Chapter,
    T_Header,
    T_Paragraph,
    T_Text,
    T_Environment,
    T_List,
    T_Item,
    T_Custom,
    T_Comment,
    T_Bundle,
}

/// Represents the metadata
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    pub author: String,
    pub date: String,
    pub title: String,
    pub fontsize: u8,
    pub papersize: String,
    pub doc_class: String,
    pub maketitle: bool,
}

impl Metadata {
    pub fn new(
        author: &str,
        date: &str,
        title: &str,
        fontsize: u8,
        papersize: &str,
        doc_class: &str,
        maketitle: bool,
    ) -> Self {
        Self {
            author: author.to_string(),
            date: date.to_string(),
            title: title.to_string(),
            fontsize,
            papersize: papersize.to_string(),
            doc_class: doc_class.to_string(),
            maketitle,
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(
            "author",
            "date",
            "title",
            11,
            "letterpaper",
            "article",
            true,
        )
    }
}

/// Represents the two different list types in latex
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub enum ListType {
    Itemized,
    Enumerated,
}

/// Represents the different text types in latex
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub enum TextType {
    Bold,
    Italics,
    Normal,
    Math,
    Par,
}

/// Represents any latex element
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Any {
    pub value: String,
    pub(crate) latex: String,
    pub type_: Type,
    pub level: Level,
    pub header_level: Option<u8>,
    pub text_type: Option<TextType>,
    pub list_type: Option<ListType>,
    pub items: Option<Vec<Item>>,
    pub elements: Option<Vec<Element<Any>>>,
    pub modified: bool,
}

/// Represents an environment in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct Environment {
    pub name: String,
    pub elements: Vec<Element<Any>>,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl ExtraOptions for Environment {
    fn modify_element(&mut self, options: Vec<Options>) {
        // we will have to rebuild the latex string
        let name = &self.name;
        let mut begin = format!(r"\begin{{{}}}", name);
        let end = format!(r"\end{{{}}}", name);
        for option in options {
            begin = option.modify(&begin);
        }
        let inner = self.inner_latex_string();
        self.latex = vec![begin, inner, end].join("\n");
        self.modified = true;
    }
}

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            elements: Vec::new(),
            latex: String::new(),
            modified: false,
        }
    }
    pub fn push(&mut self, element: Element<Any>) {
        self.elements.push(element)
    }
    pub fn set_elements(&mut self, elements: Vec<Element<Any>>) {
        self.elements = elements
    }
    pub fn inner_latex_string(&self) -> String {
        let mut strings = Vec::new();
        for e in &self.elements {
            strings.push(e.value.to_latex_string())
        }
        strings.join("\n")
    }
}

/// Represents custom Latex code given a string and a level to place it
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Custom {
    pub level: Level,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Custom {
    pub fn new(latex: &str, level: Level) -> Self {
        Self {
            level,
            latex: latex.to_string(),
            modified: false,
        }
    }
}

/// Represents a comment in LaTeX `% foo bar...`
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct Comment {
    pub value: String,
    pub level: Level,
    pub(crate) latex: String,
}

impl Comment {
    pub fn new(value: &str, level: Level) -> Self {
        Self {
            value: value.to_string(),
            level,
            latex: String::new(),
        }
    }
}

/// Represents `\input{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Input {
    pub file_name: PathBuf,
    pub level: Level,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Input {
    pub fn new(file_name: PathBuf, level: Level) -> Self {
        Self {
            file_name,
            level,
            latex: String::new(),
            modified: false,
        }
    }
    pub fn file_name_str(&self) -> String {
        match self.file_name.to_str() {
            None => String::new(),
            Some(p) => p.to_string(),
        }
    }
}

/// Represents `\usepackage{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Package {
    pub pkg: String,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Package {
    pub fn new(pkg: &str) -> Self {
        Self {
            pkg: pkg.to_string(),
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\part{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Part {
    pub name: String,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Part {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\chapter{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Chapter {
    pub name: String,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Chapter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\section{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Header {
    pub name: String,
    pub header_level: u8,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Header {
    pub fn new(name: &str, header_level: u8) -> Self {
        Self {
            name: name.to_string(),
            header_level,
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\<text type>{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct Text {
    pub content: String,
    pub type_: TextType,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Text {
    pub fn new(content: &str, type_: TextType) -> Self {
        Self {
            content: content.to_string(),
            type_,
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\paragraph{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct Paragraph {
    pub content: String,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl Paragraph {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\begin{list type} ... \end{list type}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize, ExtraOps)]
pub struct List {
    pub type_: ListType,
    pub items: Vec<Item>,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl List {
    pub fn new(type_: ListType, items: Vec<Item>) -> Self {
        Self {
            type_,
            items,
            latex: String::new(),
            modified: false,
        }
    }
}

/// Represents `\item{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
