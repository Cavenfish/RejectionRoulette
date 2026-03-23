use anyhow::Result;
use std::collections::BTreeMap;

use kuva::plot::{PiePlot};
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_pie;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;

pub fn stats_pie(stats: &BTreeMap<String, u32>) -> Result<String> {
    let pie = PiePlot::new()
        .with_slice("Pending", *stats.get("Pending").unwrap(), "yellow")
        .with_slice("Rejected", *stats.get("Rejected").unwrap(), "red")
        .with_slice("Ghost", *stats.get("Ghost").unwrap(), "grey")
        .with_slice("Interview", *stats.get("Interview").unwrap(), "green");


    let plots = vec![Plot::Pie(pie.clone())];
    let layout = Layout::auto_from_plots(&plots).with_width(250.0).with_height(250.0);
    let scene = render_pie(&pie, &layout).with_background(None);
    let svg = SvgBackend.render_scene(&scene);

    Ok(svg)
}

// TODO:
// - make sankey plot