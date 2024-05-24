use eframe::emath::Vec2;
use egui_plot::PlotUi;

pub trait Chart {

    fn is_focus(&self) -> bool;

    fn plot_movement(&self, scroll: Option<Vec2>, pointer_down: bool, modifiers: egui::Modifiers, plot_ui: &mut PlotUi, is_plot_focused: bool);
    fn plot(
        &mut self,
        ui: &mut egui::Ui,
        scroll: Option<Vec2>,
        pointer_down: bool,
        modifiers: egui::Modifiers,
    );

}