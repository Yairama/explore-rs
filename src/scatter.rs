use std::any::Any;
use egui::Color32;
use egui_plot::{MarkerShape, PlotUi, Points};
use crate::chart::{Chart, ChartType};

pub enum ScatterType{
    Line,
    Scatter,
}

impl ChartType for ScatterType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn chart_type_name(&self) -> &str {
        match self {
            ScatterType::Line => "Line",
            ScatterType::Scatter => "Scatter"
        }
    }
}

pub struct Scatter {
    name: String,
    points: Vec<[f64; 2]>,
    marker: MarkerShape,
    radius: f32,
    filled: bool,
    color: Option<Color32>,
    chart_type: ScatterType
}
impl Default for Scatter {
    fn default() -> Self {
        Self {
            name: "Scatter".to_string(),
            points: vec![],
            marker: MarkerShape::Diamond,
            radius: 5.0,
            filled: true,
            color: None,
            chart_type: ScatterType::Scatter
        }
    }
}

impl Scatter {

    pub fn new(name: &str, points: Vec<[f64; 2]>, scatter_type: ScatterType) -> Self {
        Self {
            name: name.to_string(),
            points,
            chart_type: scatter_type,
            ..Default::default()
        }
    }

    fn with_marker(mut self, marker: MarkerShape) -> Self {
        self.marker = marker;
        self
    }

    fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }



}

impl Scatter{
    fn draw_scatter(&self, plot_ui: &mut PlotUi) {

        if let Some(color) = self.color {
            plot_ui.points(
                Points::new(self.points.clone())
                    .filled(self.filled)
                    .radius(self.radius)
                    .shape(self.marker)
                    .color(color)

            );
        }
    }
}

impl Chart for Scatter {
    fn draw(&self, plot_ui: &mut PlotUi, chart_type: Box<&dyn ChartType>) {

        if let Some(scatter_type) = chart_type.as_any().downcast_ref::<ScatterType>() {

            match scatter_type {
                ScatterType::Scatter => {
                    self.draw_scatter(plot_ui);
                }
                _ => {}
            }
        }
    }

    fn get_type(&self) -> Box<&dyn ChartType> {
        Box::new(&self.chart_type)
    }

    fn get_color(&self) -> Option<Color32> {
        self.color
    }

    fn set_color(&mut self, color: Color32) {
        self.color = Some(color);
    }
}