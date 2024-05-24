use eframe::emath::Vec2;
use egui_plot::PlotUi;

pub trait Chart {
    fn name(&self) -> &str;
    fn draw(&self, plot_ui: &mut PlotUi);

    fn plot_movement(&self, scroll: Option<Vec2>, pointer_down: bool, modifiers: egui::Modifiers, plot_ui: &mut PlotUi, is_plot_focused: bool);

}