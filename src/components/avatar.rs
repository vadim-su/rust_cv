use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AvatarProps {
    pub url: AttrValue,
    pub initials: AttrValue,
}

#[function_component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let AvatarProps { url, initials } = props;
    html! {
        <span class="relative flex shrink-0 overflow-hidden rounded-xl size-28">
            <img
                class="aspect-square h-full w-full"
                style="z-index: 1;"
                src={url}
                alt={initials}
            />
            <div style="
                display: flex;
                justify-content: center;
                align-items: center;
                width: 100%;
                height: 100%;
                background-color: lightgray;
                font-size: 32px;
                color: white;
                position: absolute;
                z-index: 0;
            ">
                {initials}
            </div>
        </span>
    }
}
