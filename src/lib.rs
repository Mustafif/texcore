#![feature(doc_auto_cfg)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! # Examples
//! ```rust
#![doc = include_str ! ("../examples/doc_example.rs")]
//! ```
/// Element controls everything related to `Elements` and `ElementList`
pub mod element;
#[cfg(feature = "async")]
/// Provides asynchronous methods using the type `TexAsync`
pub mod future;
/// Level controls where each element goes
/// - Meta (Metadata like author, doc class, date)
/// - Packages (Where all packages go)
/// - Document (Inside the `\begin{document}` and `\end{document}`)
pub mod level;

/// Macro APIS that are used around the library to provide easier
/// development with features separation.
#[allow(unused_macros)]
#[macro_use]
pub(crate) mod macros;
/// Provide bundles of `Element`s for different common use cases
pub mod bundle;
#[cfg(feature = "texcreate_template")]
/// Provides the `Template` type for the TexCreate project
pub mod template;
/// Type controls the different kinds of latex elements
pub mod type_;
// Testing all in a single module
#[cfg(test)]
mod tests;

pub use element::*;
pub use level::*;
pub use type_::*;

feature! {
    "compile"
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
}

/// returns a vector of `Element<Any>`
#[allow(non_snake_case)]
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
