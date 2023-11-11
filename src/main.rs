#![allow(non_upper_case_globals)]
#![cfg_attr(not(debug_assertions), cfg_attr(target_os = "windows", windows_subsystem = "windows"))]

#[cfg(target_os = "windows")]
use windows::Win32::System::Console::{AllocConsole, FreeConsole, GetConsoleWindow};
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};

use std::process::exit;

use eframe::egui;
use rdev::{simulate, EventType, Button};
use colored::Colorize;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

#[track_caller]
fn print_error<S: std::fmt::Display>(e: S) {
	eprintln!("Line {}: {}", std::panic::Location::caller().line(), format!("{}", e).red());
}

struct App {
	is_clicking: bool,
	last_click: std::time::Instant,
	delay_ms_str: String,
	delay_ms: u128,
	console_visible: bool,
}

impl Default for App {
	fn default() -> Self {
		Self {
			is_clicking: false,
			last_click: std::time::Instant::now(),
			delay_ms_str: "0".to_string(),
			delay_ms: 0,
			console_visible: false,
		}
	}
}

impl App {
	fn click(&self) {
		simulate(&EventType::ButtonPress(Button::Left)).unwrap_or_else(|e| {
			print_error(e);
			exit(1);
		});
		simulate(&EventType::ButtonRelease(Button::Left)).unwrap_or_else(|e| {
			print_error(e);
			exit(1);
		});
	}

	fn click_pos<T: Into<f64> + Copy>(&self, pos: Vec<T>) {
		simulate(&EventType::MouseMove { x: pos[0].into(), y: pos[1].into() }).unwrap_or_else(|e| {
			print_error(e);
			exit(1);
		});
		self.click();
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal_top(|ui| {
				if ui.text_edit_singleline(&mut self.delay_ms_str).changed() {
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
				ui.label("ms");
			});

			if ui.add_enabled(!self.is_clicking, egui::Button::new("Start")).clicked() {
				self.is_clicking = true;
				self.last_click = std::time::Instant::now();
			}
			if ui.add_enabled(self.is_clicking, egui::Button::new("Stop")).clicked() {
				self.is_clicking = false;
			}
			if ui.button("Toggle console (debug)").clicked() {
				toggle_console(!self.console_visible);
				self.console_visible = !self.console_visible;
			}

			// if ui.input(|i| i.key_pressed(egui::Key::F6)) {
			// 	self.is_clicking = !self.is_clicking;
			// }

		});

		if let Ok(o) = GlobalHotKeyEvent::receiver().try_recv() {
			if o.state == global_hotkey::HotKeyState::Pressed {
				self.is_clicking = !self.is_clicking;
			}
		}

		if self.is_clicking && self.last_click.elapsed().as_millis() >= self.delay_ms {
			self.click();
			self.last_click = std::time::Instant::now();
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

	println!("register global hotkey");
	let manager = GlobalHotKeyManager::new().unwrap();
	manager.register(HotKey::new(None, Code::F6)).unwrap();

	println!("init options");
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(320.0, 240.0)),
		icon_data: Some(eframe::IconData::try_from_png_bytes(include_bytes!(".\\..\\icon.png")).unwrap_or_else(|e| {
			print_error(e.to_string());
			if cfg!(windows) && cfg!(not(debug_assertions)) {
				unsafe { FreeConsole().unwrap() };
			}
			exit(1);
		})),
		follow_system_theme: true,
		resizable: false,
		min_window_size: Some(egui::vec2(445.0, 360.0)),
		max_window_size: Some(egui::vec2(445.0, 360.0)),
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
