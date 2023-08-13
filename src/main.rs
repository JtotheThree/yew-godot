use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let onclick = {
        let window = web_sys::window().unwrap();
        let event = CustomEvent::new_with_event_init_dict(
            "_on_click",
            CustomEventInit::new().detail(&JsValue::from_str("Send JSON string here?"))
        ).unwrap();

        move |_| {
            window.dispatch_event(&event).unwrap();
        }
    };

    html! {
        <>
            <h3>{ "Yew-Godot Example!" }</h3>
            <button {onclick}>{ "Click me" }</button>
            <br />
            <canvas id={"canvas"} width="800" height="600" style="border:1px solid"></canvas>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}