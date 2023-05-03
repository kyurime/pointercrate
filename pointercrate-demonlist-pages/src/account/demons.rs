use crate::components::player_selection_dialog;
use maud::{html, Markup, PreEscaped};
use pointercrate_core::permission::PermissionsManager;
use pointercrate_core_pages::util::filtered_paginator;
use pointercrate_demonlist::LIST_MODERATOR;
use pointercrate_user::{sqlx::PgConnection, AuthenticatedUser};
use pointercrate_user_pages::account::AccountPageTab;

pub struct DemonsTab;

#[async_trait::async_trait]
impl AccountPageTab for DemonsTab {
    fn should_display_for(&self, permissions_we_have: u16, permissions: &PermissionsManager) -> bool {
        permissions.require_permission(permissions_we_have, LIST_MODERATOR).is_ok()
    }

    fn initialization_script(&self) -> String {
        use pointercrate_core_pages::with_version_string;
        with_version_string!("/static/demonlist/js/account/demon.js").into()
    }

    fn imports(&self) -> Vec<(String, String)> {
        use pointercrate_core_pages::versioned_import_string;

        vec![
            versioned_import_string!("/static/core/js/modules/form.js"),
            versioned_import_string!("/static/demonlist/js/modules/demonlist.js"),
        ]
    }

    fn tab_id(&self) -> u8 {
        5
    }

    fn tab(&self) -> Markup {
        html! {
            b {
                "Demons"
            }
            (PreEscaped("&nbsp;&nbsp;"))
            i class = "fa fa-shower fa-2x" aria-hidden="true" {}
        }
    }

    async fn content(&self, _user: &AuthenticatedUser, _permissions: &PermissionsManager, _connection: &mut PgConnection) -> Markup {
        html! {
            div.left {
                (demon_submitter())
                div.panel.fade {
                    h2.underlined.pad {
                        "Demon Manager"
                    }
                    div.flex.viewer {
                        (filtered_paginator("demon-pagination", "/api/v2/demons/listed/"))
                        p.viewer-welcome {
                            "Click on a demon on the left to get started!"
                        }

                        div.viewer-content {
                            div.flex.col{
                                h3 style = "font-size:1.1em; margin: 10px 0" {
                                    "Demon #"
                                    i #demon-demon-id {}
                                    " - "
                                    i.fa.fa-pencil-alt.clickable #demon-name-pen aria-hidden = "true" {} (PreEscaped("&nbsp;")) i #demon-demon-name {}
                                }

                                iframe."ratio-16-9"#demon-video style="width:90%; margin: 15px 5%" allowfullscreen="" {"Verification Video"}
                                p.info-red.output style = "margin: 10px" {}
                                p.info-green.output style = "margin: 10px" {}
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-video-pen aria-hidden = "true" {} " Verification Video:"
                                        }
                                        br;
                                        a.link #demon-video-link target = "_blank" {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-thumbnail-pen aria-hidden = "true" {} " Thumbnail:"
                                        }
                                        br;
                                        a.link #demon-thumbnail-link target = "_blank" {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-position-pen aria-hidden = "true" {} " Position:"
                                        }
                                        br;
                                        span #demon-position {}
                                    }
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-requirement-pen aria-hidden = "true" {} " Requirement:"
                                        }
                                        br;
                                        span #demon-requirement {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-publisher-pen aria-hidden = "true" {} " Publisher:"
                                        }
                                        br;
                                        span #demon-publisher {}
                                    }
                                    span{
                                        b {
                                            i.fa.fa-pencil-alt.clickable #demon-verifier-pen aria-hidden = "true" {} " Verifier:"
                                        }
                                        br;
                                        span #demon-verifier {}
                                    }
                                }
                                div.stats-container.flex.space  {
                                    span{
                                        i.fa.fa-plus.clickable #demon-add-creator-pen aria-hidden = "true" {} b {
                                            " Creators:"
                                        }
                                        br;
                                        span #demon-creators {}

                                    }
                                }
                            }
                        }
                    }
                }
                div style="height: 50px" {} // to make sure that the footer doesnt float. if it floats, the user page is the only one without a scrollbar at the right, which causes jumpiness when switching tabs.
            }
            div.right {
                (submit_panel())
            }
            (change_name_dialog())
            (change_position_dialog())
            (change_requirement_dialog())
            (change_video_dialog())
            (change_thumbnail_dialog())
            (change_verifier_dialog())
            (change_publisher_dialog())
            (add_creator_dialog())
        }
    }
}

pub(super) fn submit_panel() -> Markup {
    html! {
        section.panel.fade.js-scroll-anim data-anim = "fade" {
            div.underlined {
                h2 {
                    "Add Demon:"
                }
            }
            a.purple.hover.button.js-scroll data-destination = "demon-submitter" data-reveal = "true" {
                "Add a demon!"
            }
        }
    }
}

fn change_name_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog #demon-name-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change demon name:"
                }
                p style = "max-width: 400px"{
                    "Change the name of this demon. Multiple demons with the same name ARE supported!"
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input #demon-name-edit {
                        label for = "name" {"Name:"}
                        input name = "name" type = "text" required = "";
                        p.error {}
                    }
                    input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_requirement_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog #demon-requirement-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change demon requirement:"
                }
                p style = "max-width: 400px"{
                    "Change the record requirement for this demon. Has be lie between 0 and and 100 (inclusive)."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input #demon-requirement-edit {
                        label for = "requirement" {"Requirement:"}
                        input name = "requirement" type = "number" min = "0" max="100" required = "";
                        p.error {}
                    }
                    input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_position_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog #demon-position-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change demon position:"
                }
                p style = "max-width: 400px"{
                    "Change the position of this demon. Has be be greater than 0 and be at most the current list size."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input #demon-position-edit {
                        label for = "position" {"Position:"}
                        input name = "position" type = "number" min = "1" required = "";
                        p.error {}
                    }
                    input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_video_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog #demon-video-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change verification video link:"
                }
                p style = "max-width: 400px"{
                    "Change the verification video link for this record. Leave empty to remove the verification video."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input #demon-video-edit {
                        label for = "video" {"Video link:"}
                        input name = "video" type = "url";
                        p.error {}
                    }
                    input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_thumbnail_dialog() -> Markup {
    html! {
        div.overlay.closable {
            div.dialog #demon-thumbnail-dialog {
                span.plus.cross.hover {}
                h2.underlined.pad {
                    "Change thumbnail link:"
                }
                p style = "max-width: 400px"{
                    "Change the thumbnail link for this record. To link it to the thumbnail of a youtube video, set it to "
                    i {
                        "https://i.ytimg.com/vi/" b{"VIDEO_ID"} "/mqdefault.jpg"
                    }
                    "."
                }
                form.flex.col novalidate = "" {
                    p.info-red.output {}
                    p.info-green.output {}
                    span.form-input #demon-thumbnail-edit {
                        label for = "thumbnail" {"Thumbnail link:"}
                        input required="" name = "thumbnail" type = "url";
                        p.error {}
                    }
                    input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value = "Edit";
                }
            }
        }
    }
}

fn change_verifier_dialog() -> Markup {
    player_selection_dialog(
        "demon-verifier-dialog",
        "Change demon verifier:",
        "Change the verifier of this demon. If the player you want to change the verifier to already exists, search them up on the left \
         and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
         create a new player.",
        "Edit",
    )
}

fn change_publisher_dialog() -> Markup {
    player_selection_dialog(
        "demon-publisher-dialog",
        "Change demon publisher:",
        "Change the publisher of this demon. If the player you want to change the publisher to already exists, search them up on the left \
         and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
         create a new player.",
        "Edit",
    )
}

fn add_creator_dialog() -> Markup {
    player_selection_dialog(
        "demon-add-creator-dialog",
        "Add creator:",
        "Select a creator to add to this demon. If the player you want to change the publisher to already exists, search them up on the \
         left and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server \
         to create a new player.",
        "Add Creator",
    )
}

fn demon_submitter() -> Markup {
    html! {
        section.panel.fade.closable #demon-submitter style = "display: none" {
            span.plus.cross.hover {}
            form #demon-submission-form novalidate = "" {
                div.underlined {
                    h2 {"Add demon:"}
                }
                p.info-red.output {}
                p.info-green.output {}
                span.form-input.flex.col #demon-add-name {
                    label for = "name" {
                        "Demon name:"
                    }
                    input type = "text" name = "name" required="";
                    p.error {}
                }
                span.form-input.flex.col #demon-add-position {
                    label for = "position" {
                        "Position:"
                    }
                    input type = "number" name = "position" required="" min="1";
                    p.error {}
                }
                span.form-input.flex.col #demon-add-requirement {
                    label for = "requirement" {
                        "Requirement:"
                    }
                    input type = "number" name = "requirement" required="" min="0" max = "100";
                    p.error {}
                }
                span.form-input.flex.col #demon-add-verifier data-type = "html" data-target-id = "selected-verifier" data-default = "None Selected" {
                    label{"Verifier:"}
                    br;
                    span {
                        b {
                            i.fa.fa-pencil-alt.clickable #demon-add-verifier-pen aria-hidden = "true" {}
                            " "
                        }
                        i #selected-verifier data-name = "verifier" {"None Selected"}
                    }
                    p.error {}
                }
                span.form-input.flex.col #demon-add-publisher data-type = "html" data-target-id = "selected-publisher" data-default = "None Selected" {
                    label {"Publisher:"}
                    br;
                    span {
                        b {
                            i.fa.fa-pencil-alt.clickable #demon-add-publisher-pen aria-hidden = "true" {}
                            " "
                        }
                        i #selected-publisher data-name = "publisher" {"None Selected"}
                    }
                    p.error {}
                }
                span.form-input.flex.col #demon-add-video {
                    label for = "video" {
                        "Verification Video:"
                    }
                    input type = "url" name = "video";
                    p.error {}
                }
                span {
                    i.fa.fa-plus.clickable #add-demon-add-creator-pen aria-hidden = "true" {} i {
                        " Creators: "
                    }
                    span #demon-add-creators {}
                }
                input.button.purple.hover type = "submit" style = "margin: 15px auto 0px;" value="Add Demon";
            }
        }
        (player_selection_dialog(
            "demon-add-verifier-dialog",
            "Set demon verifier:",
            "Set the verifier of this demon. If the player you want to set as verifier already exists, search them up on the left \
             and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
             create a new player.",
            "Select",
        ))
        (player_selection_dialog(
            "demon-add-publisher-dialog",
            "Set demon publisher:",
            "Set the publisher of this demon. If the player you want to set as publisher already exists, search them up on the left \
             and click them. In case the player does not exist, fill out only the text field on the right. This will prompt the server to \
             create a new player.",
            "Select",
        ))
    }
}
