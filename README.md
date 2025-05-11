# Launcher

An app launcher for [Waybar](https://github.com/Alexays/Waybar), made with Rust.

## Installation

1. Install Rust, Waybar, Make
2. Prepare environment
   1. `make install` and `make install-dev` are copying `liblauncher.so` to `~/.config/waybar/custom` folder.
      So you can either create that folder, or change it in `Makefile`.
   2. Add to your waybar config:
      ```jsonc
      "cffi/launcher": {
          "module_path": "/home/<yo_home_folder>/.config/waybar/custom/liblauncher.so",

          "spacing": 8, // spacing
          "apps": {
              // Here you can add custom apps in format:
              //  "humanreadable_name": "command that will be executed"

              "Firefox": "hyprctl dispatch exec firefox",
              "Kitty": "hyprctl dispatch exec kitty"
          },

          // Button text format
          // Placeholders: {icon} {name}
          "format": "{icon}",

          // Replacements for {icon}, based of "humanreadable_name"
          "format-icons": {
              // "humanreadable_name": "icon replacement"

              "Firefox": "",
              "Kitty": ""
          }
      }
      ```
3. Run:
   ```shell
   make install-dev  # debug + unoptimized
   make install      # release
   ```
