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
        }
    }
}

impl Chart for  Scatter {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn draw(&self, plot_ui: &mut PlotUi){
        plot_ui.points(
            Points::new(self.points.clone())
                .filled(true)
                .radius(5.0)
                .shape(MarkerShape::Diamond)
        )
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

}


