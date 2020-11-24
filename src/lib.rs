use serde::Serialize;
use std::{error::Error, fmt::Display, str::FromStr};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, HtmlButtonElement, HtmlElement, HtmlParagraphElement,
    HtmlSelectElement, HtmlTextAreaElement,
};

#[derive(Clone, Copy)]
enum Format {
    Json,
    Yaml,
    Ron,
    Toml,
    Csv,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "json" => Format::Json,
            "yaml" => Format::Yaml,
            "ron" => Format::Ron,
            "toml" => Format::Toml,
            "csv" => Format::Csv,
            _ => return Err("".to_string()),
        })
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Json => write!(f, "json"),
            Format::Yaml => write!(f, "yaml"),
            Format::Ron => write!(f, "ron"),
            Format::Toml => write!(f, "toml"),
            Format::Csv => write!(f, "csv"),
        }
    }
}

fn document() -> Result<Document, JsValue> {
    let window = web_sys::window().ok_or_else(|| "No window")?;
    let document = window.document().ok_or_else(|| "No document")?;

    Ok(document)
}

fn get_current_input_format() -> Result<Format, JsValue> {
    let document = document()?;
    let select = document.query_selector("#input-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    let format = select.value().parse::<Format>().unwrap();

    Ok(format)
}

fn get_current_target_format() -> Result<Format, JsValue> {
    let document = document()?;
    let select = document.query_selector("#target-format")?.unwrap();
    let select = select.dyn_ref::<HtmlSelectElement>().unwrap();

    let format = select.value().parse::<Format>().unwrap();

    Ok(format)
}

fn get_current_left_value() -> Result<String, JsValue> {
    let document = document()?;
    let text_area = document.query_selector("textarea#left")?.unwrap();
    let text_area = text_area.dyn_ref::<HtmlTextAreaElement>().unwrap();

    let format = text_area.value();

    Ok(format)
}

fn set_current_right_value(new_text: String) -> Result<(), JsValue> {
    let document = document()?;

    let text_area = document.query_selector("textarea#right")?.unwrap();
    let text_area = text_area.dyn_ref::<HtmlTextAreaElement>().unwrap();

    text_area.set_value(&new_text);

    Ok(())
}

trait AsSequence {
    type T;
    fn as_seq(&self) -> &[Self::T];
}

impl AsSequence for serde_json::Value {
    type T = serde_json::Value;

    fn as_seq(&self) -> &[Self::T] {
        self.as_array().unwrap().as_slice()
    }
}

impl AsSequence for toml::Value {
    type T = toml::Value;

    fn as_seq(&self) -> &[Self::T] {
        self.as_array().unwrap().as_slice()
    }
}

impl AsSequence for ron::Value {
    type T = Self;

    fn as_seq(&self) -> &[Self::T] {
        if let ron::Value::Seq(s) = self {
            s.as_slice()
        } else {
            panic!(":(")
        }
    }
}

impl AsSequence for serde_yaml::Value {
    type T = Self;

    fn as_seq(&self) -> &[Self::T] {
        self.as_sequence().unwrap().as_slice()
    }
}

impl AsSequence for Vec<Vec<String>> {
    type T = Vec<String>;

    fn as_seq(&self) -> &[Self::T] {
        self.as_slice()
    }
}

fn update_right_serialize(new_text: String) -> Result<(), JsValue> {
    let input_format = get_current_input_format()?;

    match input_format {
        Format::Json => {
            let value: serde_json::Value =
                serde_json::from_str(&new_text).map_err(any_err_convert)?;

            update_right(value)?;
        }
        Format::Yaml => {
            let value: serde_yaml::Value =
                serde_yaml::from_str(&new_text).map_err(any_err_convert)?;

            update_right(value)?;
        }
        Format::Ron => {
            let value: ron::Value =
                ron::from_str(&new_text).map_err(any_err_convert)?;

            update_right(value)?;
        }
        Format::Toml => {
            let value: toml::Value =
                toml::from_str(&new_text).map_err(any_err_convert)?;

            update_right(value)?;
        }
        Format::Csv => {
            let mut records = vec![];

            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(new_text.as_str().as_bytes());

            for entry in reader.deserialize() {
                let result: Vec<String> = entry.map_err(any_err_convert)?;
                records.push(result);
            }

            update_right(records)?;
        }
    }

    Ok(())
}

fn update_right<T>(
    value: impl Serialize + AsSequence<T = T>,
) -> Result<(), JsValue>
where
    T: Serialize,
{
    let target_format = get_current_target_format()?;

    let new_right = match target_format {
        Format::Json => {
            serde_json::to_string_pretty(&value).map_err(any_err_convert)?
        }
        Format::Yaml => {
            serde_yaml::to_string(&value).map_err(any_err_convert)?
        }
        Format::Ron => {
            let config = ron::ser::PrettyConfig::default();
            ron::ser::to_string_pretty(&value, config)
                .map_err(any_err_convert)?
        }
        Format::Toml => {
            toml::to_string_pretty(&value).map_err(any_err_convert)?
        }
        Format::Csv => {
            let mut buffer = vec![];
            let mut writer = csv::Writer::from_writer(&mut buffer);

            for value in value.as_seq() {
                writer.serialize(value).map_err(any_err_convert)?;
            }

            writer.flush().map_err(any_err_convert)?;
            drop(writer);

            let s = String::from_utf8(buffer).map_err(any_err_convert)?;

            s
        }
    };

    set_current_right_value(new_right)?;

    Ok(())
}

fn any_err_convert(err: impl Error) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn update() -> Result<(), JsValue> {
    let current_value = get_current_left_value()?;

    update_right_serialize(current_value)?;

    Ok(())
}

fn update_or_display_error() {
    query_select_dyn_ref("#error-msg", |e: &HtmlParagraphElement| {
        if let Err(err) = update() {
            log::warn!("An error occured: {:?}", err);
            e.set_inner_text(&err.as_string().unwrap());
        } else {
            e.set_inner_text("");
        }
        Ok(())
    })
    .unwrap();
}

fn flip_formats_and_update() -> Result<(), JsValue> {
    let input_format = get_current_input_format()?;
    let target_format = get_current_target_format()?;
    let rendered_text = query_select_dyn_ref::<HtmlTextAreaElement, _, _>(
        "#right",
        |text_area| Ok(text_area.value()),
    )?;

    query_select_dyn_ref::<HtmlSelectElement, _, _>(
        "#input-format",
        |element| {
            element.set_value(&target_format.to_string());

            Ok(())
        },
    )?;

    query_select_dyn_ref::<HtmlSelectElement, _, _>(
        "#target-format",
        |element| {
            element.set_value(&input_format.to_string());

            Ok(())
        },
    )?;

    query_select_dyn_ref("#left", |element: &HtmlTextAreaElement| {
        element.set_value(&rendered_text);

        Ok(())
    })?;

    update_or_display_error();

    Ok(())
}

fn query_select_dyn_ref<T, R, F>(query: &str, f: F) -> Result<R, JsValue>
where
    T: JsCast,
    F: FnOnce(&T) -> Result<R, JsValue>,
{
    let document = document()?;

    let elem = document
        .query_selector(query)?
        .ok_or_else(|| "Cannot find element")?;
    let elem = elem.dyn_ref::<T>().ok_or_else(|| "Cannot cast to type")?;

    f(elem)
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    if let Err(err) = console_log::init_with_level(log::Level::Debug) {
        log::error!("Failed to set up logging: {}", err);
    }

    update_or_display_error();

    let on_change = Closure::wrap(
        Box::new(move || update_or_display_error()) as Box<dyn FnMut()>
    );

    let flip_formats_and_update =
        Closure::wrap(Box::new(move || flip_formats_and_update().unwrap())
            as Box<dyn FnMut()>);

    query_select_dyn_ref::<HtmlElement, (), _>(
        "textarea#left",
        |html_element| {
            html_element.set_oninput(Some(on_change.as_ref().unchecked_ref()));

            Ok(())
        },
    )?;

    query_select_dyn_ref::<HtmlElement, _, _>(
        "select#input-format",
        |html_element| {
            html_element.set_oninput(Some(on_change.as_ref().unchecked_ref()));

            Ok(())
        },
    )?;

    query_select_dyn_ref::<HtmlElement, _, _>(
        "select#target-format",
        |html_element| {
            html_element.set_oninput(Some(on_change.as_ref().unchecked_ref()));

            Ok(())
        },
    )?;

    query_select_dyn_ref::<HtmlButtonElement, _, _>("button#flip", |button| {
        button.set_onclick(Some(
            flip_formats_and_update.as_ref().unchecked_ref(),
        ));
        Ok(())
    })?;

    flip_formats_and_update.forget();
    on_change.forget();

    Ok(())
}
