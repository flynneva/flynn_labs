use yew_router::prelude::*;

// List of pages in the site
pub mod home;
pub mod sports;
pub mod game;
pub mod robots;
pub mod garden;
pub mod not_found;
pub mod page;

pub const DATE_FMT: &str = "%Y/%m/%d";

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/sports/:sport/:variation/:division/:year/:month/:day")]
  Sports {sport: String, variation: String, division: String, year: i32, month: u32, day: u32},
  #[at("/sports/game/:id")]
  Game {id: String},
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub type AppLink = Link<Route>;
