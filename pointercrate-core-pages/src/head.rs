use maud::{html, Markup, PreEscaped, Render};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Head {
    scripts: Vec<Script>,
    stylesheets: Vec<String>,
    meta_tags: Vec<Meta>,
    import_map: HashMap<String, String>,

    pub(crate) other: Markup,
}

impl Head {
    pub fn new(other: Markup) -> Head {
        Head {
            scripts: vec![],
            stylesheets: vec![],
            meta_tags: vec![],
            import_map: HashMap::new(),
            other,
        }
    }
}

impl Render for Head {
    fn render(&self) -> Markup {
        html! {
            @if !&self.import_map.is_empty() {
                script type = "importmap" {
                    (PreEscaped(render_import_map(&self.import_map)))
                }
            }

            @for meta in &self.meta_tags {
                meta name=(meta.name) content=(meta.content);
            }

            @for script in &self.scripts {
                (script)
            }

            @for stylesheet in &self.stylesheets {
                link rel = "stylesheet" href = (stylesheet);
            }

            (self.other)
        }
    }
}

pub trait HeadLike: Sized {
    fn head_mut(&mut self) -> &mut Head;

    fn with_stylesheet(mut self, url: String) -> Self {
        self.head_mut().stylesheets.push(url);
        self
    }

    fn with_script(mut self, script: Script) -> Self {
        self.head_mut().scripts.push(script);
        self
    }

    fn with_meta(mut self, meta: Meta) -> Self {
        self.head_mut().meta_tags.push(meta);
        self
    }

    fn with_import(mut self, import: String, val: String) -> Self {
        self.head_mut().import_map.insert(import, val);
        self
    }

    fn meta(self, name: impl Into<String>, content: impl Into<String>) -> Self {
        self.with_meta(Meta::new(name, content))
    }

    fn script(self, src: impl Into<String>) -> Self {
        self.with_script(Script::new(src))
    }

    fn module(self, module: impl Into<String>) -> Self {
        self.with_script(Script::module(module))
    }

    fn stylesheet(self, sheet: impl Into<String>) -> Self {
        self.with_stylesheet(sheet.into())
    }

    fn import<S: Into<String>>(self, import: (S, S)) -> Self {
        self.with_import(import.0.into(), import.1.into())
    }
}

impl HeadLike for Head {
    fn head_mut(&mut self) -> &mut Head {
        self
    }
}

/// Converts an import hashmap to a JSON string
fn render_import_map(map: &HashMap<String, String>) -> String {
    // there's no json here... oh no
    let mut inner = "".to_string();

    for (key, val) in map.iter() {
        inner.push_str(&format!(r#""{}": "{}","#, key, val));
    }

    // remove trailing comma
    inner.pop();

    format!(r#"{{"imports": {{ {} }}}}"#, inner)
}

#[derive(Debug, Clone)]
pub struct Script {
    src: String,
    module: bool,
}

impl Script {
    pub fn new<S: Into<String>>(src: S) -> Self {
        Script {
            src: src.into(),
            module: false,
        }
    }

    pub fn module<S: Into<String>>(src: S) -> Self {
        Script {
            src: src.into(),
            module: true,
        }
    }
}

impl Render for Script {
    fn render(&self) -> Markup {
        html! {
            @if self.module {
                script src = (self.src) type = "module" {}
            }
            @else {
                script src = (self.src) {};
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    name: String,
    content: String,
}

impl Meta {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Meta {
        Meta {
            name: name.into(),
            content: content.into(),
        }
    }
}

impl Render for &Meta {
    fn render(&self) -> Markup {
        html! {
            meta name=(self.name) property=(self.name) content=(self.content);
        }
    }
}

/// Adds a version query string to a url based on the package version.
///
/// A macro is used to make the version crate-independent.
#[macro_export]
macro_rules! with_version_string {
    ($path:literal) => {
        concat!($path, "?v=", env!("CARGO_PKG_VERSION"))
    };
}

/// Returns a pair of strings,
/// with first value being provided path and second being path with import
#[macro_export]
macro_rules! versioned_import {
    ($path:literal) => {
        ($path, $crate::with_version_string!($path))
    };
}

/// Same as versioned_import,
/// but returns a (String, String) instead of (&str, &str)
#[macro_export]
macro_rules! versioned_import_string {
    ($path:literal) => {
        ($path.to_string(), $crate::with_version_string!($path).to_string())
    };
}
