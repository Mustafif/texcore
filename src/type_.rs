use crate::{Element, Level, Tex};

#[allow(non_camel_case_types)]
/// Represents the types of latex elements
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
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
}

/// Represents the metadata
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Metadata {
    pub author: String,
    pub date: String,
    pub title: String,
    pub fontsize: u8,
    pub doc_class: String,
    pub maketitle: bool,
}

impl Metadata {
    pub fn new(
        author: String,
        date: String,
        title: String,
        fontsize: u8,
        doc_class: String,
        maketitle: bool,
    ) -> Self {
        Self {
            author,
            date,
            title,
            fontsize,
            doc_class,
            maketitle,
        }
    }
}

/// Represents the two different list types in latex
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ListType {
    Itemized,
    Enumerated,
}

/// Represents the different text types in latex
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum TextType {
    Bold,
    Italics,
    Normal,
    Math,
    Par,
}

/// Represents any latex element
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Any {
    pub value: String,
    pub type_: Type,
    pub level: Option<Level>,
    pub header_level: Option<u8>,
    pub text_type: Option<TextType>,
    pub list_type: Option<ListType>,
    pub items: Option<Vec<Item>>,
    pub elements: Option<Vec<Element<Any>>>,
}

/// Represents an environment in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Environment {
    pub name: String,
    pub elements: Vec<Element<Any>>,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            elements: Vec::new(),
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Custom {
    pub value: String,
    pub level: Level,
}

impl Custom {
    pub fn new(value: String, level: Level) -> Self {
        Self { value, level }
    }
}

/// Represents `\input{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Input {
    pub file_name: String,
    pub level: Option<Level>,
}

impl Input {
    pub fn new(file_name: String, level: Option<Level>) -> Self {
        Self { file_name, level }
    }
}

/// Represents `\usepackage{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Package {
    pub pkg: String,
}

impl Package {
    pub fn new(pkg: String) -> Self {
        Self { pkg }
    }
}

/// Represents `\part{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Part {
    pub name: String,
}

impl Part {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Represents `\chapter{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Chapter {
    pub name: String,
}

impl Chapter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Represents `\section{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Header {
    pub name: String,
    pub header_level: u8,
}

impl Header {
    pub fn new(name: String, header_level: u8) -> Self {
        Self { name, header_level }
    }
}

/// Represents `\<text type>{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Text {
    pub content: String,
    pub type_: TextType,
}

impl Text {
    pub fn new(content: String, type_: TextType) -> Self {
        Self { content, type_ }
    }
}

/// Represents `\paragraph{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Paragraph {
    pub content: String,
}

impl Paragraph {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

/// Represents `\begin{list type} ... \end{list type}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct List {
    pub type_: ListType,
    pub items: Vec<Item>,
}

impl List {
    pub fn new(type_: ListType, items: Vec<Item>) -> Self {
        Self { type_, items }
    }
}

/// Represents `\item{}` in latex
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
