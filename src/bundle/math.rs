use crate::{Any, Element, Environment, Level, Tex, Type};
use serde::{Deserialize, Serialize};

/// Greek letters symbols
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Greek {
    Alpha(Case),
    Beta(Case),
    Gamma(Case),
    Delta(Case),
    Epsilon(Case),
    VarEpsilon,
    Zeta(Case),
    Eta(Case),
    Theta(Case),
    VarTheta,
    Iota(Case),
    Kappa(Case),
    Lambda(Case),
    Mu(Case),
    Nu(Case),
    Xi(Case),
    O(Case),
    Pi(Case),
    Rho(Case),
    VarRho,
    Sigma(Case),
    Tau(Case),
    Upsilon(Case),
    Phi(Case),
    VarPhi,
    Chi(Case),
    Psi(Case),
    Omega(Case),
}

impl From<Greek> for Element<Any> {
    fn from(value: Greek) -> Self {
        let latex = value.to_latex_string();
        let any = Any {
            value: String::new(),
            latex,
            type_: Type::T_Bundle,
            level: Level::Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            modified: false,
        };
        Element::new_any(any)
    }
}

impl Tex for Greek {
    fn to_latex_string(&self) -> String {
        match self {
            Greek::Alpha(c) => match c {
                Case::Lower => "\\alpha",
                Case::Upper => "A",
            }
                .to_string(),
            Greek::Beta(c) => match c {
                Case::Lower => "\\beta",
                Case::Upper => "B",
            }
                .to_string(),
            Greek::Gamma(c) => match c {
                Case::Lower => "\\gamma",
                Case::Upper => "\\Gamma",
            }
                .to_string(),
            Greek::Delta(c) => match c {
                Case::Lower => "\\delta",
                Case::Upper => "\\Delta",
            }
                .to_string(),
            Greek::Epsilon(c) => match c {
                Case::Lower => "\\epsilon",
                Case::Upper => "E",
            }
                .to_string(),
            Greek::VarEpsilon => "\\varepsilon".to_string(),
            Greek::Zeta(c) => match c {
                Case::Lower => "\\zeta",
                Case::Upper => "Z",
            }
                .to_string(),
            Greek::Eta(c) => match c {
                Case::Lower => "\\eta",
                Case::Upper => "H",
            }
                .to_string(),
            Greek::Theta(c) => match c {
                Case::Lower => "\\theta",
                Case::Upper => "\\Theta",
            }
                .to_string(),
            Greek::VarTheta => "\\vartheta".to_string(),
            Greek::Iota(c) => match c {
                Case::Lower => "\\iota",
                Case::Upper => "I",
            }
                .to_string(),
            Greek::Kappa(c) => match c {
                Case::Lower => "\\kappa",
                Case::Upper => "K",
            }
                .to_string(),
            Greek::Lambda(c) => match c {
                Case::Lower => "\\lambda",
                Case::Upper => "\\Lambda",
            }
                .to_string(),
            Greek::Mu(c) => match c {
                Case::Lower => "\\mu",
                Case::Upper => "M",
            }
                .to_string(),
            Greek::Nu(c) => match c {
                Case::Lower => "\\nu",
                Case::Upper => "N",
            }
                .to_string(),
            Greek::Xi(c) => match c {
                Case::Lower => "\\xi",
                Case::Upper => "\\Xi",
            }
                .to_string(),
            Greek::O(c) => match c {
                Case::Lower => "o",
                Case::Upper => "O",
            }
                .to_string(),
            Greek::Pi(c) => match c {
                Case::Lower => "\\pi",
                Case::Upper => "\\Pi",
            }
                .to_string(),
            Greek::Rho(c) => match c {
                Case::Lower => "\\rho",
                Case::Upper => "P",
            }
                .to_string(),
            Greek::VarRho => "\\varrho".to_string(),
            Greek::Sigma(c) => match c {
                Case::Lower => "\\sigma",
                Case::Upper => "\\Sigma",
            }
                .to_string(),
            Greek::Tau(c) => match c {
                Case::Lower => "\\tau",
                Case::Upper => "T",
            }
                .to_string(),
            Greek::Upsilon(c) => match c {
                Case::Lower => "\\upsilon",
                Case::Upper => "\\Upsilon",
            }
                .to_string(),
            Greek::Phi(c) => match c {
                Case::Lower => "\\phi",
                Case::Upper => "\\Phi",
            }
                .to_string(),
            Greek::VarPhi => "\\varphi".to_string(),
            Greek::Chi(c) => match c {
                Case::Lower => "\\chi",
                Case::Upper => "X",
            }
                .to_string(),
            Greek::Psi(c) => match c {
                Case::Lower => "\\psi",
                Case::Upper => "\\Psi",
            }
                .to_string(),
            Greek::Omega(c) => match c {
                Case::Lower => "\\omega",
                Case::Upper => "\\Omega",
            }
                .to_string(),
        }
    }
}

/// Arrow symbols
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Arrows {
    LeftArrow(Case),
    RightArrow(Case),
    LeftRightArrow(Case),
    RightLeftHarpoons,
    UpArrow(Case),
    DownArrow(Case),
    UpdownArrow,
    Mapsto,
    LongMapsto,
    Nearrow,
    Searrow,
    Swarrow,
    Nwarrow,
    LeftHarpoonUp,
    LeftHarpoonDown,
    RightHarpoonUp,
    RightHarpoonDown,
}

impl From<Arrows> for Element<Any> {
    fn from(value: Arrows) -> Self {
        let latex = value.to_latex_string();
        let any = Any {
            value: String::new(),
            latex,
            type_: Type::T_Bundle,
            level: Level::Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            modified: false,
        };
        Element::new_any(any)
    }
}

impl Tex for Arrows {
    fn to_latex_string(&self) -> String {
        match self {
            Arrows::LeftArrow(c) => match c {
                Case::Lower => "\\leftarrow",
                Case::Upper => "\\Leftarrow",
            }
                .to_string(),
            Arrows::RightArrow(c) => match c {
                Case::Lower => "\\rightarrow",
                Case::Upper => "\\Rightarrow",
            }
                .to_string(),
            Arrows::LeftRightArrow(c) => match c {
                Case::Lower => "\\leftrightarrow",
                Case::Upper => "\\Leftrightarrow",
            }
                .to_string(),
            Arrows::RightLeftHarpoons => "\\rightleftharpoons".to_string(),
            Arrows::UpArrow(c) => match c {
                Case::Lower => "\\uparrow",
                Case::Upper => "\\Uparrow",
            }
                .to_string(),
            Arrows::DownArrow(c) => match c {
                Case::Lower => "\\downarrow",
                Case::Upper => "\\Downarrow",
            }
                .to_string(),
            Arrows::UpdownArrow => "\\Updownarrow".to_string(),
            Arrows::Mapsto => "\\mapsto".to_string(),
            Arrows::LongMapsto => "\\longmapsto".to_string(),
            Arrows::Nearrow => "\\nearrow".to_string(),
            Arrows::Searrow => "\\searrow".to_string(),
            Arrows::Swarrow => "\\swarrow".to_string(),
            Arrows::Nwarrow => "\\nwarrow".to_string(),
            Arrows::LeftHarpoonUp => "\\leftharpoonup".to_string(),
            Arrows::LeftHarpoonDown => "\\leftharpoondown".to_string(),
            Arrows::RightHarpoonUp => "\\rightharpoonup".to_string(),
            Arrows::RightHarpoonDown => "\\rightharpoondown".to_string(),
        }
    }
}

/// Miscellaneous math symbols
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Misc {
    Infty,
    ForAll,
    Re,
    Im,
    Nabla,
    Exists,
    NExists,
    Partial,
    EmptySet,
    VarNothing,
    Wp,
    Complement,
    Neg,
    CDots,
    Square,
    Surd,
    BlackSquare,
    Triangle,
}

impl Tex for Misc {
    fn to_latex_string(&self) -> String {
        match self {
            Misc::Infty => "\\infty".to_string(),
            Misc::ForAll => "\\forall".to_string(),
            Misc::Re => "\\Re".to_string(),
            Misc::Im => "\\Im".to_string(),
            Misc::Nabla => "\\nabla".to_string(),
            Misc::Exists => "\\exists".to_string(),
            Misc::NExists => "\\nexists".to_string(),
            Misc::Partial => "\\partial".to_string(),
            Misc::EmptySet => "\\emptyset".to_string(),
            Misc::VarNothing => "\\varnothing".to_string(),
            Misc::Wp => "\\wp".to_string(),
            Misc::Complement => "\\complement".to_string(),
            Misc::Neg => "\\neg".to_string(),
            Misc::CDots => "\\cdots".to_string(),
            Misc::Square => "\\square".to_string(),
            Misc::Surd => "\\surd".to_string(),
            Misc::BlackSquare => "\\blacksquare".to_string(),
            Misc::Triangle => "\\triangle".to_string(),
        }
    }
}

/// Binary operation/relations symbols
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Binary {
    Times,
    Div,
    CDot,
    Cap,
    Cup,
    Neq,
    Leq,
    Geq,
    In,
    Perp,
    Nothing,
    Subset,
    Simeq,
    Approx,
    Wegde,
    Vee,
    OPlus,
    OTimes,
    Box,
    BoxTimes,
    Equiv,
    Cong,
}

impl From<Binary> for Element<Any> {
    fn from(value: Binary) -> Self {
        let latex = value.to_latex_string();
        let any = Any {
            value: String::new(),
            latex,
            type_: Type::T_Bundle,
            level: Level::Document,
            header_level: None,
            text_type: None,
            list_type: None,
            items: None,
            elements: None,
            modified: false,
        };
        Element::new_any(any)
    }
}

impl Tex for Binary {
    fn to_latex_string(&self) -> String {
        match self {
            Binary::Times => "\\times".to_string(),
            Binary::Div => "\\div".to_string(),
            Binary::CDot => "\\cdot".to_string(),
            Binary::Cap => "\\cap".to_string(),
            Binary::Cup => "\\cup".to_string(),
            Binary::Neq => "\\neq".to_string(),
            Binary::Leq => "\\leq".to_string(),
            Binary::Geq => "\\geq".to_string(),
            Binary::In => "\\in".to_string(),
            Binary::Perp => "\\perp".to_string(),
            Binary::Nothing => "\\nothing".to_string(),
            Binary::Subset => "\\subset".to_string(),
            Binary::Simeq => "\\simeq".to_string(),
            Binary::Approx => "\\approx".to_string(),
            Binary::Wegde => "\\wedge".to_string(),
            Binary::Vee => "\\vee".to_string(),
            Binary::OPlus => "\\oplus".to_string(),
            Binary::OTimes => "\\otimes".to_string(),
            Binary::Box => "\\box".to_string(),
            Binary::BoxTimes => "\\boxtimes".to_string(),
            Binary::Equiv => "\\equiv".to_string(),
            Binary::Cong => "\\cong".to_string(),
        }
    }
}

/// Upper or lower case
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Case {
    Upper,
    Lower,
}

/// Provides the `equation` environment.
pub fn equation_env(altered: bool) -> Environment {
    if altered {
        Environment::new("equation*")
    } else {
        Environment::new("equation")
    }
}

/// Provides the `equation` environment with a nested `split` environment with your array of equations.
///
/// > Make sure these equations are `TextType::Normal` and contain a newline except for the last line
pub fn equation_split_env(altered: bool, elements: Vec<Element<Any>>) -> Environment {
    let mut eq = equation_env(altered);
    let mut split_env = Environment::new("split");
    split_env.set_elements(elements);
    eq.push(Element::from(split_env));
    eq
}
