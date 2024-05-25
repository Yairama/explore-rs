use std::rc::Rc;
use eframe::emath::Vec2;
use egui_plot::PlotUi;
use crate::color_palette::ColorPalette;

pub trait Chart {
    fn is_focus(&self) -> bool;

    fn get_color_pallet(&self) -> Option<&ColorPalette>;

    fn set_color_pallet(&mut self, color_palette:  Rc<ColorPalette>);

    fn plot_movement(&self, scroll: Option<Vec2>, pointer_down: bool, modifiers: egui::Modifiers, plot_ui: &mut PlotUi, is_plot_focused: bool);

    fn plot(
        &mut self,
        ui: &mut egui::Ui,
        scroll: Option<Vec2>,
        pointer_down: bool,
        modifiers: egui::Modifiers,
    );
}