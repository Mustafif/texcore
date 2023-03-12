use std::io::Error;
use std::path::PathBuf;

use texcore::Elements;
use texcore::*;

fn main() -> Result<(), Error> {
    let metadata = Metadata::new("Author", "date", "title", 11, "book", "article", true);
    let mut list = ElementList::new(&metadata);
    let part = Part::new("part");
    let chapter = Chapter::new("chapter");
    let header1 = Header::new("header1", 1);
    let header2 = Header::new("header2", 2);
    let text = Text::new("text", TextType::Normal);
    let pkg = Package::new("dramatist");
    list.push_array(Elements![part, chapter, header1, header2, text, pkg]);
    // To compile, use the `compile` feature
    // list.compile(PathBuf::from("test.pdf"))?;
    // To write to single tex file:
    list.write(PathBuf::from("test.tex"))?;
    // To split write Packages and Main file:
    let input = Input::new(PathBuf::from("structure"), Some(Level::Meta));
    list.write_split(
        PathBuf::from("main.tex"),
        PathBuf::from("structure.tex"),
        input,
    );
    // To print:
    list.print_tex();
    Ok(())
}
