use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum MainRoute {
  #[at("/")]
  Home,
  #[at("/sports")]
  SportsRoot,
  #[at("/sports/*")]
  Sports,
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[not_found]
  #[at("/404")]
  NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SportsRoute {
    #[at("/sports")]
    Root,
    #[at("/sports/:sport/:variation/:division/:year/:month/:day")]
    Scoreboard {sport: String, variation: String, division: String, year: i32, month: u32, day: u32},
    #[at("/sports/game/:id")]
    Game {id: String},
    #[not_found]
    #[at("/sports/404")]
    NotFound,
}
