use yew::prelude::*;

use super::bootstrap::{Modal, get_modal_by_id};

const MODAL_FORM_ID: &str = "modalForm";

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ModalProps {
    #[prop_or(false)]
    pub is_shown: bool,
    pub on_hide: Callback<()>,
}

pub struct ModalComponent {
    props: ModalProps,
    link: ComponentLink<Self>,
    modal: Option<Modal>,
}

pub enum ModalMessage {
    Hide,
}

impl ModalComponent {
    fn toggle(&self) {
        if self.props.is_shown {
            self.show()
        } else {
            self.hide()
        }
    }

    fn show(&self) {
        self.modal.as_ref().expect(MODAL_FORM_ID).show();
    }

    fn hide(&self) {
        self.modal.as_ref().expect(MODAL_FORM_ID).hide();
    }
}

impl Component for ModalComponent {
    type Message = ModalMessage;
    type Properties = ModalProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ModalComponent { props, link, modal: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModalMessage::Hide => {
                self.hide();
                self.props.on_hide.emit(());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.toggle();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let on_click_close = self.link.callback(|_| ModalMessage::Hide);
        html! {
            <div class="modal" id="modalForm" tabindex="-1">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">{ "Modal" }</h5>
                        </div>
                        <div class="modal-body">
                            <p>{ "Modal Message" }</p>
                        </div>
                        <div class="modal-footer">
                            <button type="button"
                                class="btn btn-primary"
                                onclick=on_click_close>{ "Close" }</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.modal = Some(get_modal_by_id(MODAL_FORM_ID));
        }
    }
}
