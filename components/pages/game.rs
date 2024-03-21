use yew::prelude::*;

use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::basketball::boxscore::Boxscore;


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

    if team_stats.is_empty() {
        team_stats.push(html! {
                <div>
                  <h3>{"No boxscore available yet for this game"}</h3>
                </div>
        });
    };
    html! {
        <div>
            <div class="game-header">
            </div>
            <div class="game-team-stats">
                {team_stats}
            </div>
        </div>
    }
}
