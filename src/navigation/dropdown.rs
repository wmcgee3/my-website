use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[derive(Properties, PartialEq)]
pub(crate) struct MenuProps {
    pub children: Children,
    pub label: &'static str,
    pub onclick: Callback<MouseEvent>,
    pub path: &'static str,
    pub show: bool,
}

#[function_component]
pub(crate) fn DropdownMenu(props: &MenuProps) -> Html {
    let location = use_location().unwrap();
    let current_path = location.path();

    let onclick = props.onclick.clone();
    let onclick = move |e| onclick.emit(e);

    let element_id = format!("{}Dropdown", props.label.to_lowercase());

    let active_class = if current_path.starts_with(props.path) {
        "active"
    } else {
        ""
    };
    let aria_expanded = if props.show { "true" } else { "false" };

    html! {
        <li class="nav-item dropdown">
            <a class={classes!("nav-link", "dropdown-toggle", active_class)}
                id={element_id.clone()}
                role="button" aria-haspopup="true"
                aria-expanded={aria_expanded} {onclick}
            >
                {props.label}
            </a>
            <ul class={classes!("dropdown-menu", if props.show { "show" } else { "" })}
                aria-labelledby={element_id.clone()}
            >
                { for props.children.iter() }
            </ul>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct LinkProps {
    pub label: &'static str,
    pub onclick: Callback<MouseEvent>,
    pub to: Route,
}

#[function_component]
pub(crate) fn DropdownLink(props: &LinkProps) -> Html {
    let location = use_location().unwrap();
    let current_path = location.path();

    let onclick = props.onclick.clone();
    let onclick = move |e| onclick.emit(e);

    let active_class = if current_path == props.to.to_path() {
        "active"
    } else {
        ""
    };

    html! {
        <li {onclick}>
            <Link<Route> classes={classes!("dropdown-item", active_class)} to={props.to.clone()}>
                { props.label }
            </Link<Route>>
        </li>
    }
}
