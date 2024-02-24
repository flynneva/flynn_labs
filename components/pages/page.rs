use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Page {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct PagesListProps {
    pub pages: Vec<Page>,
}

#[function_component]
pub fn PagesList(PagesListProps {pages}: &PagesListProps) -> Html {
    pages.iter().map(|page| html! {
        <div class="card" key={page.id}>
          <a class="link" href={format!("{}", page.url)}>
            <div class="container">
              <h1>{format!("{}", page.title)}</h1>
              <p>{format!("{}", page.description)}</p>
            </div>
          </a>
        </div>
    }).collect()
}