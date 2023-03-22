use eframe::{egui, epi};
use serde_derive::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    max_temperature: String,
    min_temperature: String,
    phrase: String

    // this how you opt-out of serialization of a member
    //#[cfg_attr(feature = "persistence", serde(skip))]
    //value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let weather = get_weather();
        let max_temperature: String;
        let min_temperature: String;

        match weather {
            Some(w) => {
                max_temperature = w.first().expect("Error").temp.max.to_string();
                min_temperature = w.first().expect("Error").temp.min.to_string();
            }
            None => {
                max_temperature = "No weather data available".to_string();
                min_temperature = "No weather data available".to_string();
            }
        }
        
        let phrase = get_phrase();
        let phrase_of_the_day: String;

        match phrase {
            Some(p) => {
                phrase_of_the_day = p.to_string();
            }
            None => {
                phrase_of_the_day = "No phrase data available".to_string();
            }            
        }

        Self {
            // Example stuff:
            max_temperature: max_temperature.to_owned(),
            min_temperature: min_temperature.to_owned(),
            phrase: phrase_of_the_day.to_owned()
            //value: 2.7,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Daily Feeling"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }         
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { max_temperature, min_temperature, phrase} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {            
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Welcome!");
            ui.separator();
            ui.label(phrase.to_owned());
            ui.separator();
            ui.label("Maximum temperature: ".to_owned() + max_temperature);
            ui.label("Minimum temperature: ".to_owned() + min_temperature);
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn get_weather() -> Option<Root> {
    let response = reqwest::blocking::get(
        format!("http://localhost:8080/temperature/41.412392/-8.520640").as_str(),
    );

    match response {
        Ok(response) => {
            let body = response.text();
            match body {
                Ok(body) => {
                    let weather: Root =
                        serde_json::from_str(body.as_str()).expect("Failed to parse weather data");
                    Some(weather)
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

fn get_phrase() -> Option<String> {
    let response = reqwest::blocking::get(
        format!("http://localhost:8080/day-phrase").as_str(),
    );

    match response {
        Ok(response) => {
            let body = response.text();
            match body {
                Ok(body) => {
                    let phrase: String = body.as_str().to_owned();
                    Some(phrase)
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

type Root = Vec<Daily>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Daily {
    temp: Temperature,
}

#[derive(Serialize, Deserialize)]
struct Temperature {
    day: f32,
    min: f32,
    max: f32,
    night: f32,
    eve: f32,
    morn: f32,
}
