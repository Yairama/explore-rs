use eframe::emath::Vec2;
use egui_plot::{MarkerShape, PlotUi, Points};
use crate::chart::Chart;

pub struct Scatter {
    pub name: String,
    pub points: Vec<[f64;2]>,
    pub lock_x: bool,
    pub lock_y: bool,
    pub ctrl_to_zoom: bool,
    pub shift_to_horizontal: bool,
    pub zoom_speed: f32,
    pub scroll_speed: f32,
    pub height: f32,
    pub width: f32,
    pub is_focus: bool
}

impl Default for Scatter {
    fn default() -> Self {
        Self {
            name: "Scatter".to_string(),
            points: vec![],
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
            height: 800.0,
            width: 600.0,
            is_focus: false
        }
    }
}

impl Scatter {
    fn draw(&self, plot_ui: &mut PlotUi){
        plot_ui.points(
            Points::new(self.points.clone())
                .filled(true)
                .radius(5.0)
                .shape(MarkerShape::Diamond)
        )
    }
}

impl Chart for  Scatter {
    fn is_focus(&self) -> bool {
        self.is_focus
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

    fn plot(
        &mut self,
        ui: &mut egui::Ui,
        scroll: Option<Vec2>,
        pointer_down: bool,
        modifiers: egui::Modifiers,
    ) {
        self.is_focus = false;
        egui::Frame::default()
            .show(ui, |ui| {
                ui.set_min_height(800.0); // Ajusta el tamaño mínimo según tus necesidades
                egui_plot::Plot::new(self.name.to_string())
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plot_ui| {
                        if plot_ui.response().hovered() || plot_ui.response().clicked() {
                            self.is_focus = true;
                            self.plot_movement(scroll, pointer_down, modifiers, plot_ui, self.is_focus);
                            self.draw(plot_ui);
                        } else {
                            self.draw(plot_ui);
                        }
                    });
            });
    }

}


