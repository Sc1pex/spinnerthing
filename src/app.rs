use eframe::egui::{self, NumExt, Ui};
use egui_plot::{Arrows, Points};

pub struct App {
    plots: [Plot; 2],
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            plots: [std::f64::consts::PI, std::f64::consts::PHI].map(Plot::new),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                self.plots[0].update(&mut cols[0]);
                self.plots[1].update(&mut cols[1]);
            });
        });

        ctx.request_repaint();
    }
}

struct Plot {
    num: f64,
    time: f64,
    time_scale: f64,

    points: Vec<[f64; 2]>,
}

impl Plot {
    fn new(num: f64) -> Self {
        Self {
            num,
            time: 0.,
            time_scale: 1.,
            points: Vec::new(),
        }
    }

    fn update(&mut self, ui: &mut Ui) {
        let last_time = self.time;
        let ratio = 5. / self.num;
        self.time +=
            ui.input(|i| i.unstable_dt).at_most(1. / 120.) as f64 * self.time_scale * ratio;
        let dt = self.time - last_time;
        let points_to_add = 20 * self.time_scale as usize;
        let dt_inc = dt / points_to_add as f64;
        for i in 0..=points_to_add {
            let time = last_time + dt_inc * i as f64;
            let p1 = [time.cos(), time.sin()];
            let p2 = [(time * self.num).cos(), (time * self.num).sin()];
            let p2 = [p1[0] + p2[0], p1[1] + p2[1]];
            self.points.push(p2);
        }

        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                self.reset();
            }

            ui.vertical(|ui| {
                let old_num = self.num;
                ui.add(
                    egui::DragValue::new(&mut self.num)
                        .speed(0.001)
                        .fixed_decimals(5)
                        .prefix("Num: "),
                );
                if old_num != self.num {
                    self.reset();
                }
                ui.add(
                    egui::DragValue::new(&mut self.time_scale)
                        .speed(0.1)
                        .clamp_range(0.0..=10.0)
                        .prefix("Time scale: "),
                )
            });
        });

        let plot = egui_plot::Plot::new(format!("Plot{}", self.num))
            .data_aspect(1.0)
            .show_grid(false)
            .show_axes(false)
            .show_x(false)
            .show_y(false);

        let p1 = [self.time.cos(), self.time.sin()];
        let p2 = [(self.time * self.num).cos(), (self.time * self.num).sin()];
        let p2 = [p1[0] + p2[0], p1[1] + p2[1]];
        self.points.push(p2);

        plot.show(ui, |plot_ui| {
            let arrow = Arrows::new(vec![[0.0, 0.0], p1], vec![p1, p2])
                .color(egui::Color32::LIGHT_GRAY)
                .tip_length(0.1);
            plot_ui.arrows(arrow);

            let points: Points = Points::new(self.points.clone())
                .color(egui::Color32::WHITE)
                .radius(0.5);
            plot_ui.points(points);
        });
    }

    fn reset(&mut self) {
        self.time = 0.;
        self.points.clear();
    }
}
