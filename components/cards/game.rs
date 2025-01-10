
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

#[derive(Properties, PartialEq)]
pub struct GameCardProps {
    pub game_index: usize,
}

#[function_component]
pub fn GameCard(props: &GameCardProps) -> Html {
//    let active_sport = use_context::<ActiveSport<S>>().expect("No active sport found");
//
//    let game_id = props.id.rsplit_once("/").unwrap().1;
//    let game_clock = if props.current_clock.is_empty() {
//        html!{
//            <div class="scoreboard-gameclock">
//              <p>{props.start_time.clone()}</p>
//            </div>
//        }
//    } else {
//        if props.current_period == "FINAL" {
//            html! {
//              <div class="scoreboard-gameclock">
//                <p>{props.current_period.clone()}</p>
//              </div>
//            }
//        } else if props.current_clock == ":00" {
//            html! {
//                <div class="scoreboard-gameclock">
//                  <p>{"HALFTIME"}</p>
//                </div>
//            }
//        } else {
//            html!{
//                <div class="scoreboard-gameclock">
//                  <p>{props.current_period.clone()}</p>
//                  <p>{props.current_clock.clone()}</p>
//                </div>
//            }
//        }
//    };

    html! { <div>{"game card"}</div>}
 //         <Link<SportsRoute> classes="link" to={SportsRoute::Game { id: game_id.to_string().clone() }}>
 //             <div class="scoreboard-card" id={props.id.clone()}>
 //                 <div class="scoreboard-team-container home">
 //                     <h4>{props.home_score.clone()}</h4>
 //                     <div class="scoreboard-team-details">
 //                       <h5>{props.home_name.clone()}</h5>
 //                       <p>{props.home_record.clone()}</p>
 //                     </div>
 //                     
 //                 </div>
 //                 <div class="scoreboard-team-container away">
 //                     <h4>{props.away_score.clone()}</h4>
 //                     <div class="scoreboard-team-details">
 //                       <h5>{props.away_name.clone()}</h5>
 //                       <p>{props.away_record.clone()}</p>
 //                     </div>
 //                 </div>
 //                 {game_clock.clone()}
 //             </div>
 //         </Link<SportsRoute>>
}
