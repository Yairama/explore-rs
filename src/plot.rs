use egui::Color32;
use egui_plot::{MarkerShape, PlotUi, Points};

pub trait Plot {

    fn draw(&self, plot_ui: &mut PlotUi);

    fn get_type(&self) -> &PlotType;

    fn get_color(&self) -> Color32;

    fn set_color(&mut self, color: Color32);
}

pub enum  PlotType {
    ScatterPlot,
    LinePlot,
}

pub enum TestPlot {
    ScatterPlot(egui_plot::Points),
    LinePlot(egui_plot::Line),
}

