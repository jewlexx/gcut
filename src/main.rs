use std::{thread, time::Duration};

use nvml_wrapper::Nvml;
use parking_lot::Mutex;
use sysinfo::{ComponentExt, CpuExt, RefreshKind, System, SystemExt};
use systemstat::Platform;
use tracing::Level;

use crate::gpustat::{dump_all_gpu_stats, get_gpu_temp};

#[derive(Debug, Copy, Clone)]
struct TempStats {
    gpu: u32,
    cpu: u32,
}

static TEMP_STATS: Mutex<TempStats> = Mutex::new(TempStats { gpu: 0, cpu: 0 });

#[macro_use]
extern crate tracing;

mod gpustat;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    thread::spawn(|| {
        let nvml = Nvml::init().unwrap();
        let sys = systemstat::System::new();

        loop {
            let cpu_temp = sys.cpu_temp().unwrap().ceil() as u32;
            let gpu_temp = get_gpu_temp(nvml.device_by_index(0).unwrap()).unwrap();

            {
                let mut stats = TEMP_STATS.lock();

                *stats = TempStats {
                    gpu: gpu_temp,
                    cpu: cpu_temp,
                };
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|_| Box::new(GpuTempApp)),
    );
}

pub struct GpuTempApp;

impl eframe::App for GpuTempApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (ctemp, gtemp) = {
            let stats = TEMP_STATS.lock();

            (stats.cpu, stats.gpu)
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("GPU Temperature");

                    ui.label(format!("GPU Temperature: {}°C", gtemp));
                });

                ui.vertical(|ui| {
                    ui.heading("CPU Temperature");

                    ui.label(format!("CPU Temperature: {}°C", gtemp));
                });
            });

            egui::warn_if_debug_build(ui);
        });
    }
}
