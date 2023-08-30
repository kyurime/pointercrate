use crate::{
    components::{
        submitter::{submit_panel, RecordSubmitter},
        team::Team,
        time_machine::Tardis,
    },
    statsviewer::stats_viewer_panel,
};
use maud::{html, Markup, PreEscaped};
use pointercrate_core_pages::{config as page_config, head::HeadLike, PageFragment};
use pointercrate_demonlist::{
    config as list_config,
    demon::{Demon, TimeShiftedDemon},
};

pub struct OverviewPage {
    pub team: Team,
    pub demonlist: Vec<Demon>,
    pub time_machine: Tardis,
    pub submitter_initially_visible: bool,
}

fn demon_panel(demon: &Demon, current_position: Option<i16>) -> Markup {
    html! {
        section.panel.fade style="overflow:hidden" {
            div.flex style = "align-items: center" {
                div.thumb."ratio-16-9"."js-delay-css" style = "position: relative" data-property = "background-image" data-property-value = {"url('" (demon.thumbnail) "')"} {
                    @if let Some(video) = &demon.video {
                        a.play href = (video) {}
                    }
                    @else {
                        a.play href = "https://www.youtube.com/watch?v=dQw4w9WgXcQ" {}
                    }
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
        // Gotta put the ads in this method although they dont belong here, yikes
        @if let Some(publisher_id) = page_config::adsense_publisher_id() {
            @if demon.base.position == 1 {
                section.panel.fade style = "padding: 0px; height: 90px"{
                    (PreEscaped(format!(r#"
                    <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={0}"
    crossorigin="anonymous"></script>
    <!-- Demonlist Responsive Feed Ad -->
    <ins class="adsbygoogle"
    style="display:inline-block;width:728px;height:90px"
    data-ad-client="{0}"
    data-ad-slot="2819150519"></ins>
    <script>
    (adsbygoogle = window.adsbygoogle || []).push({{}});
    </script>
                    "#, publisher_id)))
                }
            }
            // Place ad every 10th demon
            @if demon.base.position % 10 == 0 {
                section.panel.fade {
                (PreEscaped(format!(r#"
                    <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={0}"
    crossorigin="anonymous"></script>
    <ins class="adsbygoogle"
    style="display:block"
    data-ad-format="fluid"
    data-ad-layout-key="-h1+40+4u-93+n"
    data-ad-client="{0}"
    data-ad-slot="5157884729"></ins>
    <script>
    (adsbygoogle = window.adsbygoogle || []).push({{}});
    </script>
                    "#, publisher_id)))
                }
            }
        }
    }
}

impl From<OverviewPage> for PageFragment {
    fn from(page: OverviewPage) -> Self {
        use pointercrate_core_pages::{versioned_import, with_version_string};

        PageFragment::new("1.9 GDPS Demonlist", "The official pointercrate Demonlist!")
            .module(with_version_string!("/static/core/js/modules/form.js"))
            .module(with_version_string!("/static/demonlist/js/modules/demonlist.js"))
            .module(with_version_string!("/static/demonlist/js/demonlist.js"))
            .import(versioned_import!("/static/demonlist/js/modules/demonlist.js"))
            .import(versioned_import!("/static/core/js/modules/form.js"))
            .stylesheet(with_version_string!("/static/demonlist/css/demonlist.css"))
            .stylesheet(with_version_string!("/static/core/css/sidebar.css"))
            .head(page.head())
            .body(page.body())
    }
}

impl OverviewPage {
    fn head(&self) -> Markup {
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
                                    "@id": "https://pointercrate.xyze.dev/",
                                    "name": "pointercrate"
                                }
                            },
                            {
                                "@type": "ListItem",
                                "position": 2,
                                "item": {
                                    "@id": "https://pointercrate.xyze.dev/demonlist/",
                                    "name": "demonlist"
                                }
                            }
                        ]
                    },
                    "name": "1.9 GDPS Demonlist",
                    "description": "The official 1.9 GDPS Demonlist!",
                    "url": "https://pointercrate.xyze.dev/demonlist/"
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
            link ref = "canonical" href = "https://pointercrate.xyze.dev/demonlist/";
        }
    }

    fn body(&self) -> Markup {
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
                                    (demon_panel(current_demon, Some(*position_now)))
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
