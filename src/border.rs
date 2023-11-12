use eframe::egui;

pub fn custom_window_frame(ctx: &egui::Context, frame: &mut eframe::Frame, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {

	let panel_frame = egui::Frame {
		fill: ctx.style().visuals.window_fill(),
		rounding: 4.0.into(),
		stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
		outer_margin: 0.5.into(),
		..Default::default()
	};

	egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
		let app_rect = ui.max_rect();

		let title_bar_height = 32.0;

		let mut title_bar_rect = app_rect;
		title_bar_rect.max.y = title_bar_rect.min.y + title_bar_height;

		title_bar_ui(ui, frame, title_bar_rect, title);

		let mut content_rect = app_rect;
		content_rect.min.y = title_bar_rect.max.y;
		content_rect = content_rect.shrink(4.0);

		let mut content_ui = ui.child_ui(content_rect, *ui.layout());
		add_contents(&mut content_ui);

	});
}

fn title_bar_ui(ui: &mut egui::Ui, frame: &mut eframe::Frame, title_bar_rect: eframe::epaint::Rect, title: &str) {
	let painter = ui.painter();

	let title_bar_response = ui.interact(title_bar_rect, egui::Id::new("title_bar"), egui::Sense::click());

	// Paint the title:
	painter.text(
		title_bar_rect.center(),
		egui::Align2::CENTER_CENTER,
		title,
		egui::FontId::proportional(20.0),
		ui.style().visuals.text_color(),
	);

	// Paint the line under the title:
	painter.line_segment(
		[
			title_bar_rect.left_bottom() + egui::vec2(1.0, 0.0),
			title_bar_rect.right_bottom() + egui::vec2(-1.0, 0.0),
		],
		ui.visuals().widgets.noninteractive.bg_stroke,
	);

	if title_bar_response.double_clicked() {
		frame.set_maximized(!frame.info().window_info.maximized);
	} else if title_bar_response.is_pointer_button_down_on() {
		frame.drag_window();
	}

	ui.allocate_ui_at_rect(title_bar_rect, |ui| {
		ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
			ui.spacing_mut().item_spacing.x = 0.0;
			ui.visuals_mut().button_frame = false;
			ui.add_space(8.0);
			close_minimize(ui, frame);
		});
	});
}

fn close_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
	let button_height = 12.0;

	if ui.add(egui::Button::new(egui::RichText::new("🗙").size(button_height))).on_hover_text("Close").clicked() {
		frame.close();
	}
	ui.add_space(10.0);

	// if frame.info().window_info.maximized {
	// 	if ui.add(egui::Button::new(egui::RichText::new("🗗").size(button_height))).on_hover_text("Restore window").clicked() {
	// 		frame.set_maximized(false);
	// 	}
	// } else {
	// 	if ui.add(egui::Button::new(egui::RichText::new("🗗").size(button_height))).on_hover_text("Maximize window").clicked() {
	// 		frame.set_maximized(true);
	// 	}
	// }
	// ui.add_space(10.0);

	if ui.add(egui::Button::new(egui::RichText::new("🗕").size(button_height))).on_hover_text("Minimize").clicked() {
		frame.set_minimized(true);
	}
	ui.add_space(10.0);
}