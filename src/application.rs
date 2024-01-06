use adw;
use gtk::{gio, glib, glib::clone, prelude::*, subclass::prelude::*};

mod imp {
    use adw::subclass::application::AdwApplicationImpl;

    use crate::window::Window;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {}

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Example";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self) {
            let app = self.obj();
            let window = Window::new(app.as_ref());
            window.present();
        }

        fn startup(&self) {
            self.parent_startup();
            println!("Initing libadwaita");
            adw::init().expect("Could not init libadwaita");
        }
    }

    impl GtkApplicationImpl for Application {
        fn window_removed(&self, window: &gtk::Window) {
            self.parent_window_removed(window);
        }
    }
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
        pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        let app: Self = glib::Object::builder()
            .property("application-id", Some("io.test.example"))
            .build();
        app.set_default();
        app
    }
}
