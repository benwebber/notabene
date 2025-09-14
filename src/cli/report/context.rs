use std::path::Path;

use crate::span::Index;

pub struct Context<'a> {
    pub source: &'a str,
    pub path: Option<&'a Path>,
    pub index: &'a Index<'a>,
}
