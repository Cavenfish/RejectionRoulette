use anyhow::Result;
use std::collections::HashMap;

use charming::{
    Chart, ImageRenderer,
    element::{Emphasis, EmphasisFocus, JsFunction, Tooltip, Trigger, TriggerOn},
    series::Sankey,
    theme::Theme,
};

#[derive(Debug, Clone)]
pub struct Stats {
    pub sankey: StatusData,
    pub resumes: HashMap<String, StatusData>,
}

#[derive(Debug, Clone)]
pub struct StatusData {
    pub ghost: u32,
    pub reject: u32,
    pub pending: u32,
    pub interview: u32,
}

impl StatusData {
    pub fn new() -> Self {
        Self {
            ghost: 0,
            reject: 0,
            pending: 0,
            interview: 0,
        }
    }

    pub fn add_one(&mut self, status: &str) {
        match status {
            "Pending" => self.pending += 1,
            "Ghost" => self.ghost += 1,
            "Rejected" => self.reject += 1,
            "Interview" => self.interview += 1,
            _ => {}
        }
    }

    pub fn total(&self) -> u32 {
        self.ghost + self.reject + self.pending + self.interview
    }
}

impl std::fmt::Display for StatusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Pending: {}", self.pending)?;
        writeln!(f, "Rejected: {}", self.reject)?;
        writeln!(f, "Ghost: {}", self.ghost)?;
        writeln!(f, "Interview: {}", self.interview)
    }
}

pub fn stats_sankey(stats: &StatusData) -> Result<String> {
    let labels: Vec<String> = vec![
        "Applications".to_string(),
        "Pending".to_string(),
        "Ghost".to_string(),
        "Interview".to_string(),
        "Rejected".to_string(),
    ];

    let sankey = Sankey::new()
        .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Item)
                .trigger_on(TriggerOn::Mousemove)
                .value_formatter(JsFunction::new_with_args(
                    "value",
                    "return value.toFixed(1);",
                )),
        )
        .data(labels)
        .links(vec![
            ("Applications", "Ghost", stats.ghost),
            ("Applications", "Rejected", stats.reject),
            ("Applications", "Pending", stats.pending),
            ("Applications", "Interview", stats.interview),
        ]);

    let chart = Chart::new().series(sankey);

    let mut renderer = ImageRenderer::new(550, 400).theme(Theme::Custom(
        "idk",
        include_str!("../../../assets/js/custom-theme.js"),
    ));
    let svg = renderer.render(&chart)?;

    Ok(svg)
}

pub fn resumes_bar_plot() -> Result<String> {
    Ok("asd".to_string())
}
