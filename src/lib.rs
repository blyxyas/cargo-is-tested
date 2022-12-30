#[cfg(feature = "suggestions")]
pub fn did_you_mean<'a, T, I>(field: &str, alternatives: I) -> Option<String>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let mut candidate: Option<(f64, &str)> = None;
    for pv in alternatives {
        let confidence = ::strsim::jaro_winkler(field, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    candidate.map(|(_, candidate)| candidate.into())
}

#[cfg(not(feature = "suggestions"))]
pub fn did_you_mean<'a, T, I>(field: &str, alternatives: I) -> Option<String>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    None
}
