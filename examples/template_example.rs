use std::path::PathBuf;
use std::io::Result;
use texcore::{Element, Metadata, Part};
use texcore::template::Template;

static NAME: &str = "Name";
static AUTHOR: &str = "Author";
static LICENSE: &str = "MIT";
static DESCRIPTION: &str = "A Description";

fn metadata() -> Metadata {
    Metadata::new(AUTHOR, "date", NAME, 11, "article", false)
}

fn main() -> Result<()> {
    let template = Template::new(&metadata(), LICENSE, DESCRIPTION);
    let part = Part::new("name");
    template.push_element(Element::from(part));
    //template.write_then_compile(PathBuf::from("main.tex"), PathBuf::from("structure.tex"), PathBuf::from("main.pdf"))?;
    template.write_as_html(PathBuf::from("test.html"))?;
    Ok(())
}