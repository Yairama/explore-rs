use egui::Color32;
use egui_plot::{MarkerShape, PlotUi, Points};
use crate::plot::{Plot, PlotType};


pub struct ScatterPlot {
    name: String,
    series: Vec<[f64; 2]>,
    shape: MarkerShape,
    color: Color32,
    filled: bool,
    radius: f32,
    highlight: bool,
    stems: Option<f32>,
    chart_type: PlotType
}
impl Default for ScatterPlot {
    fn default() -> Self {
        Self {
            name: "Scatter".to_string(),
            series: vec![],
            shape: MarkerShape::Diamond,
            radius: 5.0,
            filled: true,
            color: Color32::TRANSPARENT,
            stems: None,
            highlight: false,
            chart_type: PlotType::ScatterPlot
        }
    }
}

#[allow(dead_code)]
impl ScatterPlot {

    pub fn new(name: &str, series: Vec<[f64; 2]>) -> Self {
        Self {
            name: name.to_string(),
            series,
            chart_type: PlotType::ScatterPlot,
            ..Default::default()
        }
    }

    pub fn shape(mut self, shape: MarkerShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }

    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    pub fn stems(mut self, y_reference: impl Into<f32>) -> Self {
        self.stems = Some(y_reference.into());
        self
    }

    pub fn radius(mut self, radius: impl Into<f32>) -> Self {
        self.radius = radius.into();
        self
    }

}

impl Plot for ScatterPlot {
    fn draw(&self, plot_ui: &mut PlotUi) {
        let mut points = Points::new(self.series.clone())
            .shape(self.shape)
            .radius(self.radius)
            .filled(self.filled)
            .color(self.color)
            .name(&self.name)
            .highlight(self.highlight);

        if let Some(stems) = self.stems {
            points = points.stems(stems);
        }
        plot_ui.points(points);
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