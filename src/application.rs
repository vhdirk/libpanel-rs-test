use adw::{self, prelude::AdwApplicationExt};
use gtk::{gio, glib, glib::clone, prelude::*, subclass::prelude::*};

mod imp {
    use adw::subclass::application::AdwApplicationImpl;
    use panel::{subclass::application::PanelApplicationImpl, Workbench};

    use crate::window::Window;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Application {
        pub workbench: Workbench,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Example";
        type Type = super::Application;
        type ParentType = panel::Application;
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
        }
    }

    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
    impl PanelApplicationImpl for Application {}

}

glib::wrapper! {
        pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application, panel::Application,
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
