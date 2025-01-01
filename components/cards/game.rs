
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::Route;

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: String, 
    pub home_name: String,
    pub away_name: String,
    pub current_clock: String, 
    pub current_period: String, 
    pub home_score: String,
    pub home_record: String,
    pub home_rank: String,
    pub away_score: String,
    pub away_record: String,
    pub away_rank: String,
    pub start_time: String, 
}

#[function_component]
pub fn GameCard(props: &GameProps) -> Html {

    let game_id = props.id.rsplit_once("/").unwrap().1;
    let game_clock = if props.current_clock.is_empty() {
        html!{
            <div class="scoreboard-gameclock">
              <p>{props.start_time.clone()}</p>
            </div>
        }
    } else {
        if props.current_period == "FINAL" {
            html! {
              <div class="scoreboard-gameclock">
                <p>{props.current_period.clone()}</p>
              </div>
            }
        } else if props.current_clock == ":00" {
            html! {
                <div class="scoreboard-gameclock">
                  <p>{"HALFTIME"}</p>
                </div>
            }
        } else {
            html!{
                <div class="scoreboard-gameclock">
                  <p>{props.current_period.clone()}</p>
                  <p>{props.current_clock.clone()}</p>
                </div>
            }
        }
    };

    html! {
        <Link<Route> classes="link" to={Route::Game { id: game_id.to_string().clone() }}>
            <div class="scoreboard-card" id={props.id.clone()}>
                <div class="scoreboard-team-container home">
                    <h4>{props.home_score.clone()}</h4>
                    <div class="scoreboard-team-details">
                      <h5>{props.home_name.clone()}</h5>
                      <p>{props.home_record.clone()}</p>
                    </div>
                    
                </div>
                <div class="scoreboard-team-container away">
                    <h4>{props.away_score.clone()}</h4>
                    <div class="scoreboard-team-details">
                      <h5>{props.away_name.clone()}</h5>
                      <p>{props.away_record.clone()}</p>
                    </div>
                </div>
                {game_clock.clone()}
            </div>
        </Link<Route>>
    }
}
