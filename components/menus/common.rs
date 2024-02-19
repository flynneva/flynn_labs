use yew::prelude::*;


#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Item {
    pub display_name: String,
    pub html: Html,
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub items: Vec<Item>,
}
