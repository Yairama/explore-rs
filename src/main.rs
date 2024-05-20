use polars::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use eframe::egui;

fn main() -> eframe::Result<()> {

    std::env::set_var("POLARS_FMT_MAX_COLS", "-1");

    let file_path = Path::new("data/train.csv");

    let file = File::open(file_path).unwrap();

    let df = CsvReader::new(file)
        .infer_schema(Some(10000))
        .has_header(true)
        .finish().unwrap();

    println!("{:?}", df);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Box::<MyApp>::default()
        }),
    )

}

struct MyApp {
  name: String,
  age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("My egui Application");
          ui.horizontal(|ui| {
              let name_label = ui.label("Your name: ");
              ui.text_edit_singleline(&mut self.name)
                  .labelled_by(name_label.id);
          });
          ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
          if ui.button("Increment").clicked() {
              self.age += 1;
          }
          ui.label(format!("Hello '{}', age {}", self.name, self.age));

      });
  }
}