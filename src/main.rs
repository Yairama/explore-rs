mod scatter_plot;
mod plot;
mod color_palette;
mod figure;
mod line_plot;

use std::default::Default;
use polars::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use eframe::{Frame};
use crate::plot::Plot;
use crate::figure::Figure;
use crate::line_plot::LinePlot;
use crate::scatter_plot::ScatterPlot;

fn main() -> eframe::Result<()> {

    let df = load_dataframe().unwrap();
    let data_points = get_series(&df, "Pclass", "Age").unwrap();
    let second_points = get_series(&df, "Pclass", "Fare").unwrap();


    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder {
          ..Default::default()
      },
      ..Default::default()
    };
    let scatter_1 = ScatterPlot::new("Scatter 1", data_points.clone());
    let scatter_2 = ScatterPlot::new("Scatter 2", second_points.clone());
    let line_1 = LinePlot::new(data_points.clone())
        .style(egui_plot::LineStyle::Dashed{ length: 5.0})
        .name("Line 1");
    let line_2 = LinePlot::new(second_points.clone())
        .name("Line 2");
    let scatters: Vec<Box<dyn Plot>> = vec![Box::new(scatter_1), Box::new(scatter_2)];
    let lines: Vec<Box<dyn Plot>> = vec![Box::new(line_1), Box::new(line_2)];
    let figure_1 = Figure::new("Scatters", scatters);
    let figure_2 = Figure::new("Lines", lines);

    let app = GeneralApp::new(vec![figure_1, figure_2]);
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Box::new(app)
        }),
    )?;

    Ok(())

}

fn get_series(df: &DataFrame, a_column: &str, b_column: &str) -> Result<Vec<[f64; 2]>, Box<dyn Error>>  {
    let df = df.drop_nulls(Some(&[a_column, b_column]))?;
    let a_casted = df.column(a_column)?.cast(&DataType::Float64)?;
    let b_casted = df.column(b_column)?.cast(&DataType::Float64)?;
    let a_series = a_casted.f64()?;
    let b_series = b_casted.f64()?;

    let result: Vec<[f64; 2]> = a_series
        .into_iter()
        .zip(b_series.into_iter())
        .map(|(a, b)| [a.unwrap(), b.unwrap()])
        .collect();
    Ok(result)
}

fn load_dataframe() -> Result<DataFrame, Box<dyn Error>> {

  std::env::set_var("POLARS_FMT_MAX_COLS", "-1");
  let file_path = Path::new("data/train.csv");

  let file = File::open(file_path).unwrap();

  let df = CsvReader::new(file)
      .infer_schema(Some(10000))
      .has_header(true)
      .finish().unwrap();

  println!("{:?}", df);
  Ok(df)
}

struct GeneralApp {
    figures: Vec<Figure>,
    is_any_plot_focused: bool
}

impl Default for GeneralApp {
    fn default() -> Self {
        Self {
            figures: Vec::new(),
            is_any_plot_focused: false
        }
    }
}

impl GeneralApp {
    fn new(figures: Vec<Figure>) -> Self {

        let app = Self {
            figures,
            is_any_plot_focused: false,
        };

        app
    }

}
impl eframe::App for GeneralApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Indicador de si el gráfico tiene el enfoque
            let mut plot_focus_states = vec![false; self.figures.len()];

            egui::ScrollArea::vertical().enable_scrolling(!self.is_any_plot_focused).show(ui, |ui| {
                self.is_any_plot_focused = false;

                let (scroll, pointer_down, modifiers) = ui.input(|i| {
                    let scroll = i.events.iter().find_map(|e| match e {
                        egui::Event::MouseWheel {
                            unit: _,
                            delta,
                            modifiers: _,
                        } => Some(*delta),
                        _ => None,
                    });
                    (scroll, i.pointer.primary_down(), i.modifiers)
                });

                for (index, fig) in self.figures.iter_mut().enumerate() {
                    egui::Window::new(index.to_string()).open(&mut true).show(ctx, |ui| {
                        fig.show(ui, scroll, pointer_down, modifiers);
                        plot_focus_states[index] = fig.is_focus();
                        if fig.is_focus() {
                            self.is_any_plot_focused = true;
                        }
                    });
                }
            });
        });
    }
}
