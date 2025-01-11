
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

use ncaa_data_rs::ncaa::sport::Get;
use ncaa_data_rs::ncaa::scoreboard::Game;

use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct GameCardProps<T: PartialEq> {
    pub id: String,
    pub sport: T,
    pub game: Game
}

#[function_component]
pub fn GameCard<T>(props: &GameCardProps<T>) -> Html
where
  T: PartialEq + Clone + Default + Get + 'static,
{
    let sport = use_state(|| props.sport.clone());
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
            <Link<MainRoute> classes="link" to={MainRoute::Game {
                sport: sport.deref().sport(),
                variation: sport.deref().variation(),
                division: sport.deref().division(),
                id: props.id.clone(),
            }}>
                <div class="scoreboard-team-container home">
                    <h4 class="scoreboard-team-score home">{props.game.details.home.score.as_str().clone()}</h4>
                    <div class="scoreboard-team-details">
                      <h5 class="scoreboard-team-name home">
                        <p class="scoreboard-team-rank home">{props.game.details.home.rank.as_str()}</p>
                        {props.game.details.home.names.char6.clone()}
                      </h5>
                      <p class="scoreboard-team-record home">{props.game.details.home.description.clone()}</p>
                    </div>
                    
                </div>
                <div class="scoreboard-team-container away">
                    <h4 class="scoreboard-team-score away">{props.game.details.away.score.as_str().clone()}</h4>
                    <div class="scoreboard-team-details">
                      <h5 class="scoreboard-team-name">
                        {props.game.details.away.names.char6.clone()}
                        <p class="scoreboard-team-rank away">{props.game.details.away.rank.as_str()}</p>
                      </h5>
                      <p class="scoreboard-team-record away">{props.game.details.away.description.clone()}</p>
                    </div>
                </div>
                {game_clock.clone()}
            </Link<MainRoute>>
        </div>
    }
}
