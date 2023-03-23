use std::path::PathBuf;
// This example utilizes the `async` feature
use texcore::Level::Meta;
use texcore::{Chapter, ElementList, Elements, Input, Metadata, Part};
use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let part = Part::new("part 1");
    let chapter = Chapter::new("chapter 1");
    // for simplicity we will use default metadata
    let mut metadata = Metadata::default();
    // we need to make sure document class is book
    metadata.doc_class = "book".to_string();

    // push elements to list
    let mut list = ElementList::new(&metadata);
    list.push_array(Elements![part, chapter]);

    // to write single source tex file use `async_write()`
    list.async_write(PathBuf::from("main.tex")).await?;

    // to write split file where packages are under a seperate file
    // use `async_split_write()`
    // these files are written in parallel
    list.async_write_split(
        PathBuf::from("main.tex"),
        PathBuf::from("structure.tex"),
        Input::new(PathBuf::from("structure"), Meta),
    )
        .await?;
    Ok(())
}
