use crate::statsviewer::stats_viewer_html;
use maud::{html, Markup};
use pointercrate_core_pages::{PageFragment, Script};
use pointercrate_demonlist::nationality::Nationality;

#[derive(Debug)]
pub struct IndividualStatsViewer {
    pub nationalities_in_use: Vec<Nationality>,
}

impl PageFragment for IndividualStatsViewer {
    fn title(&self) -> String {
        "Individual Stats Viewer".to_owned()
    }

    fn description(&self) -> String {
        "The pointercrate individual stats viewer, a ranking of the worlds best Geometry Dash players. Now more local than ever, allowing \
         you to see who's the best in your state!"
            .to_owned()
    }

    fn additional_scripts(&self) -> Vec<Script> {
        vec![
            Script::module("/static/demonlist/js/modules/statsviewer.js"),
            Script::module("/static/demonlist/js/statsviewer/individual.js"),
        ]
    }

    fn additional_stylesheets(&self) -> Vec<String> {
        vec![
            "/static/demonlist/css/statsviewer.css".to_string(),
            "/static/core/css/sidebar.css".to_string(),
        ]
    }

    fn head_fragment(&self) -> Markup {
        html! {}
    }

    fn body_fragment(&self) -> Markup {
        html! {
            nav.flex.wrap.m-center.fade#statsviewers style="text-align: center;" {
                a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/"{
                    b {"Individual"}
                }
                a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/nations/" {
                    b {"Nations"}
                }
            }
            div#world-map-wrapper {
                object#world-map data="/static/demonlist/images/world.svg" type="image/svg+xml" {}
            }
            div.flex.m-center.container {
                main.left {
                    (stats_viewer_html(Some(&self.nationalities_in_use), super::standard_stats_viewer_rows()))
                }
                aside.right {
                    (super::continent_panel())
                    (super::hide_subdivision_panel())
                    section.panel.fade style = "overflow: initial;" {
                        h3.underlined {
                            "Political Subdivision:"
                        }
                        p {
                            "For the " i {"United States of America"} ", " i {"The United Kingdom of Great Britain and Northern Ireland"} ", " i{"Australia"} " and " i{"Canada"} " you can select a state/province from the dropdown below to focus the stats viewer to that state/province."
                        }
                        div.dropdown-menu.js-search#subdivision-dropdown data-default = "None" {
                            div{
                                input type="text" style = "font-weight: bold;";
                            }
                            div.menu {
                                ul {
                                    li.colorless.hover.underlined data-value = "None" {"None"}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
