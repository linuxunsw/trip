use geojson;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StyleLayer {
    #[serde(rename = "id")]
    layer_id: String,
    #[serde(rename = "ref")]
    layer_ref: String,
    #[serde(rename = "type")]
    layer_type: String,
    #[serde(rename = "source-layer")]
    source_layer: String,
    #[serde(rename = "minzoom")]
    min_zoom: f64,
    #[serde(rename = "maxzoom")]
    max_zoom: f64,
    filter: Vec<serde_json::Value>,
    paint: HashMap<String, serde_json::Value>,
    layout: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct StyleJSON {
    name: String,
    constants: HashMap<String, String>,
    layers: Vec<serde_json::Value>,
}

pub struct Styler {
    style_by_id: HashMap<String, Box<StyleLayer>>,
    style_by_layer: HashMap<String, Box<StyleLayer>>,
    name: String,
}
