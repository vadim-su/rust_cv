use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AboutProps {
    pub about: AttrValue,
}

#[function_component]
pub fn About(props: &AboutProps) -> Html {
    let AboutProps { about } = props;
    html! {
        <div class="flex min-h-0 flex-col gap-y-3 print:gap-y-1">
            <h2 class="text-xl font-bold">
                {"About"}
            </h2>
            <p class="text-pretty font-mono text-sm text-foreground/80 print:text-[12px]">
                {about}
            </p>
        </div>
    }
}
