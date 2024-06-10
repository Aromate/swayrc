use std::{io, str::FromStr};

use cmd_lib::run_fun;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mode {
    pub width: i32,
    pub height: i32,
    pub refresh: i32,
    pub picture_aspect_ratio: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Node {
    pub id: i64,
    #[serde(rename = "type")]
    pub node_type: String,
    pub orientation: String,
    pub percent: Option<f64>,
    pub urgent: bool,
    pub marks: Vec<String>,
    pub focused: bool,
    pub layout: String,
    pub border: String,
    pub current_border_width: i32,
    pub rect: Rect,
    pub deco_rect: Rect,
    pub window_rect: Rect,
    pub geometry: Rect,
    pub name: Option<String>,
    pub window: Option<i64>,
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub focus: Vec<i64>,
    pub fullscreen_mode: i32,
    pub sticky: bool,
    pub pid: Option<i32>,
    pub app_id: Option<String>,
    pub visible: Option<bool>,
    pub max_render_time: Option<i32>,
    pub shell: Option<String>,
    pub inhibit_idle: Option<bool>,
    pub idle_inhibitors: Option<IdleInhibitors>,
    pub num: Option<i32>,
    pub output: Option<String>,
    pub representation: Option<String>,
    pub primary: Option<bool>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub modes: Option<Vec<Mode>>,
    pub non_desktop: Option<bool>,
    pub active: Option<bool>,
    pub dpms: Option<bool>,
    pub power: Option<bool>,
    pub scale: Option<f64>,
    pub scale_filter: Option<String>,
    pub transform: Option<String>,
    pub adaptive_sync_status: Option<String>,
    pub current_workspace: Option<String>,
    pub current_mode: Option<Mode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdleInhibitors {
    pub user: String,
    pub application: String,
}

impl FromStr for Node {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Node {
    pub fn new() -> io::Result<Self> {
        let result = run_fun!(
            swaymsg -t get_tree -r
        );
        Self::from_str(&result.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn find_node<F>(&self, f: F) -> Option<&Node>
    where
        F: Fn(&Node) -> bool + Copy,
    {
        if f(self) {
            return Some(self);
        }
        for node in &self.nodes {
            let found = node.find_node(f);
            if found.is_some() {
                return found;
            }
        }
        for node in &self.floating_nodes {
            let found = node.find_node(f);
            if found.is_some() {
                return found;
            }
        }
        None
    }
}
