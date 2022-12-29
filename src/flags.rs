use crate::error::ErrorKind;
use cargo_is_tested::did_you_mean;
use miette::{NamedSource, Result};

use super::lints::LINT_NAMES;

pub fn check_flags(filename: &str, shebang: &str) -> Result<()> {
    if let Some(position) = shebang.find("is-tested") {
		dbg!(position);
        let flags = shebang[position + 9..]
            .split_whitespace()
            .collect::<Vec<&str>>();

        for flag in flags {
            if !LINT_NAMES.contains(&flag) {
                // Find alternatives
				let mut note = None;
                if let Some(sugg) = did_you_mean(flag, &LINT_NAMES) {
                    note = Some(format!("did you mean: `{}`?", sugg));
				}

                let flag_span = (shebang.find(flag).unwrap(), flag.len());
                return Err(ErrorKind::FileParseError {
                    src: NamedSource::new(filename, shebang.to_owned()),
                    span: flag_span.into(),
                    note,
                }
                .into());
            }
        }
    }
    Ok(())
}
