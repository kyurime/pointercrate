use super::Page;
use crate::{model::user::User, permissions::Permissions, state::PointercrateState, ViewResult};
use actix_web::HttpResponse;
use actix_web_codegen::get;
use maud::{html, Markup, PreEscaped};

#[derive(Debug)]
struct Homepage {
    demonlist_team: Vec<User>,
    pointercrate_team: Vec<User>,
}

#[get("/")]
pub async fn index(state: PointercrateState) -> ViewResult<HttpResponse> {
    let mut connection = state.connection().await?;

    let demonlist_team = User::by_permission(Permissions::ListAdministrator, &mut connection).await?;
    let pointercrate_team = User::by_permission(Permissions::Administrator, &mut connection).await?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        Homepage {
            demonlist_team,
            pointercrate_team,
        }
        .render()
        .0,
    ))
}

impl Page for Homepage {
    fn title(&self) -> String {
        "Home".to_owned()
    }

    fn description(&self) -> String {
        "This is the home of the Geometry Dash 1.9 Private Server demonlist"
            .to_owned()
    }

    fn scripts(&self) -> Vec<&str> {
        vec!["js/home.js", "js/modules/tab.mjs"]
    }

    fn stylesheets(&self) -> Vec<&str> {
        vec!["css/home.css"]
    }

    fn body(&self) -> Markup {
        html! {
            div.information-banner.left {
                div.tab-display#information-tabs {
                    div.information {
                        div style = "display: flex; flex-flow: column;"{
                            h1 style="text-align: left; margin-top: 0px" {
                                "Pointercrate"
                            }
                            h2 style="text-align: left" {
                                "Home of the official Geometry Dash 1.9 Private Server Demonlist"
                            }
                            div.tab-content.tab-content-active data-tab-id ="1" {
                                "The pointercrate 1.9 demonlist is the only ranking of 1.9 PS' hardest demons!"
                            }
                            div.tab-content data-tab-id = "2" {
                                "The Demonlist stats viewer assigns each player a score based on how many demons they've beaten and then ranks them, showing exactly who's the best!"
                            }
                            div.tab-content data-tab-id = "3" {
                                "Each submitted record on the Demonlist is manually accepted or rejected by our competent list editors!"
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
                    a.big.dark-grey.hover.button.js-scroll-anim data-anim="fade" href = "https://github.com/zmxhawrhbg/pointercrate" target = "_blank"{
                        i.fa.fa-github aria-hidden="true" {}
                        (PreEscaped("&nbsp;&nbsp;&nbsp;"))
                        "To the repository"
                    }
                    div.information {
                        h2 { "GitHub Repo ~"}
                        h3 { "The entirety of the pointercrate codebase - on GitHub!"}
                        p{"Found a bug on the website? Want to help with development? Or maybe you just want to find out how everything here works? Head over to the pointercrate GitHub repository!"}
                    }
                }
            }
            div.center.information-banner.left {
                div.tab-display#changelog-tabs {
                    div style = "display: flex; flex-flow: column;"{
                        h2 style="text-align: left; margin-top: 0px" {
                            "Changelog"
                        }
                        div.tab-content.tab-content-active data-tab-id ="98" {
                            h3 style="text-align: left; font-size: 110%" {
                                "5/22/2020: Updated 2"
                            }
                            p {
                                "Pulled from master... again (still don't understand rust though)"
                            }
                        }
                        div.tab-content data-tab-id ="99" {
                            h3 style="text-align: left; font-size: 110%" {
                                "10/3/2019: Small Updates"
                            }
                            p {
                                "To go with nearly I understand more of rust now, I updated parts of the site " small {"stadust also did it and i'm just going off him but like.. "}
                            }
                            p {"Oh and I also changed some of the design, for hopefully the first and final time.."}
                        }
                        div.tab-content data-tab-id ="100" {
                            h3 style="text-align: left; font-size: 110%" {
                                "4/13/2019: Making it ours.."
                            }
                            p {
                                "I literally don't understand rust. Like, " i{"at all"}
                            }
                        }
                    }
                    aside.tab-selection style="padding: 20px 0px; text-align: center"{
                        h3.tab.tab-active data-tab-id="98" style="padding: 10px; text-align:left" { "2020-05-22" }
                        h3.tab data-tab-id="99" style="padding: 10px; text-align:left" { "2019-10-03" }
                        h3.tab data-tab-id="100" style="padding: 10px; text-align:left" { "2019-04-13" }
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
                                "This demonlist is managed by a large team of players lead by:"
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
                            h3 { "(Rehosted) Pointercrate Team: "}
                            p {
                                "This fork of Pointercrate is administrated and moderated by the following people:"
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

    fn head(&self) -> Vec<Markup> {
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
    "name": "GDPS1.9 Demonlist",
    "description": "This is the home of the Geometry Dash 1.9 PS demonlist",
    "url": "https://pointercrate.xyze.dev/",
    "logo": "https://pointercrate.xyze.dev/static2/images/19diamond.png",
    "sameAs": [
      "https://twitter.com/official19gdps",
      "https://www.youtube.com/channel/UCIUpOcn9GZ-IlEw34czouIg"
    ]
  }
</script>
            "#))
        }]
    }
}
