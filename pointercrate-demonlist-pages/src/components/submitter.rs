use crate::components::{demon_dropdown, player_selection_dialog};
use maud::{html, Markup, Render};
use pointercrate_demonlist::{config, demon::Demon};

pub struct RecordSubmitter<'a> {
    initially_visible: bool,
    demons: &'a [Demon],
}

impl RecordSubmitter<'_> {
    pub fn new(visible: bool, demons: &[Demon]) -> RecordSubmitter {
        RecordSubmitter {
            initially_visible: visible,
            demons,
        }
    }
}

impl Render for RecordSubmitter<'_> {
    fn render(&self) -> Markup {
        html! {
            section.panel.fade.closable#submitter style=(if !self.initially_visible {"display:none"} else {""}) {
                span.plus.cross.hover {}
                form#submission-form novalidate = "" {
                    div.underlined {
                        h2 {"Record Submission"}
                    }
                    p.info-red.output {}
                    p.info-green.output {}
                    h3 {
                        "Demon:"
                    }
                    p {
                        "The demon the record was made on. Only demons in the top " (config::extended_list_size()) " are accepted. This excludes legacy demons!"
                    }
                    span.form-input data-type = "dropdown" {
                        (demon_dropdown("id_demon", self.demons.iter().filter(|demon| demon.base.position <= config::extended_list_size())))
                        p.error {}
                    }
                    h3 {
                        "Holder:"
                    }
                    p {
                        "The holder of the record. Please enter the holders Geometry Dash name here, even if their YouTube name differs! Click the pencil to select a player!"
                    }
                    span.form-input.flex.col#id_player data-type = "html" data-target-id = "selected-holder" data-default = "None Selected" {
                        span {
                            b {
                                i.fa.fa-pencil-alt.clickable#record-submitter-holder-pen aria-hidden = "true" {}
                                " "
                            }
                            i#selected-holder data-name = "player" {"None Selected"}
                        }
                        p.error {}
                    }
                    h3 {
                        "Progress:"
                    }
                    p {
                        "The progress made as percentage. Only values greater than or equal to the demons record requirement and smaller than or equal to 100 are accepted!"
                    }
                    span.form-input.flex.col#id_progress {
                        input type = "number" name = "progress" required="" placeholder = "e. g. '50', '98'" min="0" max="100";
                        p.error {}
                    }
                    h3 {
                        "Video: "
                    }
                    p {
                        "A proof video of the legitimacy of the given record. If the record was achieved on stream, but wasn't uploaded anywhere else, please provide a twitch link to that stream."
                        br {}

                        i { "Note: " }
                        "Please pay attention to only submit well-formed URLs!"
                    }
                    span.form-input.flex.col#id_video {
                        input type = "url" name = "video" required = "" placeholder = "e.g. 'https://youtu.be/cHEGAqOgddA'" ;
                        p.error {}
                    }
                    h3 {
                        "Raw footage: "
                    }
                    p {
                        "The unedited and untrimmed video for this completion, uploaded to a non-compressing (e.g. not YouTube) file-sharing service such as google drive. If the record was achieved on stream (meaning there is no recording), please provide a link to the stream VOD"
                    }
                    p {
                        "Any personal information possibly contained within raw footage (e.g. names, sensitive conversations) will be kept strictly confidential and will not be shared outside of the demonlist team. Conversely, you acknowledge that you might inadvertently share such information by providing raw footage. You have the right to request deletion of your record note by contacting a list administrator."
                    }
                    p {
                        i {"Note: "} "This is required for every first record and recommended for big jumps!"
                    }
                    span.form-input.flex.col#submit-raw-footage {
                        input type = "url"  name = "raw_footage" placeholder = "https://drive.google.com/file/d/.../view?usp=sharing" {}
                        p.error {}
                    }
                    h3 {
                        "Notes or comments: "
                    }
                    p {
                        "Provide any additional notes you'd like to pass on to the list moderator receiving your submission."
                    }
                    span.form-input.flex.col#submit-note {
                        textarea name = "note" placeholder = "Your dreams and hopes for this record... or something like that" {}
                        p.error {}
                    }
                    p {
                        "By submitting the record you acknowledge the " a.link href = "/guidelines" {"submission guidelines"} "."
                    }
                    input.button.blue.hover type = "submit" style = "margin: 15px auto 0px;" value="Submit record";
                }
            }
            (player_selection_dialog(
                "submission-holder-dialog",
                "Select player:",
                "To select the player holding this record, search them up on the left to see if they already have records on the list and click them. In case the player does not exist, fill out only the text field on the right.",
                "Select"
            ))
        }
    }
}

pub(crate) fn submit_panel() -> Markup {
    html! {
        section#submit.panel.fade.js-scroll-anim data-anim = "fade" {
            div.underlined {
                h2 {
                    "Submit Records:"
                }
            }
            p {
                "Note: Please do not submit nonsense, it only makes it harder for us all and will get you banned. Also note that the form rejects duplicate submissions."
            }
            a.purple.hover.button.js-scroll data-destination = "submitter" data-reveal = "true" {
                "Submit a record!"
            }
        }
    }
}
