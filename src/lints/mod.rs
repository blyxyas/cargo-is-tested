use miette::Result;
use syn::{Attribute, File, Item};
use thiserror::Error;

mod validness;
mod emptiness;

pub const LINT_NAMES: [&str; 3] = [
	"emptiness",
	"validness",
	"strict"
];

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

pub fn check_lints(filename: &str, file: &File) -> Result<()> {
    for item in &file.items {
        emptiness::Emptiness::check_item(filename, &item)?;
		validness::ItemValidness::check_item(filename, &item)?;
	};

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
