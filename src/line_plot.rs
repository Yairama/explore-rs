use eframe::epaint::Stroke;
use egui::{Color32};
use egui_plot::{Line, LineStyle, PlotUi};
use crate::plot::{Plot, PlotType};


pub struct LinePlot {
    name: String,
    series: Vec<[f64; 2]>,
    color: Color32,
    width: f32,
    stroke: Stroke,
    highlight: bool,
    fill: Option<f32>,
    style: LineStyle,
    chart_type: PlotType
}

impl Default for LinePlot {
    fn default() -> Self {
        Self {
            name: "Line".to_string(),
            series: vec![],
            color: Color32::TRANSPARENT,
            width: 1.5,
            stroke: Stroke::new(1.5, Color32::TRANSPARENT),
            highlight: false,
            fill: None,
            style: LineStyle::Solid,
            chart_type: PlotType::LinePlot
        }
    }
}

#[allow(dead_code)]
impl LinePlot{
    pub fn new(name: &str, series: Vec<[f64; 2]>) -> Self {
        Self {
            name: name.to_string(),
            series,
            chart_type: PlotType::LinePlot,
            ..Default::default()
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self.stroke.width = width;
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self.stroke.color = self.color;
        self
    }

    pub fn style(mut self, style: LineStyle) -> Self {
        self.style = style;
        self
    }

    pub fn fill(mut self, fill: f32) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.width = stroke.width;
        self.color = stroke.color;
        self.stroke = stroke;
        self
    }

}

impl Plot for LinePlot {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let mut stroke = self.stroke;
        stroke.color = self.color;
        stroke.width = self.width;
        let mut line = Line::new(self.series.clone())
            .name(&self.name)
            .stroke(stroke)
            .highlight(self.highlight)
            .style(self.style);

        if let Some(fill) = self.fill {
            line = line.fill(fill);
        }

        plot_ui.line(line);
    }

    fn get_type(&self) -> &PlotType {
        &self.chart_type
    }

    fn get_color(&self) -> Color32 {
        self.color
    }

    fn set_color(&mut self, color: Color32) {
        self.color = color;
    }
}



