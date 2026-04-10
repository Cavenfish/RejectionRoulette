use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalOverlayProps {
    inner: Element,
    on_close: EventHandler<()>,
}

#[component]
pub fn ModalOverlay(props: ModalOverlayProps) -> Element {
    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "modal-content",
                onclick: |evt| evt.stop_propagation(),
                {props.inner}
            }
        }
    }
}
