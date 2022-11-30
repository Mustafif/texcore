//! # Examples
//!
//! ```
//! use std::path::PathBuf;
//! use texcore::{Chapter, ElementList, Header, Part, Text};
//! use texcore::TextType::Normal;
//!
//! fn main() {
//! use texcore::Elements;
//! let mut list = ElementList::new(
//!     "Author",
//!     "date",
//!     "title",
//!     11,
//!     "book",
//!     false);
//! let part = Part::new("part");
//! let chapter = Chapter::new("chapter");
//! let header1 = Header::new("header1", 1);
//! let header2 = Header::new("header2", 2);
//! let text = Text::new("text", Normal);
//! list.push_array(Elements![part, chapter, header1, header2, text]);
//! list.compile(PathBuf::from("test.pdf")).unwrap();
//! // Or to write tex file
//! // list.write(path1, path2 (Optional), split).unwrap();
//! }
//! ```
/// Element controls everything related to `Elements` and `ElementList`
pub mod element;
/// Level controls where each element goes
/// - Meta (Metadata like author, doc class, date)
/// - Packages (Where all packages go)
/// - Document (Inside the `\begin{document}` and `\end{document}`)
pub mod level;
/// Type controls the different kinds of latex elements
pub mod type_;

pub use element::*;
pub use level::*;
use std::io::{Error, Write};
use std::path::PathBuf;
use tectonic::latex_to_pdf;
pub use type_::*;

/// Compiles a tex file to a pdf file
pub fn compile(path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    let source = std::fs::read_to_string(path)?;
    let pdf = latex_to_pdf(source)?;
    let mut output = std::fs::File::create(output_path)?;
    output.write_all(&pdf)?;
    Ok(())
}
/// returns a vector of Element<Any>
#[macro_export]
macro_rules! Elements {
    ($($element: expr), *) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($element.into());
            )*
            vec
        }
    };
}

#[test]
fn test_elements_macro() {
    let part = Part::new("part1");
    let chapter = Chapter::new("chatper1");
    let mut list1 = ElementList::new(
        "Author", "date", "title", 11, "article", false,
    );
    list1.push(part.clone().into());
    list1.push(chapter.clone().into());
    let mut list2 = ElementList::new(
        "Author", "date", "title", 11, "article", false,
    );
    let elem_vec = Elements![part, chapter];
    list2.push_array(elem_vec);
    assert_eq!(list1, list2)
}