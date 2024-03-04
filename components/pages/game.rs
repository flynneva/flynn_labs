use yew::prelude::*;

use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::game_info::GameInfo;


#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: String,
}

#[function_component]
pub fn GamePage(props: &GameProps) -> Html {
    let this_game = use_state(|| GameInfo::default());
    let id = props.id.clone();

    {
        let this_game = this_game.clone();
        use_effect_with(props.id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match query::game_info(&id).await {
                    Ok(game_info) => {
                        this_game.set(game_info);
                    }
                    Err(_) => {
                        this_game.set(GameInfo::default());
                    }
                }
        })});
    }
    html! {
        <div>
            <div class="game-header">
                <div class="home">
                    <h3>{this_game.home.names.short.clone()}</h3>
                    <h2>{this_game.home.score.clone()}</h2>
                </div>
                <div class="away">
                    <h3>{this_game.away.names.short.clone()}</h3>
                    <h2>{this_game.away.score.clone()}</h2>
                </div>
            </div>
        </div>
    }
}
