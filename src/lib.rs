#![warn(clippy::pedantic)]

use std::{
    collections::{BTreeMap, HashMap},
    process::Command,
};

use dynfmt2::{Format, SimpleCurlyFormat};
use indexmap::IndexMap;
use serde::Deserialize;
use waybar_cffi::{
    InitInfo, Module,
    gtk::{self, prelude::*},
    waybar_module,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    apps: IndexMap<String, String>,
    spacing: Option<i32>,
    format: Option<String>,
    format_icons: Option<HashMap<String, String>>,
}

struct Launcher;

impl Launcher {
    fn create_container(config: &Config) -> gtk::Box {
        // TODO: add support for vertical

        let container = gtk::Box::new(gtk::Orientation::Horizontal, config.spacing.unwrap_or(0));
        container.set_widget_name("launcher");
        container.style_context().add_class("module");
        container.set_spacing(config.spacing.unwrap_or(0));

        let format = config.format.as_ref().map_or("{icon}", |f| f.as_str());
        for (app_name, app_command) in config.apps.clone() {
            let mut args = BTreeMap::new();

            let icon = config
                .format_icons
                .as_ref()
                .and_then(|f| f.get(&app_name))
                .unwrap_or(&app_name);
            args.insert("icon", icon);
            args.insert("name", &app_name);

            let label = SimpleCurlyFormat
                .format(format, args)
                .expect("invalid format provided");

            let app_button = gtk::Button::with_label(&label);
            app_button.style_context().add_class("flat");
            app_button.connect_clicked(move |_| {
                let mut components: Vec<&str> = app_command.split(' ').collect();
                let command = components.remove(0);
                let args = components.as_slice();

                Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap_or_else(|_| panic!("{command} failed to start"));
            });

            container.add(&app_button);
        }

        container
    }
}

impl Module for Launcher {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        container.add(&Self::create_container(&config));

        Launcher
    }
}

waybar_module!(Launcher);
