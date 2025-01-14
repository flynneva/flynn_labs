use yew::prelude::*;

use std::rc::Rc;

use gloo::utils::document;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub items: Vec<Html>,
}

#[function_component]
pub fn Dropdown(props: &Props) -> Html {
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |e: MouseEvent| {
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

        use_effect(
            move || {
                let body = document().body().unwrap();
                body.add_event_listener_with_callback("click", (*close).as_ref().unchecked_ref())
                    .unwrap();

                // Clean up the event listener when the component is unmounted
                let close_clone = close.clone();
                move || {
                    body.remove_event_listener_with_callback(
                        "click",
                        (*close_clone).as_ref().unchecked_ref(),
                    )
                    .unwrap();
                }
            },
        );
    }

    html! {
        <div class="dropdown">
            <button class="dropdown-title link" onclick={onclick}>{"SPORT"}</button>
            <ul class={classes!("dropdown-menu", (*is_open).clone().then(|| Some("show")))}>
                {props.items.clone()}               
           </ul>
        </div>
    }
}