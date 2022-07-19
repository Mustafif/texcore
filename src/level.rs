/// Level controls where each element goes
/// - Meta (Metadata like author, doc class, date)
/// - Packages (Where all packages go)
/// - Document (Inside the document)
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Level {
    Meta,
    Packages,
    Document,
}
