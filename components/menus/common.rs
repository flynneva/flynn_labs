use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Item {
    pub display_name: String,
    pub html: Html,
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub items: Vec<Item>,
}
