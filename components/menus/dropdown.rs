use std::rc::Rc;
use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use gloo::utils::document;

use crate::menus::common::ItemProps;


#[function_component(MenuDropdown)]
pub fn dropdown(ItemProps {items}: &ItemProps) -> Html {
    let is_open = use_state(|| false);
    // default to the first item in the vector
    let selected_item = use_state(|| items[0].clone());

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move | e: MouseEvent | {
            e.stop_propagation();
            is_open.set(!*is_open);
        })
    };

    {
        let is_open = is_open.clone();
        let close = Rc::new(Closure::<dyn Fn()>::new({
            move || {
                is_open.set(false);
            }
        }));

        use_effect_with((),
            move |_| {
                let body = document().body().unwrap();
                body.add_event_listener_with_callback("click", (*close).as_ref().unchecked_ref())
                  .unwrap();
                let close_clone = close.clone();
                move || {
                    body.remove_event_listener_with_callback(
                        "click",
                        (*close_clone).as_ref().unchecked_ref(),
                    )
                    .unwrap();
                }
            }
        );
    }

    let items_html: Vec<_> = items.iter().map(|item| item.html.clone()).collect();

    html! {
        <div class="dropdown">
          <button class="link" onclick={onclick}>{selected_item.display_name.to_string()}</button>
          <div class="dropdown-menu">
            {items_html}
          </div>
        </div>
    }
}