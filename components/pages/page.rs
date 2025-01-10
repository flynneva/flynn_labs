use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;


#[derive(Clone, PartialEq)]
pub struct Page {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub route: MainRoute,
}

#[derive(Properties, PartialEq)]
pub struct PagesListProps {
    pub pages: Vec<Page>,
}

#[function_component]
pub fn PagesList(PagesListProps {pages}: &PagesListProps) -> Html {
    pages.iter().map(|page| html! {
        <div class="card" key={page.id}>
          <Link<MainRoute> classes="link" to={page.route.clone()}>
            <div class="container">
              <h1>{format!("{}", page.title)}</h1>
              <p>{format!("{}", page.description)}</p>
            </div>
          </Link<MainRoute>>
        </div>
    }).collect()
}