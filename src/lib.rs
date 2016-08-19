#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct NodeColor {
    id: u64,
    color: Color,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct NodeText {
    id: u64,
    text: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct NodeRepel {
    id: u64,
    force: f64,
}

fn default_link_weight() -> f64 {
    1.0
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct NodeLink {
    relation: u64,
    from: u64,
    to: u64,
    #[serde(default = "default_link_weight")]
    weight: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    colors: Option<Vec<NodeColor>>,
    text: Option<Vec<NodeText>>,
    repels: Option<Vec<NodeRepel>>,
    links: Vec<NodeLink>,
}
