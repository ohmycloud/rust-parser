use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Metadata {
        key: &'a str,
        value: &'a str,
    },
    Ingredient {
        name: &'a str,
        quantity: Option<&'a str>,
        unit: Option<&'a str>,
    },
    RecipeRef {
        name: &'a str,
        quantity: Option<&'a str>,
        unit: Option<&'a str>,
    },
    Timer(&'a str),
    Material(&'a str),
    Word(&'a str),
    Space(&'a str),
    Comment(&'a str),
    Backstory(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ingredient { name, .. } => write!(f, "{}", name),
            Token::RecipeRef { name, .. } => write!(f, "\"{}\"", name),
            Token::Backstory(v)
            | Token::Timer(v)
            | Token::Material(v)
            | Token::Word(v)
            | Token::Space(v) => write!(f, "{}", v),
            Token::Metadata { .. } | Token::Comment(_) => Ok(()),
        }
    }
}
