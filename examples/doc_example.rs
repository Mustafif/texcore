use std::path::PathBuf;
use texcore::TextType::Normal;
use texcore::{Chapter, ElementList, Header, Package, Part, Text};

fn main() {
    let mut list = ElementList::new(
        "Author",
        "date",
        "title",
        11,
        "book",
        true,
    );
    let part = Part::new("part");
    list.push(part.into());
    let chapter = Chapter::new("chapter");
    list.push(chapter.into());
    let header1 = Header::new("header1", 1);
    list.push(header1.into());
    let header2 = Header::new("header2", 2);
    list.push(header2.into());
    let text = Text::new("text", Normal);
    list.push(text.into());
    let pkg = Package::new("dramatist");
    list.push(pkg.into());
    // To compile:
    //list.compile(PathBuf::from("test.pdf")).unwrap();
    // To write tex file:
    // list.write(PathBuf::from("test.tex"), None, false).unwrap();
    // To print:
    // list.cat();
}
