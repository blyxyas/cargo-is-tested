use crate::{error::ErrorKind, did_you_mean};
use miette::{NamedSource, Result};

use super::lints::LINT_NAMES;

pub fn check_flags(filename: &str, shebang: &str) -> Option<Result<Vec<String>>> {
    if let Some(position) = shebang.find("is-tested") {
        let flags = shebang[position + 9..]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&s| s.into())
            .collect::<Vec<String>>();

        for flag in &flags {
            if !LINT_NAMES.contains(&flag.as_str()) {
                // Find alternatives
                let mut note = None;
                if let Some(sugg) = did_you_mean(&flag, &LINT_NAMES) {
                    note = Some(format!("did you mean: `{}`?", sugg));
                }

                let flag_span = (shebang.find(flag).unwrap(), flag.len());
                return Some(Err(ErrorKind::UnknownLint {
                    src: NamedSource::new(filename, shebang.to_owned()),
                    span: flag_span.into(),
                    note,
                }
                .into()));
            }
        }
        return Some(Ok(flags));
    }

    return None;
}
