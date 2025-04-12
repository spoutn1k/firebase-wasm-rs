use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "firebase/database")]
extern "C" {
    pub type Query;
    #[wasm_bindgen(extends = Query)]
    pub type DatabaseReference;
    pub type Database;
    pub type DataSnapshot;
    pub type ListenOptions;

    #[wasm_bindgen(js_name = getDatabase)]
    pub fn get_database() -> Database;

    #[wasm_bindgen(js_name = ref)]
    pub fn ref_(database: Database, path: &str) -> DatabaseReference;

    #[wasm_bindgen(js_name = get, catch)]
    pub async fn get(ref_: Query) -> Result<DataSnapshot, JsValue>;

    #[wasm_bindgen(method)]
    pub fn exists(this: &DataSnapshot) -> bool;

    #[wasm_bindgen(method)]
    pub fn val(this: &DataSnapshot) -> JsValue;

    #[wasm_bindgen(js_name = onValue)]
    pub fn on_value(ref_: Query, callback: &Closure<dyn FnMut(DataSnapshot)>) -> js_sys::Function;
}
