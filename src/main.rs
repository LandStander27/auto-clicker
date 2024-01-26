#![allow(non_upper_case_globals)]
#![cfg_attr(not(debug_assertions), cfg_attr(target_os = "windows", windows_subsystem = "windows"))]

#[cfg(target_os = "windows")]
use windows::Win32::System::Console::{AllocConsole, FreeConsole, GetConsoleWindow};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};

use std::process::exit;

use device_query::DeviceQuery;
use eframe::{egui, emath::Align};
use colored::Colorize;
use inputbot::{MouseButton::LeftButton, MouseCursor};

mod border;

#[track_caller]
fn print_error<S: std::fmt::Display>(e: S) {
	eprintln!("Line {}: {}", std::panic::Location::caller().line(), format!("{}", e).red());
}

#[derive(PartialEq)]
enum RadioEnum {
	Follow,
	Position,
}

struct App {
	is_clicking: bool,
	last_click: std::time::Instant,
	delay_ms_str: String,
	delay_ms: u128,
	console_visible: bool,
	follow_mouse: RadioEnum,
	position: Vec<i32>,
	waiting_for_click: bool,
	debug_mode: bool,
	position_set_time: Option<std::time::Instant>,
	click_keybind: device_query::Keycode,
	settings_window: bool,
	setting_click_keybind: bool,
	last_frame: std::time::Instant,
	click_hotkey_pressed: bool,
}

impl Default for App {
	fn default() -> Self {
		Self {
			is_clicking: false,
			last_click: std::time::Instant::now(),
			delay_ms_str: "0".to_string(),
			delay_ms: 0,
			console_visible: false,
			follow_mouse: RadioEnum::Follow,
			position: vec![0, 0],
			waiting_for_click: false,
			debug_mode: false,
			position_set_time: None,
			click_keybind: device_query::Keycode::F6,
			settings_window: false,
			setting_click_keybind: false,
			last_frame: std::time::Instant::now(),
			click_hotkey_pressed: false,
			
		}
	}
}

impl App {
	fn click(&self) {
		// simulate(&EventType::ButtonPress(Button::Left)).unwrap_or_else(|e| {
		// 	print_error(e);
		// 	exit(1);
		// });
		// simulate(&EventType::ButtonRelease(Button::Left)).unwrap_or_else(|e| {
		// 	print_error(e);
		// 	exit(1);
		// });
		LeftButton.press();
		LeftButton.release();
	}

	fn click_pos<T: Into<i32> + Copy>(&self, pos: Vec<T>) {
		// simulate(&EventType::MouseMove { x: pos[0].into(), y: pos[1].into() }).unwrap_or_else(|e| {
		// 	print_error(e);
		// 	exit(1);
		// });
		MouseCursor::move_abs(pos[0].into(), pos[1].into());
		self.click();
	}
}

impl eframe::App for App {

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			// ui.horizontal(|ui| {
			// 	ui.columns(2, |c| {
			// 		c[0].horizontal(|ui| {
			// 			if ui.add(egui::TextEdit::singleline(&mut self.delay_ms_str).hint_text("0")).changed() {
			// 				self.delay_ms_str = self.delay_ms_str.chars().filter(|x| x.to_string().parse::<u128>().is_ok()).collect();
			// 				if self.delay_ms_str.len() > 0 {
			// 					self.delay_ms = self.delay_ms_str.parse::<u128>().unwrap_or_else(|e| {
			// 						print_error(e.to_string());
			// 						0
			// 					});
			// 				} else {
			// 					self.delay_ms = 0;
			// 				}
			// 			}
			// 			ui.label("ms");
			// 		});

			// 	});
			// });

			let border_left = border::custom_window_frame(ctx, frame, "Auto Clicker", |ui| {
				if self.settings_window {
					ui.set_enabled(false);
				}

				ui.vertical(|ui| {
					let rect = ui.available_rect_before_wrap();
					let mut left = rect.clone();
					left.set_right(148.3333);
					left.set_bottom(rect.top()+20.0);
					let mut right = rect.clone();
					right.set_left(148.3333);
					right.set_right(437.0);
					right.set_bottom(rect.top()+10.0);

					ui.allocate_ui_at_rect(left, |ui| {
						ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
							// ui.add_space(seperation*0.5);
							ui.label("ms");
							if ui.add(egui::TextEdit::singleline(&mut self.delay_ms_str).hint_text("0")).changed() {
								self.delay_ms_str = self.delay_ms_str.chars().filter(|x| x.to_string().parse::<u128>().is_ok()).collect();
								if self.delay_ms_str.len() > 0 {
									self.delay_ms = self.delay_ms_str.parse::<u128>().unwrap_or_else(|e| {
										print_error(e.to_string());
										0
									});
								} else {
									self.delay_ms = 0;
								}
							}
						});
					});
					
					ui.allocate_ui_at_rect(right, |ui| {
						ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
							// ui.add_space(seperation*0.5);
							if self.position_set_time.is_none() {
								if ui.add_enabled(self.follow_mouse == RadioEnum::Position, egui::Button::new("Set position")).clicked() {
									self.waiting_for_click = true;
									println!("waiting for next click");
								}
							} else if self.position_set_time.unwrap().elapsed().as_secs_f32() < 1.5 && cfg!(windows) {
								ui.add_enabled(self.follow_mouse == RadioEnum::Position, egui::Button::new("Position set!"));
							} else {
								if ui.add_enabled(self.follow_mouse == RadioEnum::Position, egui::Button::new("Set position")).clicked() {
									if cfg!(windows) {
										self.waiting_for_click = true;
										println!("waiting for next click");
									} else {
										println!("unsupported on linux");
									}
								}
							}

							ui.radio_value(&mut self.follow_mouse, RadioEnum::Position, "Position: ");
							ui.radio_value(&mut self.follow_mouse, RadioEnum::Follow, "Follow mouse");

							// ui.label("X");
							// ui.add(egui::TextEdit::singleline(&mut self.position[0]).interactive(false));
							// ui.label("Y");
							// ui.add(egui::TextEdit::singleline(&mut self.position[1]).interactive(false));
						});
					});
				});

				// ui.separator();

				ui.vertical(|ui| {
	
					let mut centered = egui::text::LayoutJob::default();
					let font_size = 16.0;
					centered.append(format!("Start ({})", self.click_keybind.to_string()).as_str(), 0.0, egui::TextFormat {
						font_id: egui::FontId::new(font_size, egui::FontFamily::Proportional),
						valign: egui::Align::Center,
						..Default::default()
					});
	
					let seperation = 24.0;
					let rect = ui.available_rect_before_wrap();
					let mut left = rect.clone();
					left.set_right(222.5);
					left.set_bottom(rect.top()+50.0);
					let mut right = rect.clone();
					right.set_left(222.5);
					right.set_right(437.0);
					right.set_bottom(rect.top()+50.0);

					ui.allocate_ui_at_rect(left, |ui| {
						ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
							ui.add_space(seperation*0.5);
							if ui.add_enabled(!self.is_clicking, egui::Button::new(centered.clone()).min_size(ui.available_size()).wrap(true)).clicked() {
								self.is_clicking = true;
								self.last_click = std::time::Instant::now();
							}
						});
					});

					centered.sections.clear();
					centered.append(format!("Stop ({})", self.click_keybind.to_string()).as_str(), 0.0, egui::TextFormat {
						font_id: egui::FontId::new(font_size, egui::FontFamily::Proportional),
						valign: egui::Align::Center,
						..Default::default()
					});

					ui.allocate_ui_at_rect(right, |ui| {
						ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
							ui.add_space(seperation*0.5);
							if ui.add_enabled(self.is_clicking, egui::Button::new(centered).min_size(ui.available_size())).clicked() {
								self.is_clicking = false;
							}
						});
					});
	
				});

				if self.debug_mode {

					let mut centered = egui::text::LayoutJob::default();
					let font_size = 16.0;
					centered.append("Toggle console (debug)", 0.0, egui::TextFormat {
						font_id: egui::FontId::new(font_size, egui::FontFamily::Proportional),
						valign: egui::Align::Center,
						..Default::default()
					});

					ui.vertical_centered(|ui| {
						if ui.add(egui::Button::new(centered).min_size(ui.available_size())).clicked() {
							toggle_console(!self.console_visible);
							self.console_visible = !self.console_visible;
						}
					});
				}

			});

			ui.allocate_ui_at_rect(border_left, |ui| {
				ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
					if self.settings_window {
						ui.set_enabled(false);
					}
					ui.add_space(10.0);
					ui.visuals_mut().button_frame = false;
					ui.push_id("settings button", |ui| {
						if ui.button("Settings").clicked() {
							self.settings_window = true;
						}
					});
				});
			});

		});

		let state = device_query::DeviceState::new();
		let keys = state.get_keys();

		if !self.settings_window {
			if keys.contains(&self.click_keybind) {
				if !self.click_hotkey_pressed {
					self.click_hotkey_pressed = true;
					self.is_clicking = !self.is_clicking;
				}
			} else {
				self.click_hotkey_pressed = false;
			}
		}

		if self.settings_window {
			egui::Window::new("Settings").collapsible(false).resizable(false).open(&mut self.settings_window).fixed_size(egui::vec2(220.0, 100.0)).show(ctx, |ui| {
				ui.columns(2, |c| {
					c[0].vertical_centered(|ui| {
						if ui.add_enabled(!self.setting_click_keybind, egui::Button::new("Set hotkey").min_size(egui::vec2(100.0, 32.0))).clicked() {
							self.setting_click_keybind = true;
						}
					});
					c[1].vertical_centered(|ui| {
						if self.setting_click_keybind {
							ui.add_enabled(false, egui::Button::new("Press new key").min_size(egui::vec2(100.0, 32.0)));
						} else {
							ui.add_enabled(false, egui::Button::new(self.click_keybind.to_string()).min_size(egui::vec2(100.0, 32.0)));
						}
					});
				});
			});

			if self.setting_click_keybind {
				if let Some(o) = keys.first() {
					self.click_keybind = *o;
					self.setting_click_keybind = false;
				}
			}
		}

		if self.is_clicking && self.last_click.elapsed().as_millis() >= self.delay_ms {
			if self.follow_mouse == RadioEnum::Follow {
				self.click();
			} else {
				self.click_pos(self.position.clone());
			}
			self.last_click = std::time::Instant::now();
		}

		if self.waiting_for_click && cfg!(windows) {
			egui::Window::new("Info").collapsible(false).resizable(false).show(ctx, |ui| {
				ui.label("On the next click your cursor position will be saved!");
			});

			if LeftButton.is_pressed() {
				self.waiting_for_click = false;

				#[cfg(target_os = "windows")]
				{
					let (x, y) = MouseCursor::pos();

					self.position = vec![x, y];
					self.position_set_time = Some(std::time::Instant::now());
					println!("position set as {}, {}", x, y);
				}
			}
		}

		if self.follow_mouse == RadioEnum::Position {
			egui::Window::new("Info").collapsible(false).resizable(false).show(ctx, |ui| {
				ui.label("This feature is unsupported on linux.");
			});
		}

		if ctx.input(|i| i.key_pressed(egui::Key::F10)) {
			let mut size = frame.info().window_info.size;
			if self.debug_mode {
				self.debug_mode = false;
				size[1] -= 40.0;
			} else {
				self.debug_mode = true;
				size[1] += 40.0;
			}
			frame.set_window_size(size);
		}

		if cfg!(debug_assertions) {
			println!("frame time: {}s", self.last_frame.elapsed().as_secs_f32());
			self.last_frame = std::time::Instant::now();
		}

		ctx.request_repaint();
	}

	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
		println!("exiting");
	}

}

#[cfg(target_os = "windows")]
fn toggle_console(val: bool) {
	if cfg!(debug_assertions) {
		return;
	}
	unsafe {
		let w = GetConsoleWindow();
		if val {
			ShowWindow(w, SW_SHOW);
		} else {
			ShowWindow(w, SW_HIDE);
		}
	}
}

#[cfg(not(target_os = "windows"))]
fn toggle_console(_: bool) {

}

fn main() {

	#[cfg(target_os = "windows")]
	if cfg!(windows) && cfg!(not(debug_assertions)) {
		unsafe {
			AllocConsole().unwrap_or_else(|e| {
				print_error(e.to_string());
				exit(1);
			});
		}
		println!("allocated console");
		println!("hiding console");
		toggle_console(false);
	}

	println!("init options");
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(320.0, 240.0)),
		icon_data: Some(eframe::IconData::try_from_png_bytes(include_bytes!("./../icon.png")).unwrap_or_else(|e| {
			print_error(e.to_string());
			#[cfg(target_os = "windows")]
			if cfg!(windows) && cfg!(not(debug_assertions)) {
				unsafe { FreeConsole().unwrap() };
			}
			exit(1);
		})),
		follow_system_theme: true,
		resizable: false,
		decorated: false,
		transparent: true,
		always_on_top: true,
		min_window_size: Some(egui::vec2(445.0, 115.0)),
		max_window_size: Some(egui::vec2(445.0, 115.0)),
		..Default::default()
	};

	println!("init window");
	eframe::run_native(
		"Autoclicker",
		options,
		Box::new(|_cc| {
			return Box::<App>::default();
		}),
	).unwrap_or_else(|e| {
		print_error(e);
	});

}
