use crate::*;
use std::path::PathBuf;
use texcore_traits::{ExtraOptions, Options};
macro_rules! testing {
    ($($func: item)+) => {
        $(
        #[test]
        $func
        )+
    };
}


testing! {
    fn test_extra_ops(){
        let mut chapter = Chapter::new("A chapter");
        let mut env = Environment::new("something");
        let mut header = Header::new("A header", 1);
        let mut input = Input::new(PathBuf::from("foo"), Level::Document);
        let mut pkg = Package::new("bar");
        let mut par = Paragraph::new("A paragraph");
        let mut part = Part::new("A part");

        let options = vec![Options::Curly("An option".to_string())];
        chapter.modify_element(options.clone());
        env.modify_element(options.clone());
        header.modify_element(options.clone());
        input.modify_element(options.clone());
        pkg.modify_element(options.clone());
        par.modify_element(options.clone());
        part.modify_element(options.clone());

        let expected_latex = vec![
            r"\chapter{A chapter}{An option}",
            "\\begin{something}{An option}\n\n\\end{something}",
            r"\section{A header}{An option}",
            r"\input{foo}{An option}",
            r"\usepackage{bar}{An option}",
            r"\paragraph{A paragraph}{An option}",
            r"\part{A part}{An option}"
        ];

        // assert each type
        assert_eq!(&chapter.latex, expected_latex[0]);
        assert_eq!(&env.latex, expected_latex[1]);
        assert_eq!(&header.latex, expected_latex[2]);
        assert_eq!(&input.latex, expected_latex[3]);
        assert_eq!(&pkg.latex, expected_latex[4]);
        assert_eq!(&par.latex, expected_latex[5]);
        assert_eq!(&part.latex, expected_latex[6]);
    }
}