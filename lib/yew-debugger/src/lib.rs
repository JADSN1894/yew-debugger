use base64::{engine::general_purpose as b64_general_purpose, Engine as _};
use gloo::utils::format::JsValueSerdeExt;
use serde_json::json;
use std::{
    cell::RefCell,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::DedicatedWorkerGlobalScope;

// YewMessage -> ComponentMessage
// YewModel -> ComponentModel
// YewDebug -> ComponentDebug

pub trait YewMessage: Debug {}

pub trait YewModel: Debug {}

// pub trait YewDebugger<Model, Message> {
//     fn send_to_debbuger(&self,message: impl YewMessage)
//     where
//         Self: YewModel;
// }

pub trait YewDebug<Model, Message> {
    thread_local! {
        static MSG_ID: RefCell<AtomicUsize> = const {
            RefCell::new(AtomicUsize::new(0))};

    }
    fn send_to_debbuger(model: &impl YewModel, msg: &impl YewMessage) {
        let msg_id = Self::MSG_ID.with(|inner| inner.borrow_mut().fetch_add(1, Ordering::SeqCst));
        let encoded_model = b64_general_purpose::STANDARD.encode(format!("{:#?}", model));
        let event = json! {
            {
                "model": encoded_model,
                "metadata": {
                    "msg_id": msg_id,
                    "msg": format!("{:#?}",msg),
                }
            }
        };

        let api = "yew-debugger-collector";
        let message_to_debbuger = json! {
        {
            "api": api,
            "event": event,
        }

        };
        let global_scope: DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();

        let _ = global_scope
            .post_message(&JsValue::from_serde(&message_to_debbuger).unwrap_or_default());
    }
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
