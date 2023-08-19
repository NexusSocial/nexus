use common::{Did, Homeserver};
use direct_message::DirectMessage;
use eframe::{egui, Frame};
use egui::Context;
use std::net::UdpSocket;

fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(320.0, 240.0)),
		..Default::default()
	};
	eframe::run_native(
		"My egui App",
		options,
		Box::new(|_cc| Box::<MyApp>::default()),
	)
}

struct MyApp {
	username: String,
	did: String,
	homeserver: String,
	logged_in: bool,
	dms: Vec<DirectMessage>,
	current_message: String,
	message_to: String,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			username: "".to_string(),
			did: "".to_string(),
			homeserver: "".to_string(),
			logged_in: false,
			dms: vec![],
			current_message: "".to_string(),
			message_to: "".to_string(),
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			if !self.logged_in {
				let label = ui.label("username");
				ui.text_edit_singleline(&mut self.username)
					.labelled_by(label.id);
				let label = ui.label("did");
				ui.text_edit_singleline(&mut self.did).labelled_by(label.id);
				let label = ui.label("homeserver");
				ui.text_edit_singleline(&mut self.homeserver)
					.labelled_by(label.id);
				if ui.button("login").clicked() {
					self.logged_in = true;
				}
			} else {
				if ui.button("logout").clicked() {
					self.logged_in = false;
				}
				let client = client_direct_message_sdk::Client {
					homeserver: Homeserver(self.homeserver.clone()),
					did: Did(self.did.clone()),
					username: self.username.clone(),
				};
				if ui.button("refresh").clicked() {
					let mut dms = None;
					tokio::runtime::Builder::new_current_thread()
						.enable_all()
						.build()
						.unwrap()
						.block_on(async {
							dms.replace(client.read_dms().await.unwrap())
						});
					self.dms = dms.unwrap();
				}
				let label = ui.label("to Did");
				ui.text_edit_singleline(&mut self.message_to)
					.labelled_by(label.id);
				let label = ui.label("message");
				ui.text_edit_singleline(&mut self.current_message)
					.labelled_by(label.id);
				if ui.button("send").clicked() {
					let dm = DirectMessage {
						from: self.username.clone(),
						contents: self.current_message.clone(),
					};
					self.current_message.clear();
					tokio::runtime::Builder::new_current_thread()
						.enable_all()
						.build()
						.unwrap()
						.block_on(async {
							client
								.send_dm(dm, Did(self.message_to.clone()))
								.await
								.unwrap()
						});
					self.message_to.clear();
				}
				for dm in &self.dms {
					ui.scope(|ui| {
						ui.label(format!("from: {}", dm.from));
						ui.label(&dm.contents);
					});
				}
			}
		});
	}
}
