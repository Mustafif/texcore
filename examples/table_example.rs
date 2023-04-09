use std::io::Result;
use std::path::PathBuf;
use texcore::bundle::tables::*;
use texcore::TextType::Normal;
use texcore::{Element, ElementList, Elements, Metadata, Text};

fn rows() -> Vec<Row> {
    let row_els = vec![
        Elements![
            Text::new("foo", Normal),
            Text::new("bar", Normal),
            Text::new("baz", Normal)
        ],
        Elements![
            Text::new("baz", Normal),
            Text::new("foo", Normal),
            Text::new("bar", Normal)
        ],
    ];
    vec_to_rows(row_els)
}

fn columns() -> Vec<Column> {
    vec![
        Column::new(Position::Left, Separator::Single),
        Column::new(Position::Centered, Separator::Single),
        Column::new(Position::Right, Separator::Single),
    ]
}

fn main() -> Result<()> {
    let table = Table::new(None, columns(), rows(), false, false);
    let mut list = ElementList::new(&Metadata::default());
    list.push(Element::from(table));
    list.write(PathBuf::from("test.tex"))?;
    Ok(())
}
