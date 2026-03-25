use anyhow::Result;
use kuva::render_sankey;
use std::collections::BTreeMap;

use kuva::plot::{PieLabelPosition, PiePlot, SankeyPlot};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_pie;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

pub fn stats_pie(stats: &BTreeMap<String, u32>) -> Result<String> {
    let pie = PiePlot::new()
        .with_slice("Pending", *stats.get("Pending").unwrap(), "yellow")
        .with_slice("Rejected", *stats.get("Rejected").unwrap(), "red")
        .with_slice("Ghost", *stats.get("Ghost").unwrap(), "grey")
        .with_slice("Interview", *stats.get("Interview").unwrap(), "green")
        .with_percent()
        .with_label_position(PieLabelPosition::Outside);


    let plots = vec![Plot::Pie(pie.clone())];
    let layout = Layout::auto_from_plots(&plots);
    let mut scene = render_pie(&pie, &layout).with_background(None);
    
    scene.text_color = Some("white".to_string());
    
    let svg = SvgBackend.render_scene(&scene);

    std::fs::write("./tmp.svg", &svg)?;

    Ok(svg)
}

pub fn stats_sankey(stats: &BTreeMap<String, u32>) -> Result<String> {
    let sankey = SankeyPlot::new()
        .with_node_color("Applications", "blue")
        .with_node_color("Ghost", "grey")
        .with_node_color("Pending", "yellow")
        .with_node_color("Rejected", "red")
        .with_node_color("Interview", "green")
        .with_link_colored("Applications", "Ghost", *stats.get("Ghost").unwrap() as f64, "grey")
        .with_link_colored("Applications", "Pending", *stats.get("Pending").unwrap() as f64, "yellow")
        .with_link_colored("Applications", "Rejected", *stats.get("Rejected").unwrap() as f64, "red")
        .with_link_colored("Applications", "Interview", *stats.get("Interview").unwrap() as f64, "green")
        .with_per_link_colors();

    let plots = vec![Plot::Sankey(sankey.clone())];
    let layout = Layout::auto_from_plots(&plots);
    let mut scene = render_sankey(&sankey, &layout).with_background(None);
    scene.text_color = Some("white".to_string());
    let svg = SvgBackend.render_scene(&scene);

    std::fs::write("./tmp.svg", &svg)?;

    Ok(svg)
}

// TODO:
// - make sankey plot