use miette::{Result, NamedSource};
use syn::{Attribute, File, Item};
use thiserror::Error;

use crate::error::ErrorKind;

mod emptiness;
mod validness;

pub const LINT_NAMES: [&str; 3] = ["emptiness", "validness", "strict"];

pub trait Pass {
    fn check_file(filename: &str, file: &File) -> Result<()> {
        Ok(())
    }
    fn check_attributes(filename: &str, attribute: &Vec<Attribute>) -> Result<()> {
        Ok(())
    }
    fn check_attribute(filename: &str, attribute: &Attribute) -> Result<()> {
        Ok(())
    }
    fn check_items(filename: &str, item: &Vec<Item>) -> Result<()> {
        Ok(())
    }
    fn check_item(filename: &str, item: &Item) -> Result<()> {
        Ok(())
    }
}

pub fn check_lints(source: &str, filename: &str, file: &File, lints: Vec<String>) -> Result<()> {
    if lints.contains(&"strict".to_owned()) {
        for item in &file.items {
            emptiness::Emptiness::check_item(filename, &item)?;
            validness::ItemValidness::check_item(filename, &item)?;
        }
    } else {
        for lint in &lints {
            match lint.as_str() {
                "emptiness" => {emptiness::Emptiness::check_items(filename, &file.items)?;},
				"validness" => {
					validness::ItemValidness::check_items(filename, &file.items)?;
				}
				_ => {}
            }
        }
    }

    Ok(())
}

pub struct LineColumn(proc_macro2::LineColumn);

impl Into<(usize, usize)> for LineColumn {
    fn into(self) -> (usize, usize) {
        (self.0.line, self.0.column)
    }
}

impl Into<LineColumn> for proc_macro2::LineColumn {
    fn into(self) -> LineColumn {
        LineColumn(self)
    }
}
