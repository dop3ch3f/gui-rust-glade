extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("applicationwindow1").unwrap();

    window.set_application(Some(app));

    // Inputs
    let meessage_input: gtk::Entry = builder.get_object("message_input").unwrap();

    // Submit Button
    let button: gtk::Button = builder.get_object("generate_btn").unwrap();

    // Outputs
    let message_output: gtk::Label = builder.get_object("message_output").unwrap();
    let image_output: gtk::Image = builder.get_object("image_output").unwrap();
    let image_output_clone = image_output.clone();

    let is_dead_switch: gtk::Switch = builder.get_object("is_dead_switch").unwrap();

    button.connect_clicked(move |_| {
        message_output.set_text(&format!(
            "{}\n   \\\n    \\",
            meessage_input.get_text().unwrap().as_str()
        ));

        let is_dead = is_dead_switch.get_active();
        if is_dead {
            image_output_clone.set_from_file("./images/cat_dead.jpeg");
        } else {
            image_output_clone.set_from_file("./images/cat.jpeg");
        }
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.dop3ch3f.catsay-gui-glade"), Default::default())
            .expect("Failed to initialize gtk");

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
