use yew_router::prelude::*;

// List of pages in the site
pub mod home;
pub mod sports;
pub mod robots;
pub mod garden;
pub mod not_found;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/sports")]
  Sports,
  #[at("/robots")]
  Robots,
  #[at("/garden")]
  Garden,
  #[at("/*")]
  NotFound,
}