#![allow(unused_variables)]
// currently, all the request parameters are unused, but they will be required in the future

use crate::config;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fmt::Display;

pub mod account;
pub mod demonlist;
pub mod documentation;
pub mod error;
pub mod home;
pub mod login;

// FIXME: we need a better dynamic url generation solution. We cannot use url_for because it breaks
// when running behind a reverse proxy (all URLs it generates are for 127.0.0.1 which is freaking
// useless)
pub const STATIC: &str = "/static2/";

pub trait Page {
    fn title(&self) -> String;
    fn description(&self) -> String;

    fn scripts(&self) -> Vec<&str>;
    fn stylesheets(&self) -> Vec<&str>;

    fn body(&self) -> Markup;

    fn head(&self) -> Vec<Markup>;

    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" prefix="og: http://opg.me/ns#" {
                head {
                    title {
                        (self.title())
                    }

                    @if let Some(analytics_tag) = config::google_analytics_tag() {
                        (PreEscaped(format!(
r#"<script async src="https://www.googletagmanager.com/gtag/js?id={0}"></script>
<script>
window.dataLayer = window.dataLayer || [];
function gtag(){{dataLayer.push(arguments);}}
gtag('js', new Date());

gtag('config', '{0}');
</script>"#,
                        analytics_tag)));
                    }
                    meta property="og:site_name" content="pointercrate";
                    meta property="og:type" content="website";
                    meta property="og:title" content = (self.title());
                    meta property="og:description" content = (self.description());

                    meta name="referrer" content = "no-referrer";
                    meta name ="viewport" content="initial-scale=1, maximum-scale=1";
                    meta name="author" content = "kyurime";
                    meta name="keywords" content ="geometry,dash,hardest,insane,demon,list,demonlist,hardest,levels,gmd,gd,game,top,1.9,gdps";
                    meta name="description" content = (self.description());
                    meta http-equiv="Content-Type" content = "text/html; charset=utf-8";
                    meta http-equiv="Content-Style-Type" content="text/css";

                    link rel="icon" type="image/png" href={(STATIC) "images/19diamond.png"};

                    @for markup in self.head() {
                        {(markup)}
                    }

                    script src = "https://ajax.googleapis.com/ajax/libs/jquery/3.1.1/jquery.min.js" {}
                    script src = "https://ajax.googleapis.com/ajax/libs/jqueryui/1.12.1/jquery-ui.min.js" {}

                    script src = {(STATIC) "js/nav.v2.js"} {}
                    script src = {(STATIC) "js/misc.v2.js"} {}
                    script src = {(STATIC) "js/ui.v2.js"} {}

                    @for script in self.scripts() {
                        script src = {(STATIC)(script)} type="module" {}
                    }

                    link rel = "stylesheet" href = "/static2/fa/css/all.min.css";
                    link rel = "stylesheet" href = "https://fonts.googleapis.com/css?family=Noto+Sans&display=swap";

                    link rel = "stylesheet" href = {(STATIC) "css/core/icon.v2.css"};
                    link rel = "stylesheet" href = {(STATIC) "css/core/nav.v2.css"};
                    link rel = "stylesheet" href = {(STATIC) "css/core/ui.v2.1.css"};
                    link rel = "stylesheet" href = {(STATIC) "css/core/core.v2.css"};
                    link rel = "stylesheet" href = {(STATIC) "css/main.v2.1.css"};

                    @for sheet in self.stylesheets() {
                        link rel = "stylesheet" href = {(STATIC) (sheet)};
                    }
                }
                body style = "z-index: -10"{
                    div style={"width: 100%;height: 100%;position: fixed;top: 0;left: 0;background-size: cover;background-repeat: repeat-y;pointer-events: none; z-index:-1"} {}
                    (nav_bar())
                    (self.body())
                    (footer())
                }
            }
        }
    }
}

pub fn nav_bar() -> Markup {
    html! {
        header {
            nav.center.collapse.underlined.see-through {
                div.nav-icon {
                    a href = "/" {
                        img src = {(STATIC) "images/19diamond.png"} style="height:55px";
                    }
                }
                div.nav-group-right.nav-group {
                    a.nav-item.hover.dark-grey href = "/demonlist/" {
                        span.flex.col {
                            span style ="font-size: 50%" {"Geometry Dash"}
                            span {"DEMONLIST"}
                        }
                        i.fas.fa-sort-down style = "height: 50%; padding-left: 5px" {}
                    }
                    ul.nav-hover-dropdown {
                        li {
                            a.hover href = "/demonlist/?statsviewer=true" {"Stats Viewer"}
                        }
                        li {
                            a.hover href = "/demonlist/?submitter=true" {"Record Submitter"}
                        }
                        li {
                            a.hover href = "/demonlist/?timemachine=true" { "Time Machine" }
                        }
                    }
                }
                div.nav-item.collapse-button {
                    div.hamburger.hover {
                        input type="checkbox"{}
                        span{}
                        span{}
                        span{}
                    }
                }
            }
            div {} // artificial spacing
        }
    }
}

pub fn footer() -> Markup {
    let first_extended = config::list_size() + 1;
    let first_legacy = config::extended_list_size() + 1;

    html! {
        footer.center {
            span.overline.pad style="text-align:center" {
                "The 1.9 Demonlist is not affiliated with the official GD Demonlist"
                br;
                "Original code and copyright can be found on pointercrate.com"
                br;
                "All rights reserved"
                br;
            }
            div.flex.no-stretch {
                nav {
                    h2 { "pointercrate:" }
                    a.link.js-scroll {
                        "Back to top"
                    }
                    br ;
                    a.link href = "/#contact" {
                        "Contact"
                    }
                    br ;
                    a.link href = "/documentation/" {
                        "API Documentation"
                    }
                    br ;
                    a.link href = "/login/" {
                        "Staff Area"
                    }
                }
                div {
                    h2 { "Terms of Use:" }
                    "All content on pointercrate.com is provided free of charge. However, you may not redistribute, in any way, any original content found here without the creator's explicit permission. All content is provided without any guarantees."
                }
                nav {
                    h2 {
                        "Demonlist:"
                    }
                    a.link href="/demonlist/1/" title = "Hardest demon" {
                        "Current top demon"
                    }
                    br;
                    a.link href = {"/demonlist/" (first_legacy) "/"} title="Legacy list" {
                        "Legacy List"
                    }
                }
            }
            div style="display: flex; justify-content: flex-end; align-items: center" {
                i class = "fab fa-twitter fa-2x" {}
                (PreEscaped("&nbsp;&nbsp;Tweet Us:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"))
                a href="https://twitter.com/stadust1971" target="_blank" style = "color: #666" {
                    "Developer"
                }
                (PreEscaped("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"))
                a href = "https://twitter.com/_zmxmx" target = "_black" style = "color: #666" {
                    "Fork Developer"
                }
                (PreEscaped("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;"))
                a href = "https://twitter.com/official19gdps" target = "_black" style = "color: #666" {
                    "1.9 GDPS"
                }
            }
        }
    }
}

pub fn paginator(id: &str, endpoint: &str) -> Markup {
    html! {
        div.flex.col.paginator#(id) data-endpoint = (endpoint) {
            p.info-red.output {}
            div style="min-height: 450px; position:relative; flex-grow:1" {
                ul.selection-list style = "position: absolute; top: 0px; bottom:0px; left: 0px; right:0px" {}
            }
            div.flex.no-stretch style = "font-variant: small-caps; font-weight: bolder; justify-content: space-around"{
                div.button.small.prev { "Previous" }
                div.button.small.next { "Next" }
            }
        }
    }
}

pub fn filtered_paginator(id: &str, endpoint: &str) -> Markup {
    html! {
        div.flex.col.paginator#(id) data-endpoint=(endpoint) {
            div.search.seperated.no-stretch {
                input placeholder = "Enter to search..." type = "text" style = "height: 1em";
            }
            p.info-red.output style = "margin: 5px 0px"{}
            div style="min-height: 400px; position:relative; flex-grow:1" {
                ul.selection-list style = "position: absolute; top: 0px; bottom:0px; left: 0px; right:0px" {}
            }
            div.flex.no-stretch style = "font-variant: small-caps; font-weight: bolder; justify-content: space-around"{
                div.button.small.prev { "Previous" }
                div.button.small.next { "Next" }
            }
        }
    }
}

pub fn dropdown(default_entry: &str, default_item: Markup, filter_items: impl Iterator<Item = Markup>) -> Markup {
    html! {
        div.dropdown-menu.js-search.no-stretch {
            input type="text" data-default=(default_entry) autocomplete="off" style = "color: inherit; font-weight: bold;";
            div.menu {
                ul {
                    (default_item)
                    @for item in filter_items {
                        (item)
                    }
                }
            }
        }
    }
}

pub fn simple_dropdown<T1: Display>(dropdown_id: &str, default: Option<T1>, items: impl Iterator<Item = T1>) -> Markup {
    html! {
        div.dropdown-menu.js-search.no-stretch#(dropdown_id) {
            @match default {
                Some(default) => {
                    input type="text" required="" autocomplete="off" data-default=(default) style = "color: #b3b3b3; font-weight: bold;";
                }
                None => {
                    input type="text" required="" autocomplete="off" style = "color: #b3b3b3; font-weight: bold;";
                }
            }

            div.menu {
                ul {
                    @for item in items {
                        li.dark-grey.hover data-value=(item) data-display = (item)  {
                            b {
                                (item)
                            }
                        }
                    }
                }
            }
        }
    }
}
