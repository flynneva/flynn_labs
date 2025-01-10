use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::IntoPropValue;

use ncaa_data_rs::ncaa::scoreboard::Scoreboard;
use ncaa_data_rs::ncaa::sport::BaseQueries;
use ncaa_data_rs::ncaa::basketball::Basketball;
use ncaa_data_rs::ncaa::football::Football;

use crate::cards::game::GameCard;

#[function_component]
pub fn ScoreboardPage<T>() -> Html
where
    T: Clone + PartialEq + Default + BaseQueries + 'static,
{
    let navigator = use_navigator().unwrap();
    let sport = use_state(|| T::default());
    let scoreboard = use_state(|| Scoreboard::default());

    // extra clone here to use the sport in the closure here and below
    let sport_clone = sport.clone();
    let scoreboard_clone = scoreboard.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let sport = sport_clone.clone();
            let scoreboard = scoreboard_clone.clone();
            scoreboard.set(sport.scoreboard(None).await.expect("No scoreboard available"));
        });
    });

    let mut games: Vec<_> = Vec::new();
    
    for (index, game) in scoreboard.games.iter().enumerate() {
        games.push(html! {<GameCard game_index={index} />})
    }

    html! {
        <div class="scoreboard-games">
            {games}
        </div>
    }
}