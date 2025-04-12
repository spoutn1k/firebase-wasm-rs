use wasm_bindgen::prelude::*;

#[wasm_bindgen_struct]
#[opts(module = "firebase/app", js_name = "FirebaseOptions", setter)]
#[derive(Clone, Debug)]
pub struct FirebaseOptions {
    api_key: String,
    auth_domain: String,
    database_url: String,
    project_id: String,
    storage_bucket: String,
    messaging_sender_id: String,
    app_id: String,
    measurement_id: String,
}

#[wasm_bindgen(module = "firebase/app")]
extern "C" {
    pub type FirebaseApp;

    #[wasm_bindgen(js_name = initializeApp)]
    pub fn initialize_app() -> FirebaseApp;

    #[wasm_bindgen(js_name = initializeApp)]
    pub fn initialize_app_options_name(
        options: FirebaseOptions,
        name: Option<String>,
    ) -> FirebaseApp;
}
