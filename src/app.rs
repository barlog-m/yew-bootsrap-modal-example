use yew::prelude::*;

use super::modal::ModalComponent;

pub struct App {
    link: ComponentLink<Self>,
    is_modal_shown: bool,
}

pub enum AppMessage {
    ModalShow,
    ModalHide,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, is_modal_shown: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::ModalShow => {
                self.is_modal_shown = true;
                true
            }
            AppMessage::ModalHide => {
                self.is_modal_shown = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_click_modal = self.link.callback(|_| AppMessage::ModalShow);
        html! {
            <>
            <ModalComponent is_shown={ self.is_modal_shown } on_hide=self.link.callback(|_| AppMessage::ModalHide) />
            <div class="container">
                <button
                    type="button"
                    class="btn btn-primary mx-auto d-block mt-5"
                    onclick=on_click_modal>
                    { "Modal" }
                </button>
            </div>
            </>
        }
    }
}
