use crate::feature;
use crate::Level::*;
use crate::TextType::*;
use crate::Type::*;
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::linked_list::{Iter, IterMut};
use std::collections::LinkedList;
use std::fs::write;
use std::io::Error;
use std::path::PathBuf;
use texcore_traits::*;
feature! {
    "compile"
    use tectonic::latex_to_pdf;
}
feature! {
    "parallel"
    use rayon::prelude::*;
    use rayon::*;
}
/// Converts a struct to a string
pub trait Tex {
    // turns the element into a latex string
    fn to_latex_string(&self) -> String;
}

impl Tex for Any {
    fn to_latex_string(&self) -> String {
        self.latex.to_string()
    }
}

impl Tex for Environment {
    fn to_latex_string(&self) -> String {
        let begin = format!(r"\begin{{{}}}", &self.name);
        let end = format!(r"\end{{{}}}", &self.name);
        let strings = vec![begin, self.inner_latex_string(), end];
        strings.join("\n")
    }
}

impl Tex for Custom {
    fn to_latex_string(&self) -> String {
        self.latex.to_string()
    }
}

impl Tex for Comment {
    fn to_latex_string(&self) -> String {
        format!("% {}", &self.value)
    }
}

impl Tex for Input {
    fn to_latex_string(&self) -> String {
        let path = self.file_name.to_str();
        match path {
            Some(p) => format!(r"\input{{{p}}}"),
            None => String::new(),
        }
    }
}

impl Tex for Package {
    fn to_latex_string(&self) -> String {
        format!(r"\usepackage{{{}}}", &self.pkg)
    }
}

impl Tex for Part {
    fn to_latex_string(&self) -> String {
        format!(r"\part{{{}}}", &self.name)
    }
}

impl Tex for Chapter {
    fn to_latex_string(&self) -> String {
        format!(r"\chapter{{{}}}", &self.name)
    }
}

impl Tex for Header {
    fn to_latex_string(&self) -> String {
        match self.header_level {
            1 => format!(r"\section{{{}}}", &self.name),
            _ => {
                let mut result = r"\".to_string();
                let mut count = 0;
                while count < self.header_level {
                    result.push_str("sub");
                    count += 1;
                }
                result.push_str(&format!("section{{{}}}", &self.name));
                result
            }
        }
    }
}

impl Tex for Paragraph {
    fn to_latex_string(&self) -> String {
        format!(r"\paragraph{{{}}}", &self.content)
    }
}

impl Tex for Text {
    fn to_latex_string(&self) -> String {
        match &self.type_ {
            Bold => format!(r"\textbf{{{}}}", &self.content),
            Italics => format!(r"\textit{{{}}}", &self.content),
            Normal => self.content.to_string(),
            Math => format!("${}$", &self.content),
            Par => format!(r"\par {{{}}}", &self.content),
        }
    }
}

impl Tex for Item {
    fn to_latex_string(&self) -> String {
        format!(r"\item {{{}}}", &self.name)
    }
}

impl Tex for List {
    fn to_latex_string(&self) -> String {
        let list = match &self.type_ {
            ListType::Itemized => "itemize",
            ListType::Enumerated => "enumerate",
        };
        let begin = format!(r"\begin{{{list}}}");
        let end = format!(r"\end{{{list}}}");

        let mut result = Vec::new();
        result.push(begin);
        for i in &self.items {
            result.push(i.to_latex_string())
        }
        result.push(end);
        result.join("\n")
    }
}

impl Tex for Metadata {
    fn to_latex_string(&self) -> String {
        let doc_class = format!(
            r"\documentclass[{}pt, {}]{{{}}}",
            &self.fontsize, &self.papersize, &self.doc_class
        );
        let title = format!(r"\title{{{}}}", &self.title);
        let author = format!(r"\author{{{}}}", &self.author);
        let date = format!(r"\date{{{}}}", &self.date);
        let result = vec![doc_class, title, author, date];
        result.join("\n")
    }
}

impl From<Part> for Element<Any> {
    fn from(value: Part) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.name,
            type_: T_Part,
            level: Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Chapter> for Element<Any> {
    fn from(value: Chapter) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.name,
            type_: T_Chapter,
            level: Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Header> for Element<Any> {
    fn from(value: Header) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.name,
            type_: T_Header,
            level: Document,
            header_level: Some(value.header_level),
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Paragraph> for Element<Any> {
    fn from(value: Paragraph) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.content,
            type_: T_Paragraph,
            level: Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Text> for Element<Any> {
    fn from(value: Text) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.content,
            type_: T_Text,
            level: Document,
            header_level: None,
            text_type: Some(value.type_),
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Package> for Element<Any> {
    fn from(value: Package) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.pkg,
            type_: T_Package,
            level: Packages,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Input> for Element<Any> {
    fn from(value: Input) -> Self {
        let latex = if value.modified {
            value.latex.to_string()
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.file_name_str(),
            type_: T_Input,
            level: value.level,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Environment> for Element<Any> {
    fn from(value: Environment) -> Self {
        let latex = if value.modified {
            value.latex
        } else {
            value.to_latex_string()
        };
        let any = Any {
            value: value.name,
            type_: T_Environment,
            level: Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: Some(value.elements),
            latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Custom> for Element<Any> {
    fn from(value: Custom) -> Self {
        let any = Any {
            value: value.latex.to_string(),
            type_: T_Custom,
            level: value.level,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex: value.latex,
            modified: value.modified,
        };
        Element::new_any(any)
    }
}

impl From<Comment> for Element<Any> {
    fn from(value: Comment) -> Self {
        let latex = value.to_latex_string();
        let any = Any {
            value: value.value,
            type_: T_Custom,
            level: value.level,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            latex,
            modified: false,
        };
        Element::new_any(any)
    }
}

/// A latex element
#[derive(PartialOrd, PartialEq, Clone, Debug, Deserialize, Serialize, ExtraOps)]
pub struct Element<T: Tex> {
    pub(crate) value: T,
    pub(crate) type_: Type,
    pub(crate) level: Level,
    pub(crate) latex: String,
    pub(crate) modified: bool,
}

impl<T: Tex> Element<T> {
    /// Creates a new Element
    pub fn new(value: T, type_: Type, level: Level, latex: String) -> Self {
        Self {
            value,
            type_,
            level,
            latex,
            modified: false,
        }
    }
}

impl Element<Any> {
    /// Creates a new `Element<Any>`
    pub fn new_any(value: Any) -> Self {
        let type_ = value.type_;
        let level = value.level;
        let latex = value.latex.to_string();
        let modified = value.modified;
        Self {
            value,
            type_,
            level,
            latex,
            modified,
        }
    }
}

/// A linked list of elements
#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq)]
pub struct ElementList<T: Tex> {
    metadata: Metadata,
    list: LinkedList<Element<T>>,
}

impl ElementList<Any> {
    /// Creates a new empty list
    pub fn new(metadata: &Metadata) -> Self {
        Self {
            metadata: metadata.to_owned(),
            list: LinkedList::new(),
        }
    }
    /// A forward iterator of elements in the list
    pub fn iter(&self) -> Iter<'_, Element<Any>> {
        self.list.iter()
    }
    /// A mutable forward iterator of elements in the list
    pub fn iter_mut(&mut self) -> IterMut<'_, Element<Any>> {
        self.list.iter_mut()
    }
    /// Changes the metadata
    pub fn change_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata
    }
    /// Pushes a vector of Packages as String
    pub fn push_packages(&mut self, packages: Vec<String>) {
        for p in packages {
            let package = Package::new(&p);
            self.push(Element::from(package))
        }
    }
    /// Adds in `\newpage` text as next element in the list
    pub fn add_newpage(&mut self) {
        let text = Text::new(r"\newpage", Normal);
        self.push(Element::from(text));
    }
    /// Pushes an element to the end of the list
    pub fn push(&mut self, element: Element<Any>) {
        self.list.push_back(element)
    }
    /// Pushes an element vector into the list
    pub fn push_array(&mut self, element_vec: Vec<Element<Any>>) {
        for element in element_vec {
            self.push(element)
        }
    }
    /// Pops an element at the end of the list
    pub fn pop(&mut self) -> Option<Element<Any>> {
        self.list.pop_back()
    }
    /// Pushes element to the front of the list
    pub fn fpush(&mut self, element: Element<Any>) {
        self.list.push_front(element)
    }
    /// Pops an element in the front of the list
    pub fn fpop(&mut self) -> Option<Element<Any>> {
        self.list.pop_front()
    }

    /// Walks the list and returns a combined latex string
    pub fn to_latex_string(&self) -> String {
        let mut meta = Vec::new();
        let mut packages = Vec::new();
        let mut document = Vec::new();
        meta.push(self.metadata.to_latex_string());
        document.push(r"\begin{document}".to_owned());
        if self.metadata.maketitle {
            document.push(r"\maketitle".to_owned());
        }
        for i in self.iter() {
            iter_push(i, &mut document, &mut packages, &mut meta)
        }
        document.push(r"\end{document}".to_owned());
        let result = vec![meta.join("\n"), packages.join("\n"), document.join("\n")];
        result.join("\n")
    }
    /// Walks the list and returns a split latex string separating Packages level
    pub fn to_latex_split_string(&self, input: Input) -> (String, String) {
        let mut meta = Vec::new();
        meta.push(self.metadata.to_latex_string());
        meta.push(input.to_latex_string());
        let mut packages = Vec::new();
        let mut document = Vec::new();
        document.push(r"\begin{document}".to_owned());
        if self.metadata.maketitle {
            document.push(r"\maketitle".to_owned());
        }
        for i in self.iter() {
            iter_push(i, &mut document, &mut packages, &mut meta)
        }
        document.push(r"\end{document}".to_owned());
        let result = vec![meta.join("\n"), document.join("\n")];
        (result.join("\n"), packages.join("\n"))
    }
    /// Writes `ElementList` into a latex file
    pub fn write(&self, main: PathBuf) -> Result<(), Error> {
        let latex = self.to_latex_string();
        write_file(main, latex.as_bytes())?;
        Ok(())
    }
    /// Writes `ElementList` into two latex files splitting the `main` content and `path` for packages
    /// Input is used to declare the appropriate `\input{}` for your package file
    pub fn write_split(&self, main: PathBuf, structure: PathBuf, input: Input) {
        let (main_tex, str_tex) = self.to_latex_split_string(input);
        std::thread::scope(|s| {
            s.spawn(move || {
                write_file(main, main_tex.as_bytes()).expect("Couldn't write main file")
            });
            s.spawn(move || {
                write_file(structure, str_tex.as_bytes()).expect("Couldn't write structure file")
            });
        })
    }
    feature! {
       "parallel"
        /// A parallel alternate to `write()`
        pub fn par_write(&self, main: PathBuf){
            let pool = ThreadPoolBuilder::default().build().expect("Couldn't build pool");
            let latex = pool.install(|| self.to_latex_string());
            pool.install(|| write_file(main, latex.as_bytes()).expect("Couldn't write latex file in pool"));
        }
        /// A parallel alternate to `write_split()`
        pub fn par_write_split(&self, main: PathBuf, structure: PathBuf, input: Input){
            let pool = ThreadPoolBuilder::default().build().expect("Couldn't build pool");
            let (main_tex, str_tex) = pool.install(|| self.to_latex_split_string(input));
            pool.join(
                || write_file(main, main_tex.as_bytes()).expect("Couldn't write main file in pool"),
                || write_file(structure, str_tex.as_bytes()).expect("Couldn't write structure file in pool")
            );
        }
        pub fn par_iter(&self) -> impl ParallelIterator<Item=&Element<Any>> {
            use rayon::prelude::*;
            self.list.par_iter()
        }
    }

    #[cfg(feature = "compile")]
    /// Compiles the list into a pdf file
    pub fn compile(&self, path: PathBuf) -> Result<(), Error> {
        use std::fs::File;
        let mut file = File::create(path)?;
        let latex = self.to_latex_string();
        let pdf = latex_to_pdf(latex)?;
        file.write_all(&pdf)?;
        Ok(())
    }
    /// Prints the whole tex source code
    pub fn print_tex(&mut self) {
        println!("{}", self.to_latex_string());
    }
    /// Returns &self.list to `Vec<Element<Any>>`
    pub fn list_to_array(&self) -> Vec<Element<Any>> {
        let mut vec = Vec::new();
        for element in &self.list {
            vec.push(element.to_owned())
        }
        vec
    }
    /// returns the list's metadata
    pub fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }
    /// returns a reference to the inner linked list
    pub fn list(&self) -> &LinkedList<Element<Any>> {
        &self.list
    }
}

impl Default for ElementList<Any> {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            list: LinkedList::new(),
        }
    }
}

// A helper function to write bytes to a file
fn write_file(path: PathBuf, bytes: &[u8]) -> Result<(), Error> {
    write(path, bytes)?;
    Ok(())
}

fn iter_push(
    i: &Element<Any>,
    document: &mut Vec<String>,
    packages: &mut Vec<String>,
    meta: &mut Vec<String>,
) {
    let latex = i.latex.to_string();
    match i.level {
        Document => document.push(latex),
        Packages => packages.push(latex),
        Meta => meta.push(latex),
    }
}
