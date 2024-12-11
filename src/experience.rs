use yew::prelude::*;

use crate::data::Work;

#[derive(PartialEq, Properties)]
pub struct ExperienceProps {
    pub work_places: Vec<Work>,
}

#[function_component]
pub fn Experience(props: &ExperienceProps) -> Html {
    let ExperienceProps { work_places } = props;
    html! {
        <>
            <h1 class="text-xl font-bold"> {"Work Experience"}</h1>

            <div class="space-y-4 print:space-y-0">
                { for work_places.iter().map(|work| html! {
                    <ExperienceItem work={work.clone()} />
                }) }
            </div>

        </>
    }
}

#[function_component]
pub fn ExperienceItem(props: &ExperienceHeaderProps) -> Html {
    let ExperienceHeaderProps { work } = props;
    html! {
        <article class="rounded-lg bg-card text-card-foreground py-1 print:py-1 print:mb-2">
            <ExperienceHeader work={work.clone()} />
            <div class="text-pretty font-mono text-sm text-muted-foreground mt-2 text-xs print:text-[10px]">
                { &work.description }
            </div>
        </article>
    }
}

#[derive(PartialEq, Properties)]
pub struct ExperienceHeaderProps {
    pub work: Work,
}

#[function_component]
pub fn ExperienceHeader(props: &ExperienceHeaderProps) -> Html {
    let ExperienceHeaderProps { work } = props;
    let period = make_period(work.start.clone(), work.end.clone());
    html! {
        <div class="flex flex-col space-y-1.5">
            <div class="flex items-center justify-between gap-x-2 text-base">
                <h3 class="inline-flex items-center justify-center gap-x-1 font-semibold leading-none">
                    if let Some(link) = &work.link {
                        <a class="hover:underline" href={link.clone()}>
                            { &work.company }
                        </a>
                    } else {
                        { &work.company }
                    }
                    <ul class="list-none p-0 hidden gap-x-1 sm:inline-flex">
                        { for work.badges.iter().map(|badge| html! { <Badge>{ badge.clone() }</Badge> }) }
                    </ul>
                </h3>
                <div class="text-sm tabular-nums text-gray-500">
                    { period }
                </div>
            </div>
            <div class="font-mono text-sm font-semibold leading-none print:text-[12px]">
                { &work.title }
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct BadgeProps {
    children: Children,
}

#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let BadgeProps { children } = props;
    html! {
        <div class="inline-flex items-center rounded-md border px-2 py-0.5 font-semibold font-mono transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 text-nowrap border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/60 align-middle text-xs print:px-1 print:py-0.5 print:text-[8px] print:leading-tight">
            {children}
        </div>
    }
}

fn make_period(start: String, end: Option<String>) -> String {
    match end {
        Some(end) => format!("{} - {}", start, end),
        None => format!("{} - Present", start),
    }
}
