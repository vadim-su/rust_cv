use yew::prelude::*;

use crate::components::Icon;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub icon: AttrValue,
    pub href: AttrValue,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { icon, href } = props;
    html! {
        <a
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground size-8"
            href={href}
        >
            <Icon icon={icon} size={1.0} />
        </a>
    }
}
