use anyhow::Result;
use scraper::{Html, Selector};
use serde_json::Value;

#[derive(Debug, serde::Serialize)]
pub struct Recipe {
    pub title: String,
    pub date: String,
    pub description: String,
    pub preptime: String,
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
    pub categories: Vec<String>,
    pub image: String,
    pub favorite: bool,
    pub temperature: String,
}

pub fn parse_recipe(html: &str) -> Result<Recipe> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();

    for element in document.select(&selector) {
        let json: Value = serde_json::from_str(&element.inner_html())?;
        if json["@type"] == "Recipe" {
            return Recipe::from(&json);
        }
        if let Some(recipe) = json["@graph"]
            .as_array()
            .and_then(|g| g.iter().find(|e| e["@type"] == "Recipe"))
        {
            return Recipe::from(&recipe);
        }
    }
    Err(anyhow::anyhow!("No recipe found"))
}

impl Recipe {
    fn from(json: &Value) -> Result<Self> {
        let title = json["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid recipe data, missing field 'name'"))?;
        let date = json["datePublished"].as_str().unwrap_or_default();
        let description = json["description"].as_str().unwrap_or_default();
        let preptime = json["totalTime"].as_str().unwrap_or_default();
        let instructions = json["recipeInstructions"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|i| match i {
                Value::String(s) => s.to_owned(),
                Value::Object(o) => o["text"].as_str().unwrap_or_default().to_owned(),
                _ => "".to_owned(),
            })
            .collect();
        let ingredients = json["recipeIngredient"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|i| i.as_str().unwrap_or_default().to_owned())
            .collect();
        let categories: Vec<String> = json["recipeCategory"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|i| i.as_str().unwrap_or_default().to_owned())
            .collect();

        Ok(Recipe {
            title: title.to_owned(),
            date: date.to_owned(),
            description: description.to_owned(),
            preptime: format_preptime(preptime),
            instructions,
            ingredients,
            categories,
            image: "".to_owned(),
            favorite: false,
            temperature: "".to_owned(),
        })
    }

    pub fn filename(&self) -> String {
        format!("{}.md", self.title.to_lowercase().replace(" ", "-"))
    }
}

/// Format a preptime string into a human readable format
///
/// The input string is expected to be in the format "PT1H30M"
fn format_preptime(preptime: &str) -> String {
    let mut output = String::new();
    for c in preptime.chars() {
        match c {
            'P' => continue,
            'T' => continue,
            'H' => output.push_str(" h "),
            'M' => output.push_str(" min "),
            'S' => output.push_str(" s "),
            _ => output.push(c),
        }
    }
    output.trim().to_owned()
}
