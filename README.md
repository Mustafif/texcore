# TexCore

> This project is under the [MIT License](LICENSE)

Texcore is the successor to `tex-rs` and uses linked lists to help walk and sort elements to either
write to tex code or compile to pdf.

To add to your project:

```toml
[dependencies]
texcore = "0.3.0"
```

## Example

```rust
use std::path::PathBuf;
use texcore::{Chapter, ElementList, Header, Part, Text};
use texcore::TextType::Normal;

fn main() {
use texcore::Elements;
let mut list = ElementList::new(
     "Author",
     "date",
     "title",
     11,
     "book",
     false);
 let part = Part::new("part");
 let chapter = Chapter::new("chapter");
 let header1 = Header::new("header1", 1);
 let header2 = Header::new("header2", 2);
 let text = Text::new("text", Normal);
 list.push_array(Elements![part, chapter, header1, header2, text]);
 list.compile(PathBuf::from("test.pdf")).unwrap();
 // Or to write tex file
 // list.write(path1, path2 (Optional), split).unwrap();
}
```
