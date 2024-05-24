mod scatter;
mod chart;

use std::default::Default;
use polars::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use eframe::{Frame};
use crate::chart::Chart;
use crate::scatter::Scatter;

fn main() -> eframe::Result<()> {

    let df = load_dataframe().unwrap();
    let data_points = get_series(&df, "Pclass", "Age").unwrap();


    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder {
          // maximized: Some(true),
          ..Default::default()
      },
      ..Default::default()
    };
    let scatter = Scatter {
        name: "Scatter".to_string(),
        points: data_points.clone(),
        ..Default::default()
    };
    let scatter2 = Scatter {
        name: "Scatter2".to_string(),
        points: data_points,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Box::new(GeneralApp {
                charts: vec![Box::new(scatter), Box::new(scatter2)],
                ..Default::default()
            })
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
    charts: Vec<Box<dyn Chart>>,
    is_any_plot_focused: bool
}

impl Default for GeneralApp {
    fn default() -> Self {
        Self {
            charts: Vec::new(),
            is_any_plot_focused: false
        }
    }
}

impl eframe::App for GeneralApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Indicador de si el gráfico tiene el enfoque
            let mut plot_focus_states = vec![false; self.charts.len()];

            egui::ScrollArea::vertical().enable_scrolling(!self.is_any_plot_focused).show(ui, |ui| {
                self.is_any_plot_focused = false;
                for (index, ch) in self.charts.iter_mut().enumerate() { // Usamos iter_mut para referencias mutables
                    ui.vertical_centered(|ui| {
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

                        ch.plot(ui, scroll, pointer_down, modifiers);
                        plot_focus_states[index] = ch.is_focus();
                        if ch.is_focus() {
                            self.is_any_plot_focused = true;
                        }
                    });
                }
            });
        });
    }
}
