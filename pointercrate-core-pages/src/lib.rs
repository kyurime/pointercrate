use crate::{
    footer::Footer,
    head::{Head, HeadLike},
    navigation::NavigationBar,
};
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

pub mod config;
pub mod error;
pub mod footer;
pub mod head;
pub mod navigation;
pub mod util;

pub struct PageConfiguration {
    pub footer: Footer,
    pub nav_bar: NavigationBar,
    pub head: Head,
}

impl HeadLike for PageConfiguration {
    fn head_mut(&mut self) -> &mut Head {
        &mut self.head
    }
}

impl PageConfiguration {
    pub fn new(site_name: impl Into<String>, nav_bar: NavigationBar, footer: Footer) -> Self {
        let default_head_html = html! {
            @if let Some(publisher_id) = config::adsense_publisher_id() {
                (PreEscaped(format!(r#"<script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={}" crossorigin="anonymous"></script>"#, publisher_id)))
            }

            @if let Some(analytics_tag) = config::google_analytics_tag() {
                (PreEscaped(format!(r#"
                <!-- Global site tag (gtag.js) - Google Analytics -->
                <script async src="https://www.googletagmanager.com/gtag/js?id={0}"></script>
                <script>
                  window.dataLayer = window.dataLayer || [];
                  function gtag(){{dataLayer.push(arguments);}}
                  gtag('js', new Date());
                
                  gtag('config', '{0}');
                </script>
                "#, analytics_tag)));
            }

            meta http-equiv="Content-Type" content = "text/html; charset=utf-8";
            meta http-equiv="Content-Style-Type" content="text/css";
        };

        PageConfiguration {
            footer,
            nav_bar,
            head: Head::new(default_head_html)
                .meta("og:site_name", site_name)
                .meta("og:type", "website")
                .meta("referrer", "no-referrer")
                .meta("viewport", "initial-scale=1, maximum-scale=1"),
        }
    }

    pub fn author(self, author: impl Into<String>) -> Self {
        self.meta("author", author)
    }

    pub fn keywords(self, keywords: impl Into<String>) -> Self {
        self.meta("keywords", keywords)
    }
}

pub struct PageFragment {
    pub head: Head,
    pub body: Markup,
}

impl HeadLike for PageFragment {
    fn head_mut(&mut self) -> &mut Head {
        &mut self.head
    }
}

impl PageFragment {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> PageFragment {
        let title = title.into();
        let description = description.into();

        PageFragment {
            head: Head::new(html! { title { (title) }}),
            body: html! {},
        }
        .meta("og:title", &title)
        .meta("og:description", &description)
        .meta("description", description)
    }

    pub fn head(mut self, head: Markup) -> Self {
        self.head.other = html! {
            (self.head.other)
            (head)
        };
        self
    }

    pub fn body(mut self, body: Markup) -> Self {
        self.body = body;
        self
    }
}

impl Render for PageFragment {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" prefix="og: http://opg.me/ns#" {
                head {
                    (self.head)
                }
                body style="z-index:-10" {
                    (self.body)
                }
            }
        }
    }
}
