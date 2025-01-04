use yew::prelude::*;

use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::basketball;
use basketball::boxscore::Boxscore;

use charming::{
    component::{Legend, RadarCoordinate, Title, VisualMap},
    element::{AreaStyle, TextStyle, Tooltip},
    series::Radar,
    Chart,
    WasmRenderer,
};

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: String,
}

#[function_component]
pub fn GamePage(props: &GameProps) -> Html {
    let this_game = use_state(|| Boxscore::default());
    let id = props.id.clone();

    {
        let this_game = this_game.clone();
        use_effect_with(props.id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match query::boxscore(&id).await {
                    Ok(boxscore) => {
                        this_game.set(boxscore);
                    }
                    Err(_) => {
                        this_game.set(Boxscore::default());
                    }
                }
        })});
    }

    let mut team_stats: Vec<_> = this_game.meta.teams.iter().map(|team| {
        let Some(ref teams) = this_game.teams else { todo!() };
        let boxscore = if team.id == teams[0].id.to_string() {
            &teams[0]
        } else {
            &teams[1]
        };

        let player_stats: Vec<_> = boxscore.player_stats.iter().map(|player| {
            html! {
                <tr>
                    <td>{player.first_name.clone()}</td>
                    <td>{player.last_name.clone()}</td>
                    <th>{player.position.clone()}</th>
                    <th>{player.points.clone()}</th>
                    <th>{player.field_goals_made.clone()}</th>
                    <th>{player.three_points_made.clone()}</th>
                    <th>{player.free_throws_made.clone()}</th>
                    <th>{player.total_rebounds.clone()}</th>
                    <th>{player.offensive_rebounds.clone()}</th>
                    <th>{player.assists.clone()}</th>
                    <th>{player.personal_fouls.clone()}</th>
                    <th>{player.steals.clone()}</th>
                    <th>{player.turnovers.clone()}</th>
                    <th>{player.blocked_shots.clone()}</th>
                    <th>{player.minutes_played.clone()}</th>
                </tr>
        }}).collect();

        html! {
            <div>
                <h3 class="game-team-name">{team.short_name.clone()}</h3>
                <table class="styled-table">
                    <thead>
                        <tr>
                            <th>{"FIRST"}</th>
                            <th>{"LAST"}</th>
                            <th>{boxscore.player_header.position.clone()}</th>
                            <th>{boxscore.player_header.points.clone()}</th>
                            <th>{boxscore.player_header.field_goals_made.clone()}</th>
                            <th>{boxscore.player_header.three_points_made.clone()}</th>
                            <th>{boxscore.player_header.free_throws_made.clone()}</th>
                            <th>{boxscore.player_header.total_rebounds.clone()}</th>
                            <th>{boxscore.player_header.offensive_rebounds.clone()}</th>
                            <th>{boxscore.player_header.assists.clone()}</th>
                            <th>{boxscore.player_header.personal_fouls.clone()}</th>
                            <th>{boxscore.player_header.steals.clone()}</th>
                            <th>{boxscore.player_header.turnovers.clone()}</th>
                            <th>{boxscore.player_header.blocked_shots.clone()}</th>
                            <th>{boxscore.player_header.minutes_played.clone()}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {player_stats}
                        <tr>
                            <td></td>
                            <td></td>
                            <td></td>
                            <td>{boxscore.player_totals.points.clone()}</td>
                            <th>{boxscore.player_totals.field_goals_made.clone()}</th>
                            <th>{boxscore.player_totals.three_points_made.clone()}</th>
                            <th>{boxscore.player_totals.free_throws_made.clone()}</th>
                            <th>{boxscore.player_totals.total_rebounds.clone()}</th>
                            <th>{boxscore.player_totals.offensive_rebounds.clone()}</th>
                            <th>{boxscore.player_totals.assists.clone()}</th>
                            <th>{boxscore.player_totals.personal_fouls.clone()}</th>
                            <th>{boxscore.player_totals.steals.clone()}</th>
                            <th>{boxscore.player_totals.turnovers.clone()}</th>
                            <th>{boxscore.player_totals.blocked_shots.clone()}</th>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }).collect();

    let advanced_stats: Html = html! {<div id="tempo-free-factors"></div>};
    if team_stats.is_empty() {
        team_stats.push(html! {
                <div>
                  <h3>{"No boxscore available yet for this game"}</h3>
                </div>
        });
    } else {
        // some stats exist, so lets calculate!
        let tempo_free_stats = basketball::advanced::calc(
            &this_game.teams.as_ref().unwrap()[0].player_totals,
            &this_game.teams.as_ref().unwrap()[1].player_totals,
        );
        let meta_teams = &this_game.meta.teams;
        let mut meta_home = &meta_teams[0];
        let mut meta_away = &meta_teams[1];
        if meta_teams[0].id.to_string() != this_game.teams.as_ref().unwrap()[0].id.to_string() {
            meta_home = &meta_teams[1];
            meta_away = &meta_teams[0];
        }
        let chart_name = format!("{} {} at {} {}",
            meta_home.short_name.clone().unwrap(),
            meta_home.nickname.clone().unwrap(),
            meta_away.short_name.clone().unwrap(),
            meta_away.nickname.clone().unwrap(),
        );
        let chart = Chart::new()
            .title(Title::new()
                .text(chart_name.clone())
                .text_style(TextStyle::new()
                    .color("white")))
            .legend(Legend::new()
                .bottom("10")
                .text_style(TextStyle::new()
                    .color("white"))
                .data(vec![
                    meta_home.short_name.clone().unwrap(),
                    meta_away.short_name.clone().unwrap()]))
            .radar(RadarCoordinate::new()
                .indicator(vec![
                    ("Shooting (eFG %)", 0, 100),
                    ("Offensive Rebounding (%)", 0, 100),
                    ("Defensive Rebounding (%)", 0, 100),
                    ("Free Throw Factor (FTF)", 0, 100),
                    ("Turnovers (%)", 0, 100),
                    ("Points-per-posession (PPP)", 0, 5)]))
            .tooltip(Tooltip::new())
            .visual_map(VisualMap::new()
                .show(false)
                .series_index(0)
                .color(vec![meta_home.color.clone()]))
            .visual_map(VisualMap::new()
                .show(false)
                .series_index(1)
                .color(vec![meta_away.color.clone()]))
            .series(Radar::new()
                .name("home")
                .area_style(AreaStyle::new())
                .data(vec![(
                    vec![
                        format!("{:.2}", tempo_free_stats.home.efgp),
                        format!("{:.2}", tempo_free_stats.home.orbp),
                        format!("{:.2}", tempo_free_stats.home.drbp),
                        format!("{:.2}", tempo_free_stats.home.ftf),
                        format!("{:.2}", tempo_free_stats.home.tovp),
                        format!("{:.2}", tempo_free_stats.home.ppp),
                    ],
                    meta_home.short_name.clone().unwrap())]))
            .series(Radar::new()
                .name("away")
                .area_style(AreaStyle::new())
                .data(vec![(
                    vec![
                        format!("{:.2}", tempo_free_stats.away.efgp),
                        format!("{:.2}", tempo_free_stats.away.orbp),
                        format!("{:.2}", tempo_free_stats.away.drbp),
                        format!("{:.2}", tempo_free_stats.away.ftf),
                        format!("{:.2}", tempo_free_stats.away.tovp),
                        format!("{:.2}", tempo_free_stats.away.ppp),
                    ],
                    meta_away.short_name.clone().unwrap())]));
    
        let renderer = WasmRenderer::new(600, 500);
        
        renderer.render("tempo-free-factors", &chart).unwrap();
    }

    html! {
        <div>
            <div class="game-header">
            </div>
            <div class="game-advanced-stats">
              {advanced_stats}
            </div>
            <div class="game-team-stats">
                {team_stats}
            </div>
        </div>
    }
}
