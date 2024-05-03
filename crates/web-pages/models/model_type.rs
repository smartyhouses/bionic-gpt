#![allow(non_snake_case)]
use daisy_rsx::*;
use db::ModelType;
use dioxus::prelude::*;

#[component]
pub fn Model(model_type: ModelType) -> Element {
    match model_type {
        ModelType::LLM => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Info,
                "Large Language Model"
            }
        ),
        ModelType::Embeddings => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Highlight,
                "Embeddings Model"
            }
        ),
    }
}
