use gio::prelude::ApplicationExtManual;
use glib;
use gtk;
mod application;
mod window;

use application::Application;

fn main() -> glib::ExitCode {
    gtk::init().expect("Could not initialize gtk");
    glib::set_application_name("Example");

    // Create a new application
    let app = Application::new();

    // Run the application
    app.run()
}
