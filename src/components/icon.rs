use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub icon: AttrValue,
    pub size: f32,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let IconProps { icon, size } = props;
    html! {
        <i
            class={"text-foreground/70"}
            style={format!("width: {size}rem; height: {size}rem;")}
            data-lucide={icon}
        ></i>
    }
}
