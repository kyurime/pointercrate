use super::Page;
use crate::{
    actor::database::GetMessage, api::PCResponder, context::RequestData, model::user::User,
    permissions::Permissions, state::PointercrateState,
};
use actix_web::{AsyncResponder, HttpRequest, Responder};
use maud::{html, Markup, PreEscaped};
use tokio::prelude::Future;

#[derive(Debug)]
struct Homepage {
    demonlist_team: Vec<User>,
    pointercrate_team: Vec<User>,
}

pub fn handler(req: &HttpRequest<PointercrateState>) -> PCResponder {
    let req_clone = req.clone();

    req.state()
        .database(GetMessage::new(
            (Permissions::ListAdministrator, Permissions::Administrator),
            RequestData::Internal,
        ))
        .map(move |(demonlist_team, pointercrate_team)| {
            Homepage {
                demonlist_team,
                pointercrate_team,
            }
            .render(&req_clone)
            .respond_to(&req_clone)
            .unwrap()
        })
        .responder()
}

impl Page for Homepage {
    fn title(&self) -> String {
        "Home".to_owned()
    }

    fn description(&self) -> String {
        "Pointercrate is the home of the official Geometry Dash demonlist, a ranking of the hardest rated demons maintained by some of the game's most skilled players".to_owned()
    }

    fn scripts(&self) -> Vec<&str> {
        vec![]
    }

    fn stylesheets(&self) -> Vec<&str> {
        vec!["css/home.css"]
    }

    fn body(&self, req: &HttpRequest<PointercrateState>) -> Markup {
        html! {
            div.tabbed.information-banner.left {
                div.tab-display {
                    div.information {
                        div style = "display: flex; flex-flow: column;"{
                            h1 style="text-align: left; margin-top: 0px" {
                                "Pointercrate"
                            }
                            h2 style="text-align: left" {
                                "Home of the official Geometry Dash Demonlist"
                            }
                            div.tab-content.tab-content-active data-tab-id ="1" {
                                "The pointercrate demonlist is the most popular ranking of the game's hardest demons with multiple thousand visitors each day! Even RobTop himself likes it!"
                            }
                            div.tab-content data-tab-id = "2" {
                                "The demonlist stats viewer assigns each player a score based on how many demons they've beaten and then ranks them, showing exactly who's the best!"
                            }
                            div.tab-content data-tab-id = "3" {
                                "Each submitted record on the demonlist is manually accepted or rejected by our competent list editors!"
                            }
                            div.tab-content data-tab-id = "4" {
                                "Thanks to our specialized way of connecting to the Geometry Dash servers we are able to display a whole range of information about the demons, including their description, download count and much more!"
                            }
                        }
                        div.tab-selection.flex.wrap style="padding: 20px 0px; text-align: center"{
                            div.tab.tab-active.hover.scale data-tab-id="1" style="padding: 10px" {
                                h3 {
                                    "Ranking"
                                }
                                i class = "fa fa-list-ol fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="2" style="padding: 10px" {
                                h3 {
                                    "Stats Viewer"
                                }
                                i class = "fa fa-globe fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="3" style="padding: 10px" {
                                h3 {
                                    "Records"
                                }
                                i class = "fa fa-trophy fa-2x" aria-hidden="true" {}
                            }
                            div.tab.hover.scale data-tab-id="4" style="padding: 10px" {
                                h3 {
                                    "Informative"
                                }
                                i class = "fa fa-info fa-2x" aria-hidden="true" {}
                            }
                        }
                    }
                    a.big.dark-grey.hover.button.js-scroll-anim data-anim="fade" href = "/demonlist/"{
                        "Check it out"(PreEscaped("&nbsp;&nbsp;&nbsp;"))
                        i.fa.fa-arrow-right aria-hidden="true" {}
                    }
                }
            }
            div.center.information-banner.right {
                div {
                    a.big.dark-grey.hover.button.js-scroll-anim data-anim="fade" href = "https://github.com/stadust/pointercrate" target = "_blank"{
                        i.fa.fa-github aria-hidden="true" {}
                        (PreEscaped("&nbsp;&nbsp;&nbsp;"))
                        "To the repository"
                    }
                    div.information {
                        h2 { "Now on GitHub "}
                        h3 { "The entirety of the pointercrate codebase can now be found on GitHub"}
                        p{"Found a bug on the website? Want to help with development? Or maybe you just want to find out how everything here works? Head over to the pointercrate GitHub repository!"}
                        p{"Even our custom Geometry Dash API wrapper, GDCF, can be found there!"}
                    }
                }
            }
            div.tabbed.center.information-banner.left#changelog {
                div.tab-display {
                    div style = "display: flex; flex-flow: column;"{
                        h2 style="text-align: left; margin-top: 0px" {
                            "Changelog"
                        }
                        div.tab-content.tab-content-active data-tab-id ="99" {
                            h3 style="text-align: left; font-size: 110%" {
                                "2019-04-13: Making it ours.."
                            }
                            p {
                                "I literally don't understand rust. Like, " i{"at all"}
                            }
                        }
                    }
                    div.tab-selection style="padding: 20px 0px; text-align: center"{
                        h3.tab.tab-active data-tab-id="99" style="padding: 10px; text-align:left" { "2019-04-13" }
                    }
                }
            }
            div.center.information-banner.right {
                div style = "flex-flow: column" {
                    h2#contact {
                        "Contact"
                    }
                    div.flex#about-inner {
                        div style = "flex-basis: 0; padding: 5px" {
                            h3 { "1.9 Demonlist Team: "}
                            p {
                                "The demonlist is managed by a large team of players lead by:"
                            }
                            div.flex.wrap style = "padding: 20px" {
                                @for member in &self.demonlist_team {
                                    h4 style="display: inline; margin: 5px" { (member.name()) }
                                }
                            }
                            p {
                                "Contact these people for any list related questions/issues"
                            }
                            i {
                                "Website: "
                                a href = "https://absolllute.com/gdps/" {"GDPS Website"}
                            }
                            br ;
                            i {
                                "Twitter: "
                                a href = "https://twitter.com/official19gdps" {"official19gdps"}
                            }
                            br ;
                            i {
                                "YouTube: "
                                a href = "https://www.youtube.com/channel/UCIUpOcn9GZ-IlEw34czouIg" {"1.9 GDPS"}
                            }
                            br ;
                            i {
                                "Discord: "
                                a href = "https://discord.gg/eCGFrCG" {"1.9 GDPS Server"}
                            }
                        }
                        div style = "flex-basis: 0; padding: 5px" {
                            h3 { "Pointercrate Team: "}
                            p {
                                "Pointercrate as an entity independent from the demonlist is administrated and moderated by the following people:"
                            }
                            div.flex.wrap style = "padding: 20px" {
                                @for member in &self.pointercrate_team {
                                    h4 style="display: inline; margin: 5px" { (member.name()) }
                                }
                            }
                            p {
                                "Contact these people for suggestion for pointercrate itself, bug reports or programming related questions"
                            }
                            i {
                                "Twitter: "
                                a href = "https://twitter.com/stadust1971" {"stadust - pointercrate"}
                            }
                            br ;
                            i {
                                "Discord: "
                                a href = "https://discord.gg/sQewUEB" {"Pointercrate Central"}
                            }
                        }
                    }
                }
            }
        }
    }

    fn head(&self, _: &HttpRequest<PointercrateState>) -> Vec<Markup> {
        vec![html! {
            (PreEscaped(r#"
<style>
    .tab-active {
        color: #0881c6;
    }
</style>
<script type="application/ld+json">
  {
    "@context": "http://schema.org",
    "@type": "Organization",
    "name": "pointercrate",
    "description": "Pointercrate is the home of the official Geometry Dash demonlist, a ranking of the hardest rated demons maintained by some of the game's most skilled players",
    "url": "https://pointercrate.com/",
    "logo": "https://pointercrate.com/static2/images/pointercrate2.png",
    "sameAs": [
      "https://twitter.com/demonlistgd",
      "https://www.youtube.com/channel/UCqI5feGZEqJRp6VcrP5gVyw"
    ]
  }
</script>
            "#))
        }]
    }
}
