# TexCore

> This project is under the [MIT License](LICENSE)

Texcore is the successor to `tex-rs` and uses linked lists to help walk and sort elements to either
write to tex code or compile to pdf.

To add to your project:

```toml
[dependencies]
texcore = "0.2.0"
```

## Example

```rust
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
        true,
    );
    let part = Part::new("part".to_string());
    list.push(part.into());
    let chapter = Chapter::new("chapter".to_string());
    list.push(chapter.into());
    let header1 = Header::new("header1".to_string(), 1);
    list.push(header1.into());
    let header2 = Header::new("header2".to_string(), 2);
    list.push(header2.into());
    let text = Text::new("text".to_string(), Normal);
    list.push(text.into());
    let pkg = Package::new("dramatist".to_string());
    list.push(pkg.into());
    // To compile:
    //list.compile(PathBuf::from("test.pdf")).unwrap();
    // To write tex file:
    // list.write(PathBuf::from("test.tex"), None, false).unwrap();
    // To print:
    // list.cat();
}
```
