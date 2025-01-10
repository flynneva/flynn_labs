use yew::prelude::*;
use yew_router::prelude::*;

use ncaa_data_rs::ncaa::{
    basketball::BasketballGame,
};

use crate::routes::SportsRoute;
use crate::switches::switch_sports;

#[derive(Clone, PartialEq)]
pub struct ActiveGame<G> {
    pub sport: G,
}

impl Default for ActiveGame<BasketballGame> {
    fn default() -> Self {
        Self {
            sport: BasketballGame::default(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub game_id: String,
}

#[function_component]
pub fn GamePage(GameProps {game_id}: &GameProps) -> Html {
    let active_game = use_state(Game::default());
    let active_sport = use_context::<ActiveSport<S>>.except("No active sport selected");

    
    
    html! {
        <ContextProvider<ActiveGame> context={(*active_game).clone()}>
            <Switch<SportsRoute> render={switch_sports} />
        </ContextProvider<ActiveGame>>
    }
}
