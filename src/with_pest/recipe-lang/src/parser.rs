use crate::token::Token;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../grammar/recipe.pest"]
pub struct RecipeParser;

fn curly_content<'a>(pair: Pair<'a, Rule>) -> &'a str {
    pair.into_inner().next().unwrap().as_str().trim()
}

fn amount_parts<'a>(pair: Pair<'a, Rule>) -> (Option<&'a str>, Option<&'a str>) {
    let mut quantity = None;
    let mut unit = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::quantity => quantity = Some(p.as_str()),
            Rule::unit_inner => unit = Some(p.as_str().trim()),
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

        Rule::backstory => Token::Backstory(pair.into_inner().next().unwrap().as_str()),

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

pub fn parse<'a>(input: &'a str) -> Result<Vec<Token<'a>>, pest::error::Error<Rule>> {
    let recipe = RecipeParser::parse(Rule::recipe, input)?.next().unwrap();
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

    fn parse_quantity(input: &str) -> Result<String, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::quantity, input)?.next().unwrap();
        Ok(pair.as_str().to_string())
    }

    fn parse_comment(input: &str) -> Result<String, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::comment, input)?.next().unwrap();
        Ok(pair
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .trim()
            .to_string())
    }

    fn parse_timer<'a>(input: &'a str) -> Result<&'a str, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::timer, input)?.next().unwrap();
        Ok(curly_content(pair.into_inner().next().unwrap()))
    }

    fn parse_material<'a>(input: &'a str) -> Result<&'a str, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::material, input)?.next().unwrap();
        Ok(curly_content(pair.into_inner().next().unwrap()))
    }

    fn parse_ingredient<'a>(input: &'a str) -> Result<Token<'a>, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::ingredient, input)?
            .next()
            .unwrap();
        Ok(pair_to_token(pair))
    }

    fn parse_recipe_ref<'a>(input: &'a str) -> Result<Token<'a>, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::recipe_ref, input)?
            .next()
            .unwrap();
        Ok(pair_to_token(pair))
    }

    fn parse_metadata<'a>(input: &'a str) -> Result<Token<'a>, pest::error::Error<Rule>> {
        let pair = RecipeParser::parse(Rule::metadata, input)?.next().unwrap();
        Ok(pair_to_token(pair))
    }

    #[rstest]
    #[case("1", "1")]
    #[case("3.2", "3.2")]
    #[case("3,2", "3,2")]
    #[case("3_000_000", "3_000_000")]
    #[case("2/3", "2/3")]
    fn test_parse_quantity_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_quantity(input).unwrap(), expected);
    }

    #[rstest]
    #[case("abc")]
    #[case("")]
    fn test_parse_quantity_err(#[case] input: &str) {
        assert!(parse_quantity(input).is_err());
    }

    #[rstest]
    #[case("/* hello */", "hello")]
    #[case("/* multi\nline */", "multi\nline")]
    #[case("/**/", "")]
    fn test_parse_comment_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_comment(input).unwrap(), expected);
    }

    #[rstest]
    #[case("t{1 minute}", "1 minute")]
    #[case("t{2 hours}", "2 hours")]
    fn test_parse_timer_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_timer(input).unwrap(), expected);
    }

    #[rstest]
    #[case("&{pot}", "pot")]
    #[case("&{small jar}", "small jar")]
    #[case("&{stick}", "stick")]
    fn test_parse_material_ok(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(parse_material(input).unwrap(), expected);
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
    #[case(">> author: woile", "author", "woile")]
    #[case(">> servings: 4", "servings", "4")]
    fn test_parse_metadata_ok(#[case] input: &str, #[case] key: &str, #[case] value: &str) {
        assert_eq!(
            parse_metadata(input).unwrap(),
            Token::Metadata {
                key: key,
                value: value
            }
        );
    }

    #[test]
    fn test_full_recipe_basic() {
        let tokens = parse("Take the {potatoe}(1) and boil it").unwrap();
        assert!(tokens.iter().any(|t| matches!(
            t, Token::Ingredient { name, .. } if *name == "potatoe"
        )));
    }

    #[test]
    fn test_full_recipe_comment() {
        let tokens = parse("Add {salt} /* to taste */").unwrap();
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Comment(c) if *c == "to taste"))
        );
    }

    #[test]
    fn test_full_recipe_backstory() {
        let input = "Make {pasta}(200g)\n---\nThis recipe is from grandma";
        let tokens = parse(input).unwrap();
        assert!(tokens.iter().any(|t| matches!(
            t, Token::Backstory(v) if v.contains("grandma")
        )));
    }

    #[test]
    fn test_full_recipe_mixed() {
        let input = "\
>> servings: 4
Boil {water}(1L) with {salt}(1 tsp).
Add @{pasta-base}(200 g) and cook for t{10 minutes}.
Use &{large pot}. /* from grandma */
";
        let tokens = parse(input).unwrap();
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Metadata { key, .. } if *key == "servings"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Ingredient { name, .. } if *name == "water"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::RecipeRef  { name, .. } if *name == "pasta-base"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Timer(v) if *v == "10 minutes"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Material(v) if *v == "large pot"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Comment(c) if *c == "from grandma"))
        );
    }
}
