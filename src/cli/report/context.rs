use std::path::Path;

use crate::location::Locator;

pub struct Context<'a> {
    pub source: &'a str,
    pub path: Option<&'a Path>,
    pub locator: &'a Locator<'a>,
}
