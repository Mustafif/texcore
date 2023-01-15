use crate::{Any, compile, Element, ElementList, Metadata, Tex};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::{Result, Write};
use std::path::PathBuf;
use uuid::Uuid;

/// A TexCreate-template that will be used to store and create TexCreate projects
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub version: Version,
    element_list: RefCell<ElementList<Any>>,
}

impl Template {
    /// Creates a new template using metadata (`&Metadata`), license (`&str`) and a description (`&str`)
    pub fn new(name: &str, description: &str, metadata: &Metadata) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: Version::new(),
            element_list: RefCell::new(ElementList::new(metadata)),
        }
    }
    /// Creates a new Template by deserializing a file using the path
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = read_to_string(path)?;
        Ok(from_str(&content).unwrap())
    }
    /// Creates a new Template by deserializing a string
    pub fn from_string(content: &str) -> Self {
        from_str(&content).unwrap()
    }
    /// Serializes a Template into a JSON string
    pub fn to_json_string(&self) -> String {
        to_string_pretty(&self).unwrap()
    }
    /// Returns the details of the template as HTML code
    pub fn to_latex_for_html(&self) -> String {
        self.element_list.borrow_mut().to_latex_for_html()
    }
    /// Pushes an element to the template
    pub fn push_element(&self, element: Element<Any>) {
        self.element_list.borrow_mut().push(element);
    }
    /// Pushes an array of elements to the template
    pub fn push_element_array(&self, elements: Vec<Element<Any>>) {
        self.element_list.borrow_mut().push_array(elements);
    }
    /// Write the tex files from the template
    fn write_tex_files(&self, main_path: PathBuf, str_path: PathBuf) -> Result<()> {
        self.element_list.borrow_mut().write_split(main_path, str_path)?;
        Ok(())
    }
    /// Writes then compiles the document
    pub fn write_then_compile(&self, main_path: PathBuf, str_path: PathBuf, pdf_path: PathBuf) -> Result<()> {
        self.write_tex_files(main_path.clone(), str_path)?;
        compile(main_path, pdf_path)?;
        Ok(())
    }
    /// Writes the Template as an HTML file
    pub fn write_as_html(&self, html_path: PathBuf) -> Result<()> {
        let mut file = File::create(html_path)?;
        file.write_all(self.to_latex_for_html().as_bytes())?;
        Ok(())
    }
}

impl Tex for Template {
    fn to_latex_string(&self) -> String {
        self.element_list.borrow_mut().to_latex_string()
    }
}

// Semantic versioning for Templates
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    /// Creates Template with default `v1.0.0`
    pub fn new() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
    pub fn bump_major(&mut self) {
        self.major += 1;
    }
    pub fn bump_minor(&mut self) {
        self.minor += 1;
    }
    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }
    pub fn set_version(&mut self, major: u8, minor: u8, patch: u8) {
        self.major = major;
        self.minor = minor;
        self.patch = patch;
    }
    pub fn to_string(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}
