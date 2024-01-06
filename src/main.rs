use gio::prelude::ApplicationExtManual;
use glib;
use gtk;
mod application;
mod window;

use application::Application;

fn main() -> glib::ExitCode {
    gtk::init().unwrap();

    // Create a new application
    let app = Application::new();

    // Run the application
    app.run()
}
