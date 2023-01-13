mod expression;
mod points;
mod projection;
mod matrix;

use std::path::PathBuf;
use eframe::egui;
use eframe::egui::{Painter, Context, Id, LayerId, Order};
use crate::points::{Edge, EdgeAdder, Figure, PointAdder};
use crate::projection::{Projection, Screen, ToRealScreen};
use points::Point;
use eframe::epaint::{ Color32, Stroke };
use rfd::FileDialog;
use std::fs;

struct MyWindowHanlder {
    screens: [Screen; 2],
    figure: Figure<3>
}

fn drawEdge(painter: &Painter, s: &Screen, e: &Edge, f: &Figure<3>) {
    let points = f.edge_points(e);
    let p1 = s.project(points.0);
    let p2 = s.project(points.1);
    painter.line_segment([s.place(&p1),  s.place(&p2)], Stroke::new(2.4, s.color));
}

impl eframe::App for MyWindowHanlder {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                ui.heading("Vision 3D");
                let painter =
                    ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("graphics")));

                let window = &self;
                let s1 :&Screen = &window.screens[0];

                window.figure.edges
                    .iter()
                    .for_each(|e: &Edge| -> () {
                        drawEdge(&painter, s1, e, &window.figure);
                    });
                let s2 :&Screen = &window.screens[1];
                window.figure.edges.iter().for_each(|e| -> () {
                    drawEdge(&painter, s2, e, &window.figure);
                });

                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    // The top panel is often a good place for a menu bar:
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Load").clicked() {
                                let files: Option<PathBuf> = FileDialog::new()
                                    .add_filter("figure", &["fig"])
                                    .set_directory("${HOME}")
                                    .pick_file();
                                let path: PathBuf = files.unwrap();
                                let data = fs::read_to_string(path).expect("Unable to read file");
                                println!("content = {}", data);
                                points::loadFigure(data.as_str(), &mut self.figure);
                            }
                            if ui.button("Quit").clicked() {
                                _frame.close();
                            }
                        });
                    });
                });
            }
        );

        //helper.request_redraw();
    }


}

fn main() {
    const s1: Screen = Screen {
        zoom: 150,
        projectionpos: -250,
        center: Point { coords: [-30, 0] },
        color: Color32::RED,
        size: [900, 700],
    };
    const s2: Screen = Screen {
        zoom: 150,
        projectionpos: -250,
        center: Point { coords: [30, 0] },
        color: Color32::BLUE,
        size: [900, 700],
    };

    let handler: MyWindowHanlder = MyWindowHanlder {
        screens: [s1, s2],
        figure: Figure {
            points: vec![],
            edges: vec![] }
    };
   // let application = gtk::Application::new(Some("The.name.goes.here"), Default::default())
    //    .expect("Initialization failed");
    println!("Hello, world!");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vision3D App",
        options,
        Box::new(|_cc| Box::new(handler)),
    );
}
