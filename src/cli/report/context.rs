use std::path::Path;

use crate::span::Locator;

pub struct Context<'a> {
    pub source: &'a str,
    pub path: Option<&'a Path>,
    pub locator: &'a Locator<'a>,
}
