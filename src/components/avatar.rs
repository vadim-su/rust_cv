use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::Blob;
use web_sys::HtmlVideoElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AvatarProps {
    pub initials: AttrValue,
}

#[function_component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let AvatarProps { initials } = props;

    let video_ref = use_node_ref();

    let video_url = use_state(|| {
        // Use the embedded MP4 data
        let uint8_array = Uint8Array::new_with_length(crate::AVATAR_MP4_DATA.len() as u32);
        uint8_array.copy_from(crate::AVATAR_MP4_DATA);

        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type("video/mp4");

        let blob = Blob::new_with_u8_array_sequence_and_options(
            &js_sys::Array::of1(&uint8_array),
            &blob_options,
        )
        .unwrap();

        web_sys::Url::create_object_url_with_blob(&blob).unwrap()
    });

    // Effect to ensure video plays automatically
    use_effect_with(video_ref.clone(), move |video_ref| {
        if let Some(video_element) = video_ref
            .get()
            .and_then(|el| el.dyn_into::<HtmlVideoElement>().ok())
        {
            let _ = video_element.play();
        }
        || ()
    });

    html! {
        <span class="relative flex shrink-0 overflow-hidden rounded-xl size-28">
            <video
                ref={video_ref}
                class="aspect-square object-cover"
                style="z-index: 1;"
                src={(*video_url).clone()}
                autoplay=true
                muted=true
                loop=true
                controls=false
                width="125"
                preload="auto"
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
