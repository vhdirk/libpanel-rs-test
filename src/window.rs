use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use adw::prelude::AdwApplicationWindowExt;
    use gtk::prelude::{ButtonExt, WidgetExt, ToggleButtonExt};

    use super::*;

    #[derive(Debug, Default)]
    pub struct Window {}

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "ExampleWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            let view = adw::ToolbarView::new();

            let button = gtk::ToggleButton::builder()
                .label("click me!")
                .margin_top(100)
                .margin_bottom(100)
                .margin_start(100)
                .margin_end(100)
                .build();


            button.connect_clicked(|btn| {

                if btn.is_active() {
                    btn.set_label("Set light");
                    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
                } else {
                    btn.set_label("Set dark");
                    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceLight);

                };
            });


            view.set_content(Some(&button));

            let header = adw::HeaderBar::new();
            view.add_top_bar(&header);

            self.obj().set_content(Some(&view));
            self.obj().set_height_request(300);
            self.obj().set_width_request(300);
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl AdwWindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
        pub struct Window(ObjectSubclass<imp::Window>)
                @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window, adw::ApplicationWindow, //, panel::Workspace,
                @implements gtk::Accessible, gio::ActionGroup, gio::ActionMap, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let this: Self = glib::Object::builder()
            .property("application", application)
            .build();
        this
    }
}
