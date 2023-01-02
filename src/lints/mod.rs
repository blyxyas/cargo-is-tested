use miette::Result;
use syn::{Attribute, File, Item};

pub mod emptiness;
pub mod validness;

pub const LINT_NAMES: [&str; 3] = ["emptiness", "validness", "strict"];

// The first two attributes in every method is `source` and `filename`.
pub trait Pass {
    fn check_file(_: &str, _: &str, _: &File) -> Result<()> {
        Ok(())
    }
    fn check_attributes(_: &str, _: &str, _: &Vec<Attribute>) -> Result<()> {
        Ok(())
    }
    fn check_attribute(_: &str, _: &str, _: &Attribute) -> Result<()> {
        Ok(())
    }
    fn check_items(_: &str, _: &str, _: &Vec<Item>) -> Result<()> {
        Ok(())
    }
    fn check_item(_: &str, _: &str, _: &Item) -> Result<()> {
        Ok(())
    }
}

#[doc(hidden)]
pub fn check_lints(
    source: &str,
    filename: &str,
    items: &Vec<Item>,
    lints: Vec<String>,
) -> Result<()> {
    if lints.contains(&"strict".to_owned()) {
        emptiness::Emptiness::check_items(source, filename, items)?;
        validness::ItemValidness::check_items(source, filename, items)?;
    } else {
        for lint in &lints {
            match lint.as_str() {
                "emptiness" => {
                    emptiness::Emptiness::check_items(source, filename, items)?;
                }
                "validness" => {
                    validness::ItemValidness::check_items(source, filename, items)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
