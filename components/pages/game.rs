use yew::prelude::*;

use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::basketball;
use basketball::boxscore::Boxscore;


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

    let mut advanced_stats: Html = html! {<div>{"No advanced statistics available"}</div>};
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
        let mut home_name = &meta_teams[0].short_name;
        let mut away_name = &meta_teams[1].short_name;
        if meta_teams[0].id.to_string() != this_game.teams.as_ref().unwrap()[0].id.to_string() {
            home_name = &meta_teams[1].short_name;
            away_name = &meta_teams[0].short_name;
        }

        advanced_stats = html! {
            <div>
              <div>
                <h4>{home_name}</h4>
                <p>{"eFG percentage: "}{tempo_free_stats.home.efgp}</p>
                <p>{"Offensive rebounding percentage: "}{tempo_free_stats.home.orbp}</p>
                <p>{"Defensive rebounding percentage: "}{tempo_free_stats.home.drbp}</p>
                <p>{"Free throw factor: "}{tempo_free_stats.home.ftf}</p>
                <p>{"Points per posession: "}{tempo_free_stats.home.ppp}</p>
              </div>
              <div>
                <h4>{away_name}</h4>
                <p>{"eFG percentage: "}{tempo_free_stats.away.efgp}</p>
                <p>{"Offensive rebounding percentage: "}{tempo_free_stats.away.orbp}</p>
                <p>{"Defensive rebounding percentage: "}{tempo_free_stats.away.drbp}</p>
                <p>{"Free throw factor: "}{tempo_free_stats.away.ftf}</p>
                <p>{"Points per posession: "}{tempo_free_stats.away.ppp}</p>
              </div>
            </div>
        };
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
