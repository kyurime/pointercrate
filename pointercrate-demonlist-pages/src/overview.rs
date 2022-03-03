use crate::{
    components::{
        submitter::{submit_panel, RecordSubmitter},
        team::Team,
        time_machine::Tardis,
    },
    statsviewer::stats_viewer_panel,
};
use maud::{html, Markup, PreEscaped};
use pointercrate_core_pages::{config as page_config, PageFragment, Script};
use pointercrate_demonlist::{
    config as list_config,
    demon::{Demon, TimeShiftedDemon},
};
use std::borrow::Cow;
use url::Url;

pub struct OverviewPage {
    pub team: Team,
    pub demonlist: Vec<Demon>,
    pub time_machine: Tardis,
    pub submitter_initially_visible: bool,
}

fn demon_panel(demon: &Demon, current_position: Option<i16>) -> Markup {
    html! {
        section.panel.fade style="overflow:hidden" {
            @if let Some(ref video) = demon.video {
                div.flex style = "align-items: center" {
                    div.thumb."ratio-16-9"."js-delay-css" style = "position: relative" data-property = "background-image" data-property-value = {"url('" (thumbnail(video)) "')"} {
                        a.play href = (video) {}
                    }
                    div style = "padding-left: 15px" {
                        h2 style = "text-align: left; margin-bottom: 0px" {
                            a href = {"/demonlist/permalink/" (demon.base.id) "/"} {
                                "#" (demon.base.position) (PreEscaped(" &#8211; ")) (demon.base.name)
                            }
                        }
                        h3 style = "text-align: left" {
                            i {
                                (demon.publisher.name)
                            }
                            @if let Some(current_position) = current_position {
                                br;
                                @if current_position > list_config::extended_list_size() {
                                    "Currently Legacy"
                                }
                                @else {
                                    "Currently #"(current_position)
                                }
                            }
                        }
                    }
                }
            }
            @else {
                div.flex.col style = "align-items: center" {
                    h2 style = "margin-bottom: 0px"{
                        a href = {"/demonlist/permalink/" (demon.base.id) "/"} {
                            "#" (demon.base.position) (PreEscaped(" &#8211; ")) (demon.base.name)
                        }
                    }
                    h3 {
                        i {
                            (demon.publisher.name)
                        }
                        @if let Some(current_position) = current_position {
                            br;
                            @if current_position > list_config::extended_list_size() {
                                "Currently Legacy"
                            }
                            @else {
                                "Currently #"(current_position)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PageFragment for OverviewPage {
    fn title(&self) -> String {
        "Geometry Dash Demonlist".to_string()
    }

    fn description(&self) -> String {
        "The official pointercrate Demonlist!".to_string()
    }

    fn additional_scripts(&self) -> Vec<Script> {
        vec![
            Script::module("/static/core/js/modules/form.js"),
            Script::module("/static/demonlist/js/modules/demonlist.js"),
            Script::module("/static/demonlist/js/demonlist.js"),
        ]
    }

    fn additional_stylesheets(&self) -> Vec<String> {
        vec![
            "/static/demonlist/css/demonlist.css".to_string(),
            "/static/core/css/sidebar.css".to_string(),
        ]
    }

    fn head_fragment(&self) -> Markup {
        html! {
            (PreEscaped(r#"
                <script type="application/ld+json">
                {
                    "@context": "http://schema.org",
                    "@type": "WebPage",
                    "breadcrumb": {
                        "@type": "BreadcrumbList",
                        "itemListElement": [
                            {
                                "@type": "ListItem",
                                "position": 1,
                                "item": {
                                    "@id": "https://pointercrate.com/",
                                    "name": "pointercrate"
                                }
                            },
                            {
                                "@type": "ListItem",
                                "position": 2,
                                "item": {
                                    "@id": "https://pointercrate.com/demonlist/",
                                    "name": "demonlist"
                                }
                            }
                        ]
                    },
                    "name": "Geometry Dash Demonlist",
                    "description": "The official pointercrate Demonlist!",
                    "url": "https://pointercrate.com/demonlist/"
                }
                </script>
            "#))
            (PreEscaped(format!("
                <script>
                    window.list_length = {0};
                    window.extended_list_length = {1}
                </script>", list_config::list_size(), list_config::extended_list_size())
            ))
            // FIXME: abstract away
            link ref = "canonical" href = "https://pointercrate.com/demonlist/";
        }
    }

    fn body_fragment(&self) -> Markup {
        let demons_for_dropdown: Vec<&Demon> = match self.time_machine {
            Tardis::Activated { ref demons, .. } => demons.iter().map(|demon| &demon.current_demon).collect(),
            _ => self.demonlist.iter().collect(),
        };

        let dropdowns = super::dropdowns(&demons_for_dropdown[..], None);

        html! {
            (dropdowns)

            div.flex.m-center.container {
                main.left {
                    (self.time_machine)
                    (RecordSubmitter::new(self.submitter_initially_visible, &self.demonlist))

                    @match &self.time_machine {
                        Tardis::Activated { demons, ..} => {
                            @for TimeShiftedDemon {current_demon, position_now} in demons {
                                @if current_demon.base.position <= list_config::extended_list_size() {
                                    (demon_panel(&current_demon, Some(*position_now)))
                                }
                            }
                        },
                        _ => {
                            @for demon in &self.demonlist {
                                @if demon.base.position <= list_config::extended_list_size() {
                                    (demon_panel(demon, None))
                                }
                            }
                        }
                    }
                }

                aside.right {
                    (self.team)
                    (submit_panel())
                    (stats_viewer_panel())
                    (super::discord_panel())
                }
            }
        }
    }
}

fn thumbnail(video: &str) -> String {
    // Videos need to be well formed once we get here!
    let url = Url::parse(video).unwrap();
    let video_id = url
        .query_pairs()
        .find_map(|(key, value)| if key == "v" { Some(value) } else { None })
        .unwrap_or(Cow::Borrowed("dQw4w9WgXcQ"));

    format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", video_id)
}
