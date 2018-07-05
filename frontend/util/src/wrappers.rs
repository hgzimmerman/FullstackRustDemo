use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::virtual_dom::VText;
use yew::virtual_dom::VList;

pub fn empty_vdom_node<CMP>() -> Html<CMP>
    where
        CMP: Component
{
    VNode::from(VList::new())
}
pub fn html_string<CMP>(text: String) -> Html<CMP>
    where
        CMP: Component
{
    VNode::from(VText::new(text))
}
