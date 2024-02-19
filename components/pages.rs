use yew_router::prelude::*;

// List of pages in the site
pub mod home;
pub mod sports;
pub mod sport_page;
pub mod robots;
pub mod garden;
pub mod not_found;
pub mod page;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/sports")]
  Sports,
  #[at("/sports/:sport")]
  SportPage { sport: String },
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[at("/*")]
  NotFound,
}

pub type AppLink = Link<AppRoute>;
