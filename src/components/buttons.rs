use material_yew::MatIconButton;
use yew::prelude::*;

pub struct IconFlipButton {
    showing_icon_a,
}

pub enum Msg {
    FlipIcon,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    // pub children: Children,
    pub title: String,
}

impl Icon for IconFlipButton {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            showing_icon_a: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipIcon => {
                self.showing_icon_a = !self.showing_icon_a;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        html! { <>
            
        </>}
    }
}