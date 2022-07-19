//! # Examples
//!
//! ```
//! use std::path::PathBuf;
//! use texcore::{Chapter, ElementList, Header, Part, Text};
//! use texcore::TextType::Normal;
//!
//! fn main() {
//! let mut list = ElementList::new(
//!     "Author".to_string(),
//!     "date".to_string(),
//!     "title".to_string(),
//!     11,
//!     "book".to_string());
//! let part = Part::new("part".to_string());
//! let chapter = Chapter::new("chapter".to_string());
//! let header1 = Header::new("header1".to_string(), 1);
//! let header2 = Header::new("header2".to_string(), 2);
//! let text = Text::new("text".to_string(), Normal);
//! list.push(part.into());
//! list.push(chapter.into());
//! list.push(header1.into());
//! list.push(header2.into());
//! list.push(text.into());
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
pub use type_::*;
use std::io::{Error, Write};
use std::path::PathBuf;
use tectonic::latex_to_pdf;

/// Compiles a tex file to a pdf file
pub fn compile(path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    let source = std::fs::read_to_string(path)?;
    let pdf = latex_to_pdf(source)?;
    let mut output = std::fs::File::create(output_path)?;
    output.write_all(&pdf)?;
    Ok(())
}
