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
    name: String,
    author: String,
    license: String,
    description: String,
    element_list: RefCell<ElementList<Any>>,
}

impl Template {
    /// Creates a new template using metadata (`&Metadata`), license (`&str`) and a description (`&str`)
    pub fn new(metadata: &Metadata, license: &str, desc: &str) -> Self {
        Self {
            name: metadata.title.to_string(),
            author: metadata.author.to_string(),
            license: license.to_string(),
            description: desc.to_string(),
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
    pub fn to_html(&self) -> String {
        let mut html = Vec::new();
        html.push(format!("<h2>{}</h2>", &self.name));
        html.push(format!("<h3>By {}</h3>", &self.author));
        html.push(format!("<h4>Under the {} License</h4>", &self.license));
        html.push(format!("<h5>{}</h5>", &self.description));
        html.push("<code>".to_owned());
        html.push(self.to_latex_string());
        html.push("</code>".to_owned());
        html.join("\n")
    }
    /// Pushes a Template as an entry
    pub fn push_to_map(&self, map: &mut Map) -> Uuid {
        let name = self.name.to_string();
        let json = self.to_json_string();
        let entry = Entry::new(name, json);
        let id = Uuid::new_v4();
        let _ = map.insert(id.clone(), entry);
        id
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
        file.write_all(self.to_html().as_bytes())?;
        Ok(())
    }
}

impl Tex for Template {
    fn to_latex_string(&self) -> String {
        self.element_list.borrow_mut().to_latex_for_html()
    }
}

/// A Hashmap with an entry connected to a unique id
pub type Map = HashMap<Uuid, Entry>;

/// A Template entry that is stored in the archiver
pub struct Entry {
    name: String,
    json: String,
}

impl Entry {
    /// Creates a new Entry using a name and the template as json
    pub fn new(name: String, json: String) -> Self {
        Self {
            name,
            json,
        }
    }
    /// Returns a reference to the name
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    /// Returns a reference to the json template
    pub fn get_json(&self) -> String {
        self.json.to_string()
    }
}