use crate::token::Token;
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../grammar/recipe.pest"]
pub struct RecipeParser;

type Result<T> = std::result::Result<T, Error<Rule>>;

fn curly_content<'a>(pair: Pair<'a, Rule>) -> &'a str {
    pair.into_inner().next().unwrap().as_str().trim()
}

fn amount_parts<'a>(pair: Pair<'a, Rule>) -> (Option<&'a str>, Option<&'a str>) {
    let mut quantity = None;
    let mut unit = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::quantity => quantity = Some(p.as_str()),
            Rule::unit => unit = Some(p.as_str().trim()),
            _ => {}
        }
    }
    (quantity, unit)
}

fn pair_to_token<'a>(pair: Pair<'a, Rule>) -> Token<'a> {
    match pair.as_rule() {
        Rule::recipe_value => pair_to_token(pair.into_inner().next().unwrap()),

        Rule::metadata => {
            let mut inner = pair.into_inner();
            Token::Metadata {
                key: inner.next().unwrap().as_str().trim(),
                value: inner.next().unwrap().as_str().trim(),
            }
        }

        Rule::material => Token::Material(curly_content(pair.into_inner().next().unwrap())),
        Rule::timer => Token::Timer(curly_content(pair.into_inner().next().unwrap())),
        Rule::comment => Token::Comment(pair.into_inner().next().unwrap().as_str().trim()),
        Rule::backstory => Token::Backstory(pair.into_inner().next().unwrap().as_str().trim()),

        Rule::ingredient | Rule::recipe_ref => {
            let is_ref = pair.as_rule() == Rule::recipe_ref;
            let mut inner = pair.into_inner();
            let name = curly_content(inner.next().unwrap());
            let (quantity, unit) = inner.next().map(amount_parts).unwrap_or_default();
            if is_ref {
                Token::RecipeRef {
                    name,
                    quantity,
                    unit,
                }
            } else {
                Token::Ingredient {
                    name,
                    quantity,
                    unit,
                }
            }
        }

        Rule::word => Token::Word(pair.as_str()),
        Rule::space => Token::Space(pair.as_str()),

        r => unreachable!("Unexpected rule in pair_to_token: {r:?}"),
    }
}

pub fn parse(input: &str) -> Result<Vec<Token<'_>>> {
    let recipe = RecipeParser::parse(Rule::recipe, input)?
        .next()
        .expect("pest always returns at least one pair for a matched rule");

    Ok(recipe
        .into_inner()
        .filter(|p| p.as_rule() != Rule::EOI)
        .map(pair_to_token)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn parse_valid_string<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::valid_string, input)?
            .next()
            .unwrap();
        Ok(pair.as_str())
    }

    fn parse_quantity<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::quantity, input)?.next().unwrap();
        Ok(pair.as_str())
    }

    fn parse_comment<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::comment, input)?.next().unwrap();
        Ok(pair.into_inner().next().unwrap().as_str().trim())
    }

    fn parse_curly<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::curly, input)?.next().unwrap();
        Ok(pair.into_inner().next().unwrap().as_str().trim())
    }

    fn parse_timer<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::timer, input)?.next().unwrap();
        Ok(curly_content(pair.into_inner().next().unwrap()))
    }

    fn parse_material<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::material, input)?.next().unwrap();
        Ok(curly_content(pair.into_inner().next().unwrap()))
    }

    fn parse_ingredient_amount<'a>(input: &'a str) -> Result<(Option<&'a str>, Option<&'a str>)> {
        let pair = RecipeParser::parse(Rule::ingredient_amount, input)?
            .next()
            .unwrap();

        let mut inner = pair.into_inner();
        Ok((
            inner.next().map(|p| p.as_str().trim()),
            inner.next().map(|p| p.as_str().trim()),
        ))
    }

    fn parse_ingredient<'a>(input: &'a str) -> Result<Token<'a>> {
        let pair = RecipeParser::parse(Rule::ingredient, input)?
            .next()
            .unwrap();
        Ok(pair_to_token(pair))
    }

    fn parse_recipe_ref<'a>(input: &'a str) -> Result<Token<'a>> {
        let pair = RecipeParser::parse(Rule::recipe_ref, input)?
            .next()
            .unwrap();
        Ok(pair_to_token(pair))
    }

    fn parse_metadata<'a>(input: &'a str) -> Result<Token<'a>> {
        let pair = RecipeParser::parse(Rule::metadata, input)?.next().unwrap();
        Ok(pair_to_token(pair))
    }

    fn parse_backstory<'a>(input: &'a str) -> Result<&'a str> {
        let pair = RecipeParser::parse(Rule::backstory, input)?.next().unwrap();
        Ok(pair.into_inner().next().unwrap().as_str().trim())
    }

    #[rstest]
    #[case("salt", "salt")]
    #[case("sweet potato", "sweet potato")]
    #[case("ToMaToeS", "ToMaToeS")]
    #[case("1/2 lemon", "1/2 lemon")]
    #[case("my-best-sauce", "my-best-sauce")]
    #[case("1.2", "1.2")]
    #[case("1,2", "1,2")]
    #[case("1_200", "1_200")]
    #[case("@woile", "@woile")]
    #[case("10%", "10%")]
    #[case("#vegan", "#vegan")]
    #[case("mango's", "mango's")]
    fn test_parse_valid_string(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_valid_string(input).unwrap(), expected)
    }

    #[rstest]
    #[case("/* */", "")]
    #[case("/* hello */", "hello")]
    #[case("/* multi\nline\ncomment */", "multi\nline\ncomment")]
    fn test_parse_comment_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_comment(input).unwrap(), expected);
    }

    #[rstest]
    fn test_parse_comment_wrong() {
        let input = "/* unclosed";
        let res = parse_comment(input);
        assert!(res.is_err());

        let err = res.unwrap_err();
        println!("{:?}", err);
    }

    #[rstest]
    #[case("{salt}", "salt")]
    #[case("{black pepper}", "black pepper")]
    #[case("{smashed potatoes}", "smashed potatoes")]
    #[case("{15 minutes}", "15 minutes")]
    #[case("{   15 minutes  }", "15 minutes")]
    fn test_parse_curly_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_curly(input).unwrap(), expected);
    }

    #[test]
    fn test_parse_curly_wrong() {
        let input = "{}";
        let res = parse_curly(input);
        assert!(res.is_err());

        let input = "{unclosed";
        let res = parse_curly(&input);
        assert!(res.is_err());
    }

    #[rstest]
    #[case("200", "200")]
    #[case("2.1", "2.1")]
    #[case("2_1", "2_1")]
    #[case("2,1", "2,1")]
    #[case("2.1", "2.1")]
    #[case("1/2", "1/2")]
    #[case(".2", ".2")]
    fn test_parse_quantity_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_quantity(input).unwrap(), expected);
    }

    #[rstest]
    #[case("2.")]
    #[case("2..0")]
    #[case("2,,0")]
    #[case("2//0")]
    fn test_parse_quantity_invalid(#[case] input: &str) {
        let res = parse_quantity(input);
        assert!(res.is_err());
    }

    #[rstest]
    #[case("(200gr)", (Some("200"), Some("gr")))]
    #[case("(1/2)", (Some("1/2"), None))]
    #[case("(100 gr)", (Some("100"), Some("gr")))]
    #[case("(10 ml)", (Some("10"), Some("ml")))]
    #[case("( 10 ml )", (Some("10"), Some("ml")))]
    #[case("(1.5 cups)", (Some("1.5"), Some("cups")))]
    fn test_parse_ingredient_amount_ok(
        #[case] input: &str,
        #[case] expected: (Option<&str>, Option<&str>),
    ) {
        let content = parse_ingredient_amount(input).unwrap();
        assert_eq!(expected, content);
    }

    #[rstest]
    #[case("{quinoa}(200gr)", "quinoa", Some("200"), Some("gr"))]
    #[case("{tomatoes}(2)", "tomatoes", Some("2"), None)]
    #[case("{sweet potatoes}(2)", "sweet potatoes", Some("2"), None)]
    #[case("{salt}", "salt", None, None)]
    #[case("{water}(1L)", "water", Some("1"), Some("L"))]
    fn test_parse_ingredient_ok(
        #[case] input: &str,
        #[case] name: &str,
        #[case] quantity: Option<&str>,
        #[case] unit: Option<&str>,
    ) {
        assert_eq!(
            parse_ingredient(input).unwrap(),
            Token::Ingredient {
                name: name,
                quantity: quantity,
                unit: unit,
            }
        );
    }

    #[rstest]
    #[case("&{pot}", "pot")]
    #[case("&{small jar}", "small jar")]
    #[case("&{stick}", "stick")]
    #[case("&{bricks}", "bricks")]
    fn test_parse_material_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_material(input).unwrap(), expected);
    }

    #[rstest]
    #[case("t{1 minute}", "1 minute")]
    #[case("t{2 hours}", "2 hours")]
    fn test_parse_timer_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_timer(input).unwrap(), expected);
    }

    #[rstest]
    #[case(
        "@{woile/special-tomato-sauce}",
        "woile/special-tomato-sauce",
        None,
        None
    )]
    #[case(
        "@{woile/special-tomato-sauce}(100 ml)",
        "woile/special-tomato-sauce",
        Some("100"),
        Some("ml")
    )]
    #[case(
        "@{woile/tomato-sauce}(200gr)",
        "woile/tomato-sauce",
        Some("200"),
        Some("gr")
    )]
    #[case("@{woile/tomato-sauce}", "woile/tomato-sauce", None, None)]
    #[case("@{special stew}", "special stew", None, None)]
    fn test_parse_recipe_ref_ok(
        #[case] input: &str,
        #[case] name: &str,
        #[case] quantity: Option<&str>,
        #[case] unit: Option<&str>,
    ) {
        assert_eq!(
            parse_recipe_ref(input).unwrap(),
            Token::RecipeRef {
                name: name,
                quantity: quantity,
                unit: unit,
            }
        );
    }

    #[rstest]
    #[case(">> tags: vegan\n", ("tags", "vegan"))]
    #[case(">> key: pepe\n", ("key", "pepe"))]
    #[case(">>key: pepe\n", ("key", "pepe"))]
    #[case(">>    key: pepe\n", ("key", "pepe"))]
    #[case(">>    key:     pepe\n", ("key", "pepe"))]
    #[case(">>    key:\t\tpepe\n", ("key", "pepe"))]
    #[case(">>    key:pepe\n", ("key", "pepe"))]
    fn test_parse_metadata_ok(#[case] input: &str, #[case] expected: (&str, &str)) {
        assert_eq!(
            parse_metadata(input).unwrap(),
            Token::Metadata {
                key: expected.0,
                value: expected.1
            }
        );
    }

    #[rstest]
    #[case("\n---\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\n\nwhat a backstory", "what a backstory")]
    #[case("\n   ---\n\nthis is **markdown**", "this is **markdown**")]
    #[case("\n   ---\n\nthis is [markdown](url)", "this is [markdown](url)")]
    fn test_parse_backstory_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_backstory(input).unwrap(), expected);
    }

    #[rstest]
    #[case("\n---    \nwhat a backstory")]
    fn test_parse_backstory_fail(#[case] input: &str) {
        assert!(parse_backstory(input).is_err())
    }

    #[rstest]
    #[case(" ", Token::Space(" "))]
    #[case("{holis}(100 gr)", Token::Ingredient { name: "holis", quantity: Some("100"), unit: Some("gr") })]
    fn test_recipe_value_ok(#[case] input: &str, #[case] expected: Token) {
        let pair = RecipeParser::parse(Rule::recipe_value, input)
            .unwrap()
            .next()
            .unwrap();
        let token = pair_to_token(pair);
        assert_eq!(token, expected)
    }

    #[test]
    fn test_recipe_ok() {
        let input = "Boil the quinoa for t{5 minutes} in a &{pot}.\nPut the boiled {quinoa}(200gr) in the base of the bowl.";
        let expected = "Boil the quinoa for 5 minutes in a pot.\nPut the boiled quinoa in the base of the bowl.";
        let recipe = parse(input).expect("parse failed");
        let fmt_recipe = recipe
            .iter()
            .fold(String::new(), |acc, val| format!("{acc}{val}"));
        println!("{}", fmt_recipe);

        assert_eq!(expected, fmt_recipe);
        println!("{:?}", recipe);
    }

    #[rstest]
    #[case(" ", vec![Token::Space(" ")])]
    #[case("\n\nhello", vec![Token::Space("\n\n"), Token::Word("hello")])]
    #[case("hello\n", vec![Token::Word("hello"), Token::Space("\n")])]
    #[case(">> tags: hello\n\nhello", vec![Token::Metadata {key: "tags", value: "hello"}, Token::Space("\n\n"), Token::Word("hello")])]
    #[case(">> source: https://hello.com\n>> tags: hello\n", vec![Token::Metadata {key: "source", value: "https://hello.com"}, Token::Space("\n"), Token::Metadata {key: "tags", value: "hello"}, Token::Space("\n")])]
    #[case("{holis}(100 gr)", vec![Token::Ingredient { name: "holis", quantity: Some("100"), unit: Some("gr") }])]
    fn test_recipe_cases_ok(#[case] input: &str, #[case] expected: Vec<Token>) {
        let token = parse(input).expect("failed to parse token");
        assert_eq!(token, expected)
    }

    #[rstest]
    #[case("Foo. ")]
    #[case("Foo.")]
    #[case("Foo, bar")]
    #[case("Foo,bar")]
    #[case("Foo,")]
    #[case("Foo, ")]
    #[case("Foo,\n")]
    #[case("Foo.\n")]
    #[case("Foo.\nfoo")]
    #[case("Foo,\nfoo")]
    fn test_symbol_parsing(#[case] input: &str) {
        let recipe_result = parse(input);

        assert!(recipe_result.is_ok());
    }

    #[test]
    fn test_parse_ok() {
        let input = "Boil the quinoa for t{5 minutes} in a &{pot}.\nPut the boiled {quinoa}(200gr) in the base of the bowl.";
        let expected = "Boil the quinoa for 5 minutes in a pot.\nPut the boiled quinoa in the base of the bowl.";
        let recipe = parse(input).expect("parse failed");
        let fmt_recipe = recipe
            .iter()
            .fold(String::new(), |acc, val| format!("{acc}{val}"));
        println!("{}", fmt_recipe);

        assert_eq!(expected, fmt_recipe);
        println!("{:?}", recipe);
    }

    #[test]
    fn test_parse_with_backstory_ok() {
        let input = "Foo. \n---\nA backstory";
        let expected = vec![
            Token::Word("Foo."),
            Token::Space(" "),
            Token::Backstory("A backstory"),
        ];
        let recipe = parse(input).expect("parse failed");

        println!("{:?}", recipe);

        assert_eq!(expected, recipe);
        println!("{:?}", recipe);
    }
}
