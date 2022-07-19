use std::path::PathBuf;
use texcore::TextType::Normal;
use texcore::{Chapter, ElementList, Header, Package, Part, Text};

fn main() {
    let mut list = ElementList::new(
        "Author".to_string(),
        "date".to_string(),
        "title".to_string(),
        11,
        "book".to_string(),
        true
    );
    let part = Part::new("part".to_string());
    let chapter = Chapter::new("chapter".to_string());
    let header1 = Header::new("header1".to_string(), 1);
    let header2 = Header::new("header2".to_string(), 2);
    let text = Text::new("text".to_string(), Normal);
    let pkg = Package::new("dramatist".to_string());
    list.push(part.into());
    list.push(chapter.into());
    list.push(header1.into());
    list.push(header2.into());
    list.push(text.into());
    list.push(pkg.into());
    // To compile:
    // list.compile(PathBuf::from("test.pdf")).unwrap();
    // To write tex file:
    // list.write(PathBuf::from("test.tex"), None, false).unwrap();
    // To print:
    // list.cat();
}
