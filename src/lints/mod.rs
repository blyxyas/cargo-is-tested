use miette::Result;
use syn::{Attribute, File, Item};
use thiserror::Error;

mod emptiness;

pub trait Pass {
    fn check_file(file: &File) -> Result<()> {
        Ok(())
    }
    fn check_attributes(attribute: &Vec<Attribute>) -> Result<()> {
        Ok(())
    }
    fn check_attribute(attribute: &Attribute) -> Result<()> {
        Ok(())
    }
    fn check_items(item: &Vec<Item>) -> Result<()> {
        Ok(())
    }
    fn check_item(item: &Item) -> Result<()> {
        Ok(())
    }
}

pub fn check_lints(file: &File) -> Result<()> {
    for item in &file.items {
        emptiness::Emptiness::check_item(&item)?;
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