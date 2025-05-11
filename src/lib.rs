#![warn(clippy::pedantic)]

use std::{
    collections::BTreeMap,
    process::Command,
};

use dynfmt2::{Format, SimpleCurlyFormat};
use serde::Deserialize;
use waybar_cffi::{
    InitInfo, Module,
    gtk::{self, gio, prelude::*},
    waybar_module,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    apps: Vec<App>,
    spacing: Option<i32>,
    format: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct App {
    command: String,
    name: Option<String>,
    icon: Option<String>,
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
        for app in &config.apps {
            let mut args = BTreeMap::new();

            let name = app.name.as_ref().map_or_else(
                || app.command.split(' ').next().unwrap_or("???"),
                |icon| icon.as_str(),
            );
            args.insert("icon", app.icon.as_ref().map_or(name, |icon| icon.as_str()));
            args.insert("name", name);

            let label = SimpleCurlyFormat
                .format(format, args)
                .expect("invalid format provided");

            let app_button = gtk::Button::with_label(&label);
            app_button.style_context().add_class("flat");

            let command = app.command.clone();
            app_button.connect_clicked(move |_| {
                let command = command.clone();
                gio::spawn_blocking(move || {
                    let mut components: Vec<&str> = command.split(' ').collect();
                    let command = components.remove(0);
                    let args = components.as_slice();

                    _ = Command::new(command)
                        .args(args)
                        .spawn()
                        .unwrap_or_else(|_| panic!("{command} failed to start"))
                        .wait()
                        .inspect_err(|err| {
                            eprint!("Command \"{command}\" failed with error: {err}");
                        });
                });
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
