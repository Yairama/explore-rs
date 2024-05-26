use std::any::Any;
use egui_plot::*;


pub trait WrappedPlot {
    fn as_any(&self) -> &dyn Any;
    fn draw(self, plot_ui: &mut PlotUi);
}


impl WrappedPlot for Points{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.points(self);
    }

}
impl WrappedPlot for Line{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn draw(self, plot_ui: &mut PlotUi) {
        plot_ui.line(self);
    }
}
