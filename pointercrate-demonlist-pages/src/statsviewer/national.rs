use crate::statsviewer::{stats_viewer_html, StatsViewerRow};
use maud::{html, Markup};
use pointercrate_core_pages::{PageFragment, Script};

#[derive(Debug)]
pub struct NationBasedStatsViewer;

impl PageFragment for NationBasedStatsViewer {
    fn title(&self) -> String {
        "Nation Stats Viewer".to_owned()
    }

    fn description(&self) -> String {
        "The pointercrate nation stats viewer, ranking how well each nations player's are doing in their quest to collectively complete \
         the entire demonlist!"
            .to_owned()
    }

    fn additional_scripts(&self) -> Vec<Script> {
        vec![
            Script::module("/static/demonlist/js/modules/statsviewer.js"),
            Script::module("/static/demonlist/js/statsviewer/nation.js"),
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
        let mut rows = super::standard_stats_viewer_rows();

        rows[0].0.insert(1, ("Players", "players"));
        rows.push(StatsViewerRow(vec![("Unbeaten demons", "unbeaten")]));

        html! {
            nav.flex.wrap.m-center.fade #statsviewers style="text-align: center;" {
                a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/"{
                    b {"Individual"}
                }
                a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/nations/" {
                    b {"Nations"}
                }
            }
            div.flex.m-center.container {
                main.left {
                    (stats_viewer_html(None, rows))
                }
                aside.right {
                    (super::continent_panel())
                }
            }
        }
    }
}
