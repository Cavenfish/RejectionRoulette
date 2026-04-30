use anyhow::Result;
use std::collections::HashMap;

use charming::{
    Chart,
    component::{Calendar, Legend, VisualMap, VisualMapType},
    datatype::DataFrame,
    element::{CoordinateSystem, Emphasis, ItemStyle, Orient, Tooltip, Trigger, TriggerOn},
    series::{Heatmap, Pie, Sankey, SankeyNode},
};

#[derive(Debug, Clone)]
pub struct Stats {
    pub sankey: StatusData,
    pub resumes: HashMap<String, i64>,
    pub dates: HashMap<String, i64>,
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

fn get_mount_code(id: &str, chart: &Chart) -> Result<String> {
    let theme_load = include_str!("../../../assets/js/custom-theme.js");
    let chart_json = chart.to_string();
    let mount_code = format!(
        r#"
            var millis = 100;
            setTimeout(function() {{
                {theme_load}
                var chart = echarts.init(document.getElementById('{id}'), 'Rejection Roulette', {{renderer: 'canvas'}});
                window.addEventListener('resize', function() {{
                    chart.resize();
                }});
                chart.setOption({chart_json});
            }}, millis)
        "#
    );

    Ok(mount_code)
}

pub fn stats_sankey(stats: &StatusData) -> Result<String> {
    let labels = vec![
        SankeyNode::new("Applications").item_style(ItemStyle::new().color("#0037ff")),
        SankeyNode::new("Pending").item_style(ItemStyle::new().color("#eeff00")),
        SankeyNode::new("Ghost").item_style(ItemStyle::new().color("#acacac")),
        SankeyNode::new("Interview").item_style(ItemStyle::new().color("#26ff00")),
        SankeyNode::new("Rejected").item_style(ItemStyle::new().color("#ff0000")),
    ];

    let chart = Chart::new().series(Sankey::new().data(labels).links(vec![
        ("Applications", "Ghost", stats.ghost),
        ("Applications", "Rejected", stats.reject),
        ("Applications", "Pending", stats.pending),
        ("Applications", "Interview", stats.interview),
    ]));

    get_mount_code("sankey", &chart)
}

pub fn activity_calendar(
    date_range: (String, String),
    counts: HashMap<String, i64>,
) -> Result<String> {
    let mut data: Vec<DataFrame> = Vec::new();

    for (key, &value) in counts.iter() {
        data.push(vec![key.clone().into(), value.into()]);
    }

    let chart = Chart::new()
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(15)
                .type_(VisualMapType::Continuous)
                .orient(Orient::Horizontal),
        )
        .calendar(
            Calendar::new()
                .top(20)
                .height("auto")
                .width("auto")
                .range(date_range)
                .item_style(ItemStyle::new().border_width(0.25))
                .orient(Orient::Horizontal),
        )
        .series(
            Heatmap::new()
                .coordinate_system(CoordinateSystem::Calendar)
                .data(data),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Item)
                .trigger_on(TriggerOn::Mousemove),
        )
        .legend(Legend::new());

    get_mount_code("calendar", &chart)
}

pub fn resume_pie_chart(counts: HashMap<String, i64>) -> Result<String> {
    let mut data = Vec::new();

    for (key, &value) in counts.iter() {
        data.push((value, key.clone()));
    }

    let chart = Chart::new()
        .series(
            Pie::new().data(data).emphasis(
                Emphasis::new().item_style(
                    ItemStyle::new()
                        .shadow_blur(10)
                        .shadow_offset_x(0)
                        .shadow_color("rgba(0, 0, 0, 0.5)"),
                ),
            ),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Item)
                .trigger_on(TriggerOn::Mousemove),
        );

    get_mount_code("pie", &chart)
}
