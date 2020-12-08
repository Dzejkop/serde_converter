use crate::{any_err_convert, Format};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Document, HtmlDivElement, HtmlInputElement, HtmlSelectElement, Window,
};

pub fn window() -> Result<Window, JsValue> {
    let window = web_sys::window().ok_or_else(|| "No window")?;

    Ok(window)
}

pub fn get() -> Result<Document, JsValue> {
    let window = window()?;
    let document = window.document().ok_or_else(|| "No document")?;

    Ok(document)
}

pub fn get_csv_options() -> Result<bool, JsValue> {
    let document = get()?;
    let has_header_checkbox = document
        .query_selector("#csv-options label input#has-header")?
        .unwrap();
    let has_header_checkbox =
        has_header_checkbox.dyn_ref::<HtmlInputElement>().unwrap();

    Ok(has_header_checkbox.checked())
}

pub fn set_current_input_format(input_format: Format) -> Result<(), JsValue> {
    let document = get()?;
    let select = document.query_selector("#input-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    select.set_value(&input_format.to_string());

    Ok(())
}

pub fn set_current_target_format(input_format: Format) -> Result<(), JsValue> {
    let document = get()?;
    let select = document.query_selector("#target-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    select.set_value(&input_format.to_string());

    Ok(())
}

pub fn get_current_input_format() -> Result<Format, JsValue> {
    let document = get()?;
    let select = document.query_selector("#input-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    let format = select.value().parse::<Format>().unwrap();

    Ok(format)
}

pub fn get_current_target_format() -> Result<Format, JsValue> {
    let document = get()?;
    let select = document.query_selector("#target-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    let format = select.value().parse::<Format>().unwrap();

    Ok(format)
}

pub fn get_current_left_value() -> Result<String, JsValue> {
    let document = get()?;
    let text_area = document.query_selector("#left")?.unwrap();
    let text_area = text_area.dyn_ref::<HtmlDivElement>().unwrap();

    let format = text_area.inner_text();

    Ok(format)
}

pub fn set_current_right_value(new_text: String) -> Result<(), JsValue> {
    let document = get()?;

    let text_area = document.query_selector("#right")?.unwrap();
    let text_area = text_area.dyn_ref::<HtmlDivElement>().unwrap();

    text_area.set_inner_text(&new_text);

    Ok(())
}

pub fn set_current_left_value(new_text: String) -> Result<(), JsValue> {
    let document = get()?;

    let text_area = document.query_selector("#left")?.unwrap();
    let text_area = text_area.dyn_ref::<HtmlDivElement>().unwrap();

    text_area.set_inner_text(&new_text);

    Ok(())
}

pub fn query_select_dyn_ref<T, R, F>(query: &str, f: F) -> Result<R, JsValue>
where
    T: JsCast,
    F: FnOnce(&T) -> Result<R, JsValue>,
{
    let document = get()?;

    let elem = document
        .query_selector(query)?
        .ok_or_else(|| "Cannot find element")?;
    let elem = elem.dyn_ref::<T>().ok_or_else(|| "Cannot cast to type")?;

    f(elem)
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct WindowSearchQuery {
    pub left: Option<String>,
    pub input_format: Option<Format>,
    pub target_format: Option<Format>,
}

pub fn get_window_search_query() -> Result<WindowSearchQuery, JsValue> {
    let window = window()?;
    let s = window.location().search()?;
    if s.is_empty() {
        return Ok(Default::default());
    }
    let s = &s[1..];

    let wsq = serde_urlencoded::from_str(s).map_err(any_err_convert)?;

    Ok(wsq)
}
