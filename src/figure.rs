use std::default::Default;
use eframe::emath::Vec2;
use egui_plot::PlotUi;
use crate::plot::Plot;
use crate::color_palette::ColorPalette;

pub struct Figure {
    pub title: String,
    pub charts: Vec<Box<dyn Plot>>,
    pub color_palette: ColorPalette,
    pub lock_x: bool,
    pub lock_y: bool,
    pub ctrl_to_zoom: bool,
    pub shift_to_horizontal: bool,
    pub zoom_speed: f32,
    pub scroll_speed: f32,
    pub min_height: f32,
    pub min_width: f32,
    pub max_height: f32,
    pub max_width: f32,
    pub is_focus: bool,
    pub show_x: bool,
    pub show_y: bool,
    pub show_axes: bool,
    pub show_grid: bool
}

impl Default for Figure {
    fn default() -> Self {
        Self {
            title: "Figure".to_string(),
            charts: vec![],
            color_palette: ColorPalette::deep(),
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
            min_height: 800.0,
            min_width: 800.0,
            max_height: 800.0,
            max_width: 800.0,
            is_focus: false,
            show_x: true,
            show_y: true,
            show_axes:  true,
            show_grid: true
        }
    }
}

#[allow(dead_code)]
impl Figure {
    pub fn new(title: &str, charts: Vec<Box<dyn Plot>>) -> Self {
        let color_palette = ColorPalette::deep();
        Self {
            title: title.to_string(),
            charts,
            color_palette,
            ..Default::default()
        }
    }

    pub fn with_color_palette(mut self, color_palette: ColorPalette) -> Self {

        for (i, chart) in self.charts.iter_mut().enumerate() {
            let colors = color_palette.colors.as_slice();
            let color = colors[i % colors.len()];
            chart.set_color(color);
        }
        self.color_palette = color_palette;

        self
    }

    fn size(mut self, min_height: f32, min_width: f32, max_height: f32, max_width: f32) -> Self {
        self.min_height = min_height;
        self.min_width = min_width;
        self.max_height = max_height;
        self.max_width = max_width;

        self
    }

    pub fn is_focus(&self) -> bool {
        self.is_focus
    }

    fn set_color_pallet(&mut self, color_palette: ColorPalette) {
        self.color_palette = color_palette;
    }

    fn plot_movement(&self, scroll: Option<Vec2>, pointer_down: bool, modifiers: egui::Modifiers, plot_ui: &mut PlotUi, is_plot_focused: bool) {
        if let Some(mut scroll) = scroll {
            if is_plot_focused {
                if modifiers.ctrl == self.ctrl_to_zoom {
                    scroll = Vec2::splat(scroll.x + scroll.y);
                    let mut zoom_factor = Vec2::from([
                        (scroll.x * self.zoom_speed / 10.0).exp(),
                        (scroll.y * self.zoom_speed / 10.0).exp(),
                    ]);
                    if self.lock_x {
                        zoom_factor.x = 1.0;
                    }
                    if self.lock_y {
                        zoom_factor.y = 1.0;
                    }
                    plot_ui.zoom_bounds_around_hovered(zoom_factor);
                } else {
                    if modifiers.shift == self.shift_to_horizontal {
                        scroll = Vec2::new(scroll.y, scroll.x);
                    }
                    if self.lock_x {
                        scroll.x = 0.0;
                    }
                    if self.lock_y {
                        scroll.y = 0.0;
                    }
                    let delta_pos = self.scroll_speed * scroll;
                    plot_ui.translate_bounds(delta_pos);
                }
            }
        }
        if plot_ui.response().hovered() && pointer_down {
            let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
            if self.lock_x {
                pointer_translate.x = 0.0;
            }
            if self.lock_y {
                pointer_translate.y = 0.0;
            }
            plot_ui.translate_bounds(pointer_translate);
        };
    }

    pub fn show(&mut self,
                ui: &mut egui::Ui,
                scroll: Option<Vec2>,
                pointer_down: bool,
                modifiers: egui::Modifiers,
    ) {
        self.is_focus = false;
        let mut new_focus = false;

        egui::Frame::default()
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(self.min_width, self.min_height));
                ui.set_max_size(Vec2::new(self.max_width, self.max_height));
                egui_plot::Plot::new(self.title.to_string())
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .legend(egui_plot::Legend::default())
                    .show_x(self.show_x)
                    .show_y(self.show_y)
                    .show_axes(self.show_axes)
                    .show_grid(self.show_grid)
                    .show(ui, |plot_ui| {

                        for ch in self.charts.iter_mut() {
                            if plot_ui.response().hovered() || plot_ui.response().clicked() {
                                new_focus = true;
                                ch.draw(plot_ui);
                            } else {
                                ch.draw(plot_ui);
                            }
                        }

                        if new_focus {
                            self.plot_movement(scroll, pointer_down, modifiers, plot_ui, new_focus);
                        }
                    });
            });

        self.is_focus = new_focus;
    }
}