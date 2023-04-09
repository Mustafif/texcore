use crate::{Any, Element, Elements, Environment, Package, Tex, Text, TextType};
use texcore_traits::{ExtraOptions, Options};

/// A wrapper over the `Environment` element that provides a better approach to create tables.
#[derive(Debug, Clone)]
pub struct Table {
    rows: Vec<Row>,
    col_pos: Vec<Column>,
    extension: bool,
    op_pkgs: bool,
    width: Option<f64>,
}

impl From<Table> for Element<Any> {
    fn from(value: Table) -> Self {
        let env = value.build_table();
        Self::from(env)
    }
}

impl Table {
    pub fn new(
        width: Option<f64>,
        col_pos: Vec<Column>,
        rows: Vec<Row>,
        extension: bool,
        op_pkgs: bool,
    ) -> Self {
        Self {
            width,
            col_pos,
            rows,
            extension,
            op_pkgs,
        }
    }
    fn get_env(&self) -> Environment {
        if self.extension {
            let name = "tabular*";
            Environment::new(name)
        } else {
            let name = "tabular";
            Environment::new(name)
        }
    }
    fn build_table(&self) -> Environment {
        let mut env = self.get_env();
        for row in &self.rows {
            env.push(Element::from(Text::new(r"\hline", TextType::Normal)));
            env.push(row.to_element())
        }
        let col_opt = Options::Curly(self.col_pos.to_latex_string());
        if self.extension {
            match self.width {
                None => env.modify_element(vec![col_opt]),
                Some(w) => {
                    let s = format!("{w}\\textwidth");
                    let opt = Options::Curly(s);
                    env.modify_element(vec![opt, col_opt])
                }
            }
            env
        } else {
            env.modify_element(vec![Options::Curly(self.col_pos.to_latex_string())]);
            env
        }
    }
    pub fn build(&self) -> Vec<Element<Any>> {
        if self.op_pkgs {
            let arr = Package::new("array");
            let tabx = Package::new("tabularx");
            Elements![arr, tabx, self.build_table()]
        } else {
            Elements![self.build_table()]
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Position {
    /// Left justified
    Left,
    /// Right justified
    Right,
    /// Centered
    Centered,
    /// Paragraph column with text vertically aligned at the top (requires width)
    Paragraph(f64),
    /// Paragraph column with text vertically aligned in the middle (requires width).
    ///
    /// Requires `array` package
    Middle(f64),
    /// Paragraph column with text vertically aligned at the bottom (requires width).
    ///
    /// Requires `array` package
    Bottom(f64),
}

impl Tex for Position {
    fn to_latex_string(&self) -> String {
        match self {
            Position::Left => 'l'.to_string(),
            Position::Right => 'r'.to_string(),
            Position::Centered => 'c'.to_string(),
            Position::Paragraph(w) => {
                format!("p{{{w}}}")
            }
            Position::Middle(w) => {
                format!("m{{{w}}}")
            }
            Position::Bottom(w) => {
                format!("b{{{w}}}")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Separator {
    /// A single vertical line
    Single,
    /// Double vertical lines
    Double,
    /// No vertical lines
    None,
}

impl Tex for Separator {
    fn to_latex_string(&self) -> String {
        match self {
            Separator::Single => '|'.to_string(),
            Separator::Double => "||".to_string(),
            Separator::None => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Column {
    /// Position of the column
    pos: Position,
    /// Places a separator line to the left
    sep: Separator,
}

impl Column {
    pub fn new(pos: Position, sep: Separator) -> Self {
        Self { pos, sep }
    }
}

impl Tex for Column {
    fn to_latex_string(&self) -> String {
        format!(
            "{} {}",
            self.sep.to_latex_string(),
            self.pos.to_latex_string()
        )
    }
}

impl Tex for Vec<Column> {
    fn to_latex_string(&self) -> String {
        let mut s = Vec::new();
        for c in self {
            s.push(c.to_latex_string())
        }
        // temporary fix
        // TODO!: figure out last column separator right side
        s.push("|".to_string());
        s.join(" ")
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    /// Elements inside of the row
    elements: Vec<Element<Any>>,
}

impl Row {
    pub fn new(elements: Vec<Element<Any>>) -> Self {
        Self { elements }
    }
    pub fn to_element(&self) -> Element<Any> {
        let text = Text::new(&self.to_latex_string(), TextType::Normal);
        Element::from(text)
    }
}

impl Tex for Row {
    fn to_latex_string(&self) -> String {
        let mut s = String::new();
        for i in &self.elements {
            s.push_str(&i.latex);
            s.push_str(" & ")
        }
        let _ = s.remove(s.len() - 2);
        s.push_str(r"\\");
        s
    }
}

/// Given a vector of Element arrays, create an array of rows
pub fn vec_to_rows(vec: Vec<Vec<Element<Any>>>) -> Vec<Row> {
    let mut rows = Vec::new();
    for v in vec {
        rows.push(Row::new(v))
    }
    rows
}
