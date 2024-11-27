mod i18n;
mod lang_t;
mod parse_yaml;

use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use i18n::_i18n;
use lang_t::_lang_t;
use parse_yaml::LangYaml;
use proc_macro::TokenStream;

pub(crate) static YAML_PATH: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub(crate) static YAML_DATA: LazyLock<Mutex<Option<LangYaml>>> = LazyLock::new(|| Mutex::new(None));
pub(crate) static YAML_LANGS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

#[proc_macro]
pub fn i18n(tokens: TokenStream) -> TokenStream {
    _i18n(tokens.into()).into()
}

#[proc_macro]
pub fn lang_t(tokens: TokenStream) -> TokenStream {
    _lang_t(tokens.into()).into()
}
