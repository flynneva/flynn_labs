use yew::html;
use yew::function_component;
use yew::Html;

use crate::menus::common::ItemProps;


#[function_component]
pub fn NavBar(ItemProps {items}: &ItemProps) -> Html {
    let nav_bar_html: Vec<_> = items.iter().map(|item|
        html! {item.html.clone()}
    ).collect();

    html! {
        <div class="nav-bar flex-container">
          {nav_bar_html}
        </div>
    }
}