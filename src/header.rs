use yew::prelude::*;

use crate::components::Avatar;
use crate::components::Button;
use crate::components::Icon;
use crate::data::CVData;
use crate::data::Contact;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub cv_data: CVData,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps { cv_data } = props;
    html! {
        <header class="flex items-center justify-between">
            <div class="flex-1 space-y-1.5">
                <h1 class="text-2xl font-bold">{cv_data.name.clone()}</h1>
                <p class="max-w-md text-pretty font-mono text-sm text-foreground/80 print:text-[12px]">
                    {cv_data.summary.clone()}
                </p>

                <p class="max-w-md items-center text-pretty font-mono text-xs text-foreground">
                    <a
                        class="inline-flex gap-x-1.5 align-baseline leading-none hover:underline"
                        href={cv_data.location_link.clone()} target="_blank"
                    >
                        <Icon icon="globe" size=0.75 />
                        {cv_data.location.clone()}
                    </a>
                </p>

                <Contacts contact={cv_data.contact.clone()} />
            </div>
            <Avatar url={cv_data.avatar_url.clone()} initials={cv_data.initials.clone()} />
        </header>
    }
}

#[derive(PartialEq, Properties)]
pub struct ContactsProps {
    contact: Contact,
}

#[function_component]
pub fn Contacts(props: &ContactsProps) -> Html {
    let ContactsProps { contact } = props;
    html! {
        <div class="flex gap-x-1 pt-1 font-mono text-sm text-foreground/80 print:hidden">
            <Button icon="mail" href={format!("mailto:{}", contact.email)}></Button>
            <Button icon="phone" href={format!("tel:{}", contact.tel)}></Button>
            { for contact.social.iter().map(|social| html! {
                <Button icon={social.icon.clone()} href={social.url.clone()}></Button>
            }) }
        </div>
    }
}
