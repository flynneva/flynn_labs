use yew_router::prelude::*;

// List of pages in the site
pub mod home;
pub mod sports;
pub mod game;
pub mod robots;
pub mod garden;
pub mod not_found;
pub mod page;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/sports/:sport/:variation/:division")]
  Sports {sport: String, variation: String, division: String},
  #[at("/sports/game/:id")]
  Game {id: String},
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[at("/*")]
  NotFound,
}

pub type AppLink = Link<Route>;
