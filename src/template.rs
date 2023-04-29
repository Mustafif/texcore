use crate::{Any, Element, ElementList, Input, Metadata, Tex};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

use std::io::Result;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

/// A TexCreate-template that will be used to store and create TexCreate projects
#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub version: Version,
    element_list: ElementList<Any>,
}

impl Template {
    /// Creates a new template using metadata (`&Metadata`), license (`&str`) and a description (`&str`)
    pub fn new(name: &str, description: &str, metadata: &Metadata) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: Version::default(),
            element_list: ElementList::new(metadata),
        }
    }
    /// Creates a new Template by deserializing a file using the path
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = read_to_string(path)?;
        Ok(from_str(&content).unwrap())
    }
    /// Creates a new Template by deserializing a string
    pub fn from_string(content: &str) -> Self {
        from_str(content).unwrap()
    }
    /// Serializes a Template into a JSON string
    pub fn to_json_string(&self) -> String {
        to_string_pretty(&self).unwrap()
    }
    /// Returns a split string for a main file and input file
    pub async fn to_latex_split_string(&self, input: Input) -> (String, String) {
        self.element_list.async_latex_split_string(input).await
    }
    /// Pushes an element to the template
    pub fn push_element(&mut self, element: Element<Any>) {
        self.element_list.push(element)
    }
    /// Pushes an array of elements to the template
    pub async fn push_element_array(&mut self, elements: Vec<Element<Any>>) {
        self.element_list.push_array(elements)
    }
    /// Change the metadata
    pub fn change_metadata(&mut self, metadata: Metadata) {
        self.element_list.change_metadata(metadata)
    }
    /// Write the tex files from the template
    pub async fn write_tex_files(
        &self,
        main_path: PathBuf,
        str_path: PathBuf,
        input: Input,
    ) -> Result<()> {
        self.element_list
            .async_write_split(main_path, str_path, input)
            .await?;
        Ok(())
    }
}

impl Tex for Template {
    fn to_latex_string(&self) -> String {
        self.element_list.to_latex_string()
    }
}

/// Semantic versioning for Templates
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Eq)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    /// Creates a new version using a major, minor and patch values
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Increases major version by 1
    pub fn bump_major(&mut self) {
        self.major += 1;
    }
    /// Increases minor version by 1
    pub fn bump_minor(&mut self) {
        self.minor += 1;
    }
    /// Increases patch version by 1
    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }
    /// Sets the version to a specific version
    pub fn set_version(&mut self, major: u8, minor: u8, patch: u8) {
        self.major = major;
        self.minor = minor;
        self.patch = patch;
    }
    /// Returns the version of a `Cargo` project. 
    pub fn cargo_version() -> Self {
        let vers: &str = env!("CARGO_PKG_VERSION");
        Self::from_str(vers.trim()).unwrap()
    }
}


impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(".").collect();
        let version = Version::new(
            split[0].parse()?,
            split[1].parse()?,
            split[2].parse()?,
        );
        Ok(version)
    }
}


/// Creates Template with default `v1.0.0`
impl Default for Version {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = format!("v{}.{}.{}", self.major, self.minor, self.patch);
        f.write_str(&s)
    }
}


#[test]
fn test_version() {
    let vers_str = "1.0.0";
    let version = Version::from_str(vers_str).unwrap();
    let expected = Version::new(1, 0, 0);
    assert_eq!(version, expected)
}

#[test]
fn test_ge_le_version() {
    let ex = Version::new(3, 0, 0);
    let ex2 = Version::new(3, 1, 0);
    assert_eq!(ex2 > ex, true)
}