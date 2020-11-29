use payload_string::PayloadString;
use serde::Serialize;
use std::{collections::HashMap, convert::TryFrom};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

mod convertible;
mod document;
mod format;
mod payload_string;

use self::convertible::Convertible;
use self::document::*;
use self::format::Format;

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
            let has_headers = get_csv_options()?;

            let mut reader = csv::ReaderBuilder::new()
                .has_headers(has_headers)
                .from_reader(new_text.as_str().as_bytes());

            if has_headers {
                let headers =
                    reader.headers().map_err(any_err_convert)?.clone();

                let mut records = vec![];

                for entry in reader.deserialize() {
                    let result: Vec<String> = entry.map_err(any_err_convert)?;
                    let mut map_entry = HashMap::new();
                    for (idx, header) in headers.iter().enumerate() {
                        map_entry
                            .insert(header.to_string(), result[idx].clone());
                    }
                    records.push(map_entry);
                }

                update_right(records)?;
            } else {
                let mut records = vec![];

                for entry in reader.deserialize() {
                    let result: Vec<String> = entry.map_err(any_err_convert)?;
                    records.push(result);
                }

                update_right(records)?;
            }
        }
    }

    Ok(())
}

fn update_right<T>(
    value: impl Serialize + Convertible<T = T>,
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

            let mut writer = csv::WriterBuilder::new().from_writer(&mut buffer);
            if value.is_vec_of_maps() {
                for entry in value.try_into_records().unwrap() {
                    writer.serialize(&entry).map_err(any_err_convert)?;
                }

                writer.flush().map_err(any_err_convert)?;
            } else {
                for value in value.as_seq() {
                    writer.serialize(value).map_err(any_err_convert)?;
                }

                writer.flush().map_err(any_err_convert)?;
            }

            drop(writer);

            let s = String::from_utf8(buffer).map_err(any_err_convert)?;

            s
        }
    };

    set_current_right_value(new_right)?;

    Ok(())
}

pub fn any_err_convert(err: impl Into<anyhow::Error>) -> JsValue {
    JsValue::from_str(&err.into().to_string())
}

#[wasm_bindgen(js_name = "generateShareQuery")]
pub fn generate_share_query() -> Result<JsValue, JsValue> {
    let left = PayloadString::from(get_current_left_value()?);
    let wsq = WindowSearchQuery {
        left: Some(left.to_string()),
        input_format: Some(get_current_input_format()?),
        target_format: Some(get_current_target_format()?),
    };

    let s = serde_urlencoded::to_string(&wsq).map_err(any_err_convert)?;

    Ok(s.into())
}

#[wasm_bindgen]
pub fn update() -> Result<(), JsValue> {
    query_select_dyn_ref("#csv-options", |e: &HtmlDivElement| {
        let input_format = get_current_input_format()?;

        if matches!(input_format, Format::Csv) {
            e.style().set_property("visibility", "inherit")?;
        } else {
            e.style().set_property("visibility", "hidden")?;
        }

        Ok(())
    })?;

    let current_value = get_current_left_value()?;

    update_right_serialize(current_value)?;

    Ok(())
}

#[wasm_bindgen(js_name = "tryUpdateSearchQuery")]
pub fn try_update_search_query() -> Result<(), JsValue> {
    let wsq = document::get_window_search_query()?;

    log::debug!("{:?}", &wsq);

    if let Some(left) = wsq.left {
        let payload = PayloadString::from_encoded(left);
        let converted = String::try_from(payload).map_err(any_err_convert)?;
        set_current_left_value(converted)?;
    }

    if let Some(input_format) = wsq.input_format {
        set_current_input_format(input_format)?;
    }

    if let Some(target_format) = wsq.target_format {
        set_current_target_format(target_format)?;
    }

    update()?;

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    if let Err(err) = console_log::init_with_level(log::Level::Debug) {
        log::error!("Failed to set up logging: {}", err);
    }

    log::info!("Library initialized");

    Ok(())
}
