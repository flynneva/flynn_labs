use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum MainRoute {
  #[at("/")]
  Home,
  #[at("/sports")]
  Sports,
  #[at("/sports/:sport/:variation/:division")]
  Scoreboard {sport: String, variation: String, division: String},
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[not_found]
  #[at("/404")]
  NotFound,
}
