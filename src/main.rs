mod about;
mod components;
mod data;
mod experience;
mod header;

include!(concat!(env!("OUT_DIR"), "/cv_data.rs"));

use experience::Experience;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use about::About;
use data::CVData;
use header::Header;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let cv_data: CVData = serde_json::from_str(RAW_CV_DATA).expect("file should be proper JSON");

    use_effect(|| {
        // Call the JS function when the component mounts
        create_icons();
        || ()
    });

    html! {
        <>
            <div class="container relative mx-auto scroll-my-12 overflow-auto p-4 print:p-11 md:p-16">
                <section class="mx-auto w-full max-w-2xl space-y-8 bg-white print:space-y-4">

                    <Header cv_data={cv_data.clone()} />

                    <div class="space-y-4 print:space-y-4">
                        <section class="flex min-h-0 flex-col gap-y-3 print:gap-y-1">
                            <About about={cv_data.about} />
                        </section>

                        <section class="flex min-h-0 flex-col gap-y-3 print:gap-y-1">
                            <Experience work_places={cv_data.work} />
                        </section>
                    </div>

                </section>

            </div>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = lucide, js_name = createIcons)]
    fn create_icons();
}
