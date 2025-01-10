
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

use ncaa_data_rs::ncaa::scoreboard::Game;

#[derive(Properties, PartialEq)]
pub struct GameCardProps {
    pub index: usize,
    pub game: Game
}

#[function_component]
pub fn GameCard(props: &GameCardProps) -> Html {

    let game_id = props.game.details.id.clone();
    let game_clock = if props.game.details.contest_clock.is_empty() {
        html!{
            <div class="scoreboard-gameclock">
              <p>{props.game.details.start_time.clone()}</p>
            </div>
        }
    } else {
        if props.game.details.current_period == "FINAL" {
            html! {
              <div class="scoreboard-gameclock">
                <p>{props.game.details.current_period.clone()}</p>
              </div>
            }
        } else if props.game.details.contest_clock == ":00" {
            html! {
                <div class="scoreboard-gameclock">
                  <p>{"HALFTIME"}</p>
                </div>
            }
        } else {
            html!{
                <div class="scoreboard-gameclock">
                  <p>{props.game.details.current_period.clone()}</p>
                  <p>{props.game.details.contest_clock.clone()}</p>
                </div>
            }
        }
    };

    html! {
        <div class="gamecard">
            <Link<MainRoute> classes="link" to={MainRoute::Game { id: game_id.to_string().clone() }}>
                <div class="scoreboard-card" id={props.game.details.id.clone()}>
                    <div class="scoreboard-team-container home">
                        <h4>{props.game.details.home.score.as_str().clone()}</h4>
                        <div class="scoreboard-team-details">
                          <h5>{props.game.details.home.names.short.clone()}</h5>
                          <p>{props.game.details.home.description.clone()}</p>
                        </div>
                        
                    </div>
                    <div class="scoreboard-team-container away">
                        <h4>{props.game.details.away.score.as_str().clone()}</h4>
                        <div class="scoreboard-team-details">
                          <h5>{props.game.details.away.names.short.clone()}</h5>
                          <p>{props.game.details.away.description.clone()}</p>
                        </div>
                    </div>
                    {game_clock.clone()}
                </div>
            </Link<MainRoute>>
        </div>
    }
}
