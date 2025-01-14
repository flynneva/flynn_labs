use yew::prelude::*;
use yew_router::prelude::*;

use ncaa_data_rs::ncaa::sport::Get;
use ncaa_data_rs::ncaa::scoreboard::Scoreboard;
use ncaa_data_rs::ncaa::sport::BaseQueries;

use crate::cards::game::GameCard;
use crate::menus::dropdown::Dropdown;
use crate::routes::MainRoute;

use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub sport: T,
}

#[derive(PartialEq, Clone)]
pub struct SportButtonArgs {
    pub sport: String,
    pub variation: String,
    pub division: String,
}

#[derive(Clone)]
struct MenuItem {
    pub callback: Callback<SportButtonArgs>,
    pub display: String,
    pub args: SportButtonArgs,
}

#[function_component]
pub fn ScoreboardPage<T>(props: &Props<T>) -> Html
where
    T: Clone + PartialEq + Default + BaseQueries + Get + 'static,
{
    let navigator = use_navigator().unwrap();
    let sport = use_state(|| props.sport.clone());
    let scoreboard = use_state(|| Scoreboard::default());
    // extra clone here to use the sport in the closure here and below
    let sport_clone = sport.clone();
    let scoreboard_clone = scoreboard.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let scoreboard = scoreboard_clone.clone();
            scoreboard.set(sport_clone.scoreboard(None).await.expect("No scoreboard available"));
        });
    });

    let mut games: Vec<_> = Vec::new();
    
    for (index, game) in scoreboard.games.iter().enumerate() {
        let game_id = scoreboard.get_id_of_game(index);
        games.push(html! {<GameCard<T> sport={sport.deref().clone()} game={game.clone()} id={game_id} />})
    }

    let navigate_click = {
        let navigator = navigator.clone();

        Callback::from(move |args: SportButtonArgs| {
            navigator.push(&MainRoute::Scoreboard {
                sport: args.sport,
                variation: args.variation,
                division: args.division,
            })
        })
    };

    let menu_items = Vec::from([
        MenuItem {
            callback: navigate_click.clone(),
            display: "D1 Mens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "men".to_string(),
                division: "d1".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "FBS Football".to_string(),
            args: SportButtonArgs {
                sport: "football".to_string(),
                variation: "none".to_string(),
                division: "fbs".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "D1 Womens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "women".to_string(),
                division: "d1".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "D2 Mens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "mens".to_string(),
                division: "d1".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "D3 Mens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "men".to_string(),
                division: "d3".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "D2 Womens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "women".to_string(),
                division: "d2".to_string(),
            }
        },
        MenuItem {
            callback: navigate_click.clone(),
            display: "D3 Womens Basketball".to_string(),
            args: SportButtonArgs {
                sport: "basketball".to_string(),
                variation: "women".to_string(),
                division: "d3".to_string(),
            }
        },
    ]);

    let mut dropdown_buttons: Vec<Html> = Vec::new();
    let _menu_clone = menu_items.clone();
    for item in menu_items.iter() {
        let item = item.clone();
        let new_button = html! {
            <button class="dropdown-item link" onclick={move |_| {
                let args = item.args.clone();
                item.callback.emit(args)}
            }>
                {item.display.clone()}
            </button>
        };

        dropdown_buttons.push(new_button);
    }

    html! {
        <div class="scoreboard">
            <Dropdown items={dropdown_buttons} />
            <div class="scoreboard-games">
                {games}
            </div>
            <h4 class="scoreboard-sport-name">{
                format!(
                    "{} {} games",
                    sport.deref().division(),
                    sport.deref().sport(),
                )
            }</h4>
        </div>
    }
}