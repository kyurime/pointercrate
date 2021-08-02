use crate::{
    config,
    gd::GDIntegrationResult,
    model::demonlist::demon::{FullDemon, MinimalDemon},
    state::PointercrateState,
    video,
    view::{
        demonlist::overview::{DemonlistOverview, OverviewQueryData},
        Page,
    },
    ViewResult,
};
use actix_web::{web::Path, HttpResponse};
use actix_web_codegen::get;
use chrono::NaiveDateTime;
use dash_rs::Thunk;
use log::error;
use maud::{html, Markup, PreEscaped, Render};

#[derive(Debug)]
pub struct DemonMovement {
    from_position: i16,
    at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Demonlist {
    overview: DemonlistOverview,
    data: FullDemon,
    movements: Vec<DemonMovement>,
    link_banned: bool,
    integration: GDIntegrationResult,
}

#[get("/demonlist/permalink/{id}/")]
pub async fn demon_permalink(state: PointercrateState, id: Path<i32>) -> ViewResult<HttpResponse> {
    let mut connection = state.connection().await?;
    let demon = MinimalDemon::by_id(id.into_inner(), &mut connection).await?;

    Ok(actix_web::HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, format!("/demonlist/{}/", demon.position))
        .finish())
}

#[get("/demonlist/{position}/")]
pub async fn page(state: PointercrateState, position: Path<i16>) -> ViewResult<HttpResponse> {
    let mut connection = state.connection().await?;
    let overview = DemonlistOverview::load(&mut connection, None, OverviewQueryData::default()).await?;
    let demon = FullDemon::by_position(position.into_inner(), &mut connection).await?;
    let link_banned = sqlx::query!(
        r#"SELECT link_banned AS "link_banned!: bool" FROM players WHERE id = $1"#,
        demon.demon.verifier.id
    ) // not NULL
    .fetch_one(&mut connection)
    .await?
    .link_banned;

    let mut movements: Vec<DemonMovement> = sqlx::query_as!(
        DemonMovement,
        // note that position is not null as by the WHERE-clause
        r#"SELECT position AS "from_position!: i16", time AS at FROM demon_modifications WHERE position IS NOT NULL AND id = $1 AND position > 0 
         ORDER BY time"#,
        demon.demon.base.id
    )
    .fetch_all(&mut connection)
    .await?;

    let addition = sqlx::query!("SELECT time FROM demon_additions WHERE id = $1", demon.demon.base.id)
        .fetch_optional(&mut connection)
        .await?;

    match addition {
        Some(time) => {
            let from_position = movements.first().map(|m| m.from_position).unwrap_or(demon.demon.base.position);

            movements.insert(0, DemonMovement {
                at: time.time,
                from_position,
            });
        },
        None => error!("No addition logged for demon {}!", demon),
    }

    let integration = state.gd_integration.data_for_demon(state.http_client.clone(), &demon.demon).await?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        Demonlist {
            overview,
            data: demon,
            movements,
            link_banned,
            integration,
        }
        .render()
        .0,
    ))
}

impl Demonlist {
    fn demon_panel(&self) -> Markup {
        let position = self.data.demon.base.position;
        let name = &self.data.demon.base.name;

        let score100 = self.data.demon.score(100);
        let score_requirement = self.data.demon.score(self.data.demon.requirement);

        html! {
            section.panel.fade.js-scroll-anim data-anim = "fade" {
                div.underlined {
                    h1#demon-heading style = "overflow: hidden"{
                        @if self.data.demon.base.position != 1 {
                            a href=(format!("/demonlist/{:?}", self.data.demon.base.position - 1)) {
                                i class="fa fa-chevron-left" style="padding-right: 5%" {}
                            }
                        }
                        (name)
                        @if position as usize != self.overview.demon_overview.len() {
                            a href=(format!("/demonlist/{:?}", position + 1)) {
                                i class="fa fa-chevron-right" style="padding-left: 5%" {}
                            }
                        }
                    }
                    (PreEscaped(format!(r#"
                    <script>
                    document.getElementById("demon-heading").addEventListener('click', () => navigator.clipboard.writeText('https://pointercrate.xyze.dev/demonlist/permalink/{}/'))
                    </script>
                    "#, self.data.demon.base.id)))
                    h3 {
                        @if self.data.creators.len() > 3 {
                            "by " (self.data.creators[0].name) " and "
                            div.tooltip {
                                "more"
                                div.tooltiptext.fade {
                                    (self.data.creators.iter().map(|player| player.name.to_string()).collect::<Vec<_>>().join(", "))
                                }
                            }
                            ", " (self.data.short_headline())
                        }
                        @else {
                            (self.data.headline())
                        }
                    }
                }
                @if let GDIntegrationResult::Success(ref level, ..) = self.integration {
                    @if let Some(Thunk::Processed(ref description)) = level.description {
                        div.underlined.pad {
                            q {
                                (description.0)
                            }
                        }
                    }
                }
                @if self.link_banned {
                    p {
                        "Due to the questionable nature of the verifier's youtube content, embedding of their videos has been disabled"
                    }
                }
                @else {
                    @if let Some(ref video) = self.data.demon.video {
                        @if let Some(embedded_video) = video::embed(video) {
                            iframe."ratio-16-9"."js-delay-attr" style="width:90%; margin: 15px 5%" allowfullscreen="" data-attr = "src" data-attr-value = (embedded_video) {"Verification Video"}
                        }
                    }
                }
                div.underlined.pad.flex.wrap#level-info {
                    @if position <= config::extended_list_size() {
                        span {
                            b {
                                "Demonlist score (100%): "
                            }
                            br;
                            (format!("{:.2}", score100))
                        }
                    }
                    @if position <= config::list_size(){
                        span {
                            b {
                                "Demonlist score (" (self.data.demon.requirement) "%): "
                            }
                            br;
                            (format!("{:.2}", score_requirement))
                        }
                    }
                }
            }
        }
    }

    fn level_info_panel(&self) -> Markup {
        html! {
            section.records.panel.fade.js-scroll-anim data-anim = "fade" {
                div {
                    h2 {
                        "Info"
                    }
                }
                div.flex.wrap#level-info {
                    @match &self.integration {
                        GDIntegrationResult::DemonNotFoundByName => {
                            p.info-red {
                                "A demon with this name was not found on the Geometry Dash servers. Please notify a list moderator of this, as it means they most likely misspelled the name!"
                            }
                        }
                        GDIntegrationResult::DemonNotYetCached => {
                            p.info-yellow {
                                "The data from the Geometry Dash servers has not yet been cached. Please wait a bit and refresh the page."
                            }
                        }
                        GDIntegrationResult::LevelDataNotFound => {
                            p.info-red {
                                "It seems like this level has been deleted from the Geometry Dash servers"
                            }
                        }
                        GDIntegrationResult::LevelDataNotCached => {
                            p.info-red {
                                "This demon's level data is not stored in our database, even though the demon ID was successfully resolved. This either indicates a (hopefully temporary) inconsistent database state, or an error in dash-rs' level data processing. If this error persists, please contact an administrator!"
                            }
                        }
                        GDIntegrationResult::Success(level, level_data, song) => {
                            span {
                                b {
                                    "ID:"
                                }
                                br;
                                (level.level_id)
                            }
                            span {
                                b {
                                    "Length:"
                                }
                                br;
                                @match level_data.level_data {
                                    Thunk::Processed(ref objects) => {
                                        @let length_in_seconds = objects.length_in_seconds();
                                        (format!("{}:{:02}", (length_in_seconds as i32)/ 60, (length_in_seconds as i32) % 60))
                                    }
                                    _ => "unreachable!()"
                                }
                            }
                            span {
                                b {
                                    "Objects:"
                                }
                                br;
                                @match level_data.level_data {
                                    Thunk::Processed(ref objects) => (objects.objects.len()),
                                    _ => "unreachable!()"
                                }
                            }
                            @if let Some(song) = song {
                                span {
                                    b {
                                        "Song:"
                                    }
                                    br;
                                    @match song.link {
                                        Thunk::Processed(ref link) => a.link title = (song.song_id) href = (link.0) {(song.name) " by " (song.artist)},
                                        _ => "unreachable!()"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn records_panel(&self) -> Markup {
        let position = self.data.demon.base.position;
        let name = &self.data.demon.base.name;

        html! {
            @if !self.data.records.is_empty() || position <= config::extended_list_size() {
                section.records.panel.fade.js-scroll-anim data-anim = "fade" {
                    div.underlined.pad {
                        h2 {
                            "Records"
                        }
                        @if position <= config::list_size() {
                            h3 {
                                (self.data.demon.requirement) "% or better required to qualify"
                            }
                        }
                        @else if position <= config::extended_list_size() {
                            h3 {
                                "100% required to qualify"
                            }
                        }
                        @if !self.data.records.is_empty() {
                            h4 {
                                @let records_registered_100_count = self.data.records.iter().filter(|record| record.progress == 100).count();
                                (self.data.records.len())
                                " records registered, out of which "
                                (records_registered_100_count)
                                @if records_registered_100_count == 1 { " is" } @else { " are" }
                                " 100%"
                            }
                        }
                    }
                    @if self.data.records.is_empty() {
                        h3 {
                            @if position > config::extended_list_size() {
                                "No records!"
                            }
                            @else {
                                "No records yet! Be the first to achieve one!"
                            }
                        }
                    }
                    @else {
                        table {
                            tbody {
                                tr {
                                    th.dark-grey {}
                                    th.dark-grey {
                                        "Record Holder"
                                    }
                                    th.dark-grey {
                                        "Progress"
                                    }
                                    th.video-link.dark-grey {
                                        "Video Proof"
                                    }
                                }
                                @for record in &self.data.records {
                                    tr style = { @if record.progress == 100 {"font-weight: bold"} @else {""} } {
                                        td {
                                            @if let Some(ref nationality) = record.nationality {
                                                (*nationality)
                                            }
                                        }
                                        td {
                                            @if let Some(ref video) = record.video {
                                                 a href = (video) target = "_blank"{
                                                    (record.player.name)
                                                 }
                                            }
                                            @else {
                                                (record.player.name)
                                            }
                                        }
                                        td {
                                            (record.progress) "%"
                                        }
                                        td.video-link {
                                            @if let Some(ref video) = record.video {
                                                 a.link href = (video) target = "_blank"{
                                                     (video::host(video))
                                                 }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Page for Demonlist {
    fn title(&self) -> String {
        format!(
            "#{} - {} - Geometry Dash Demonlist",
            self.data.demon.base.position,
            self.data.demon.base.name // FIXME: flatten the structs, holy shit
        )
    }

    fn description(&self) -> String {
        if let GDIntegrationResult::Success(ref level, ..) = self.integration {
            if let Some(Thunk::Processed(ref description)) = level.description {
                return format!("{}: {}", self.title(), description.0)
            }
        }
        format!("{}: <No Description Provided>", self.title())
    }

    fn scripts(&self) -> Vec<&str> {
        vec!["js/modules/formv2.js", "js/modules/demonlistv2.js", "js/demonlist.v2.2.js"]
    }

    fn stylesheets(&self) -> Vec<&str> {
        vec!["css/demonlist.v2.1.css", "css/sidebar.css"]
    }

    fn body(&self) -> Markup {
        let dropdowns = super::dropdowns(&self.overview.demon_overview, Some(&self.data.demon));

        let mut labels = Vec::new();

        let year_only = self.movements.len() > 30;
        let mut last_label = None;

        for movement in &self.movements {
            let would_be_label = if year_only {
                movement.at.date().format("%Y").to_string()
            } else {
                movement.at.date().format("%b %y").to_string()
            };

            match last_label {
                Some(ref label) if &would_be_label == label => labels.push(String::new()),
                _ => {
                    last_label = Some(would_be_label.clone());
                    if labels.is_empty() {
                        labels.push(format!("Added ({})", would_be_label))
                    } else {
                        labels.push(would_be_label)
                    }
                },
            }
        }

        html! {
            (dropdowns)

            div.flex.m-center.container {
                main.left {
                    (super::submission_panel(&self.overview.demon_overview, false))
                    (self.demon_panel())
                    (self.level_info_panel())
                    div.panel.fade.js-scroll-anim.js-collapse data-anim = "fade" {
                        h2.underlined.pad {
                            "Position History"
                            span.arrow.hover {}
                        }
                        div.ct-chart.ct-perfect-fourth.js-collapse-content#position-chart style="display:none" {}
                    }
                    (self.records_panel())
                    (PreEscaped(format!("
                    <script>
                    window.positionChartLabels = ['{}', 'Now'];
                    window.positionChartData = [{},{}];
                    </script>",
                    labels.join("','"),
                    self.movements.iter().map(|movement| movement.from_position.to_string()).collect::<Vec<_>>().join(","), self.data.demon.base.position
                ))) // FIXME: bad
            }
            aside.right {
                    (self.overview.team_panel())
                    (super::submit_panel())
                    (super::rules_panel())
                    (super::stats_viewer_panel())
                    (super::discord_panel())
                }
            }
        }
    }

    fn head(&self) -> Vec<Markup> {
        vec![
            html! {
                (PreEscaped(format!(r##"
                    <link href="https://cdnjs.cloudflare.com/ajax/libs/flag-icon-css/3.4.3/css/flag-icon.min.css" rel="stylesheet">
                    <script type="application/ld+json">
                    {{
                        "@context": "http://schema.org",
                        "@type": "WebPage",
                        "breadcrumb": {{
                            "@type": "BreadcrumbList",
                            "itemListElement": [{{
                                    "@type": "ListItem",
                                    "position": 1,
                                    "item": {{
                                        "@id": "https://pointercrate.xyze.dev/",
                                        "name": "pointercrate"
                                    }}
                                }},{{
                                    "@type": "ListItem",<
                                    "position": 2,
                                    "item": {{
                                        "@id": "https://pointercrate.xyze.dev/demonlist/",
                                        "name": "demonlist"
                                    }}
                                }},{{
                                    "@type": "ListItem",
                                    "position": 3,
                                    "item": {{
                                        "@id": "https://pointercrate.xyze.dev/demonlist/{0}/",
                                        "name": "{1}"
                                    }}
                                }}
                            ]
                        }},
                        "name": "#{0} - {1}",
                        "description": "{2}",
                        "url": "https://pointercrate.xyze.dev/demonlist/{0}/"
                    }}
                    </script>
                "##, self.data.position(), self.data.name(), self.description().render().0)))
            },
            html! {
                (PreEscaped(format!("
                    <script>
                        window.list_length = {0};
                        window.extended_list_length = {1}
                    </script>", config::list_size(), config::extended_list_size()
                )))
            },
            html! {
                   (PreEscaped("<link rel='stylesheet' href='//cdn.jsdelivr.net/chartist.js/latest/chartist.min.css'>
                    <script src='//cdn.jsdelivr.net/chartist.js/latest/chartist.min.js'></script>"))
            },
        ]
    }
}
