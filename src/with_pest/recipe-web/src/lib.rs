pub mod scrollbar;

use std::collections::BTreeMap;

use recipe_lang::Token;

pub const HUMMUS: &str = ">> name: Everyday hummus
>> description: Creamy, lemony, and made for sharing.
>> tags: plant-based, high-protein, everyday
>> servings: 4
>> time: 10 minutes

Add {chickpeas}(400 g) to a &{blender} with
{garlic}(1 clove), {tahini}(2 tbsp), {lemon juice}(2 tbsp),
{olive oil}(2 tbsp), and {salt}(1/2 tsp).

Blend for t{3 minutes}, until smooth and creamy.

Spoon into a &{serving bowl}. Finish with a drizzle of
{olive oil} and a little {paprika}.
---
The simplest things are often the best.
Serve with warm flatbread and whatever is in season.";

pub const PASTA: &str = ">> name: A very good tomato pasta
>> description: A small ingredient list. A very happy table.
>> tags: vegetarian, weeknight, comfort-food
>> servings: 2
>> time: 25 minutes

Bring a &{large pot} of salted water to a boil. Add {spaghetti}(200 g) and cook for t{10 minutes}.

Warm {olive oil}(2 tbsp) in a &{pan}. Add {garlic}(2 cloves) and {cherry tomatoes}(300 g). Cook for t{8 minutes}.

Toss the pasta with the sauce and {basil}(1 handful). Finish with {parmesan}(30 g).
---
Save a cup of pasta water. It is the secret to a sauce that really comes together.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub unit: Option<String>,
}

impl Ingredient {
    pub fn amount(&self) -> String {
        [self.quantity.as_deref(), self.unit.as_deref()]
            .into_iter()
            .flatten()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inline {
    Text(String),
    Ingredient(String),
    Material(String),
    Timer(String),
    Reference(String),
}

impl Inline {
    pub fn text(&self) -> &str {
        match self {
            Self::Text(text)
            | Self::Ingredient(text)
            | Self::Material(text)
            | Self::Timer(text)
            | Self::Reference(text) => text,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Recipe {
    pub metadata: BTreeMap<String, String>,
    pub ingredients: Vec<Ingredient>,
    pub references: Vec<Ingredient>,
    pub materials: Vec<String>,
    pub timers: Vec<String>,
    pub steps: Vec<Vec<Inline>>,
    pub backstory: Option<String>,
    pub token_count: usize,
}

impl Recipe {
    pub fn parse(source: &str) -> Result<Self, String> {
        let tokens = recipe_lang::parse(source).map_err(|error| error.to_string())?;
        let mut recipe = Self {
            token_count: tokens.len(),
            ..Self::default()
        };
        let mut step = Vec::new();

        for token in tokens {
            match token {
                Token::Metadata { key, value } => {
                    recipe.metadata.insert(key.to_lowercase(), value.to_owned());
                }
                Token::Ingredient {
                    name,
                    quantity,
                    unit,
                }
                | Token::RecipeRef {
                    name,
                    quantity,
                    unit,
                } => {
                    let is_reference = matches!(token, Token::RecipeRef { .. });
                    let item = Ingredient {
                        name: name.to_owned(),
                        quantity: quantity
                            .filter(|value| !value.is_empty())
                            .map(str::to_owned),
                        unit: unit.filter(|value| !value.is_empty()).map(str::to_owned),
                    };
                    let list = if is_reference {
                        &mut recipe.references
                    } else {
                        &mut recipe.ingredients
                    };
                    // Keep different amounts separate; never silently add incompatible units.
                    if !list.contains(&item) {
                        list.push(item);
                    }
                    step.push(if is_reference {
                        Inline::Reference(name.to_owned())
                    } else {
                        Inline::Ingredient(name.to_owned())
                    });
                }
                Token::Material(name) => {
                    if !recipe.materials.iter().any(|material| material == name) {
                        recipe.materials.push(name.to_owned());
                    }
                    step.push(Inline::Material(name.to_owned()));
                }
                Token::Timer(duration) => {
                    recipe.timers.push(duration.to_owned());
                    step.push(Inline::Timer(duration.to_owned()));
                }
                Token::Word(word) => step.push(Inline::Text(word.to_owned())),
                Token::Space(space) => {
                    let newlines = space.replace("\r\n", "\n").replace('\r', "\n");
                    if newlines.matches('\n').count() >= 2 {
                        finish_step(&mut recipe.steps, &mut step);
                    } else if !step.is_empty() {
                        step.push(Inline::Text(if newlines.contains('\n') {
                            " ".to_owned()
                        } else {
                            space.to_owned()
                        }));
                    }
                }
                Token::Backstory(text) => recipe.backstory = Some(text.to_owned()),
                Token::Comment(_) => {}
            }
        }
        finish_step(&mut recipe.steps, &mut step);
        Ok(recipe)
    }

    pub fn title(&self) -> &str {
        self.metadata
            .get("name")
            .map(String::as_str)
            .filter(|s| !s.is_empty())
            .unwrap_or("Untitled recipe")
    }

    pub fn tags(&self) -> Vec<&str> {
        self.metadata
            .get("tags")
            .map(|tags| {
                tags.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn finish_step(steps: &mut Vec<Vec<Inline>>, step: &mut Vec<Inline>) {
    while matches!(step.last(), Some(Inline::Text(text)) if text.trim().is_empty()) {
        step.pop();
    }
    if !step.is_empty() {
        steps.push(std::mem::take(step));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_has_structured_content_and_backstory() {
        let recipe = Recipe::parse(HUMMUS).unwrap();
        assert_eq!(recipe.title(), "Everyday hummus");
        assert_eq!(recipe.steps.len(), 3);
        assert_eq!(recipe.ingredients.len(), 8);
        assert_eq!(recipe.materials, ["blender", "serving bowl"]);
        assert_eq!(recipe.timers, ["3 minutes"]);
        assert_eq!(recipe.tags(), ["plant-based", "high-protein", "everyday"]);
        assert!(recipe.backstory.unwrap().starts_with("The simplest"));
    }

    #[test]
    fn pasta_sample_has_three_steps() {
        let recipe = Recipe::parse(PASTA).unwrap();
        assert_eq!(recipe.steps.len(), 3);
        assert_eq!(recipe.ingredients.len(), 6);
    }

    #[test]
    fn empty_and_whitespace_do_not_create_steps() {
        for source in ["", " \n\t", ">> name: Test\n/* hidden */"] {
            let recipe = Recipe::parse(source).unwrap();
            assert!(recipe.steps.is_empty());
        }
    }

    #[test]
    fn comments_are_hidden_and_references_are_separate() {
        let recipe = Recipe::parse("Use @{stock}(100 ml) /* secret */ with {salt}.").unwrap();
        assert_eq!(recipe.references[0].amount(), "100 ml");
        assert_eq!(recipe.ingredients[0].amount(), "");
        assert!(
            !recipe.steps[0]
                .iter()
                .any(|part| part.text().contains("secret"))
        );
    }

    #[test]
    fn duplicate_items_are_only_deduplicated_when_identical() {
        let recipe = Recipe::parse("{salt}(1 tsp) {salt}(1 tsp) {salt}(2 tsp)").unwrap();
        assert_eq!(recipe.ingredients.len(), 2);
    }

    #[test]
    fn soft_line_breaks_stay_in_one_step_and_blank_lines_start_new_steps() {
        let recipe = Recipe::parse("Mix well\r\nwith care.\r\n\r\nServe.").unwrap();
        assert_eq!(recipe.steps.len(), 2);
        let text: String = recipe.steps[0].iter().map(Inline::text).collect();
        assert_eq!(text, "Mix well with care.");
    }

    #[test]
    fn unicode_text_and_html_are_plain_text() {
        let recipe = Recipe::parse("搅拌 🍲 <script>alert(1)</script>").unwrap();
        let text: String = recipe.steps[0].iter().map(Inline::text).collect();
        assert_eq!(text, "搅拌 🍲 <script>alert(1)</script>");
    }
}
