use std::any::Any;
use egui::Color32;
use egui_plot::PlotUi;

pub trait Chart {

    fn draw(&self, plot_ui: &mut PlotUi, chart_type: Box<&dyn ChartType>);

    fn get_type(&self) -> Box<&dyn ChartType>;

    fn get_color(&self) -> Option<Color32>;

    fn set_color(&mut self, color: Color32);
}

pub trait ChartType: Any {
    fn as_any(&self) -> &dyn Any;
    fn chart_type_name(&self) -> &str;
}
