extern crate comrak;
extern crate syntect;
use comrak::{
    adapters::SyntaxHighlighterAdapter, markdown_to_html_with_plugins, ComrakExtensionOptions,
    ComrakOptions, ComrakParseOptions, ComrakPlugins, ComrakRenderOptions,
};
use std::collections::HashMap;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use wasm_bindgen::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct SyntaxHighLightAdapter {}

impl SyntaxHighLightAdapter {
    pub fn new() -> Self {
        SyntaxHighLightAdapter {}
    }
}

impl SyntaxHighlighterAdapter for SyntaxHighLightAdapter {
    fn build_pre_tag(&self, attributes: &HashMap<String, String>) -> String {
        if attributes.contains_key("lang") {
            format!("<pre language-{}>", attributes["lang"])
        } else {
            String::from("<pre>")
        }
    }

    fn build_code_tag(&self, attributes: &HashMap<String, String>) -> String {
        if attributes.contains_key("class") {
            format!("<code class=\"{}\">", attributes["class"])
        } else {
            String::from("<code>")
        }
    }

    fn highlight(&self, _lang: Option<&str>, code: &str) -> String {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let theme = &ts.themes["base16-ocean.dark"];
        let sr = ss.find_syntax_by_extension("js").unwrap();

        highlighted_html_for_string(code, &ss, &sr, theme).unwrap()
    }
}

#[wasm_bindgen]
pub fn parse_to_html(input: &str) -> String {
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            autolink: true,
            strikethrough: true,
            tagfilter: true,
            table: true,
            tasklist: true,
            superscript: true,
            ..Default::default()
        },
        parse: ComrakParseOptions {
            smart: true,
            ..Default::default()
        },
        render: ComrakRenderOptions::default(),
    };
    let adapter = SyntaxHighLightAdapter::new();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    let html = markdown_to_html_with_plugins(input, &options, &plugins);
    html
}
