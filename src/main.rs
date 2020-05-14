use gtk::*;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;

fn main() {
    let application = gtk::Application::new(Some("com.example.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {

        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../resources/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load the CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/example/Example/style.css");
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Load the window UI
        let builder = Builder::new_from_resource("/org/example/Example/main_window.glade");

        // Get a reference to the window
        let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get window");
        window.set_application(Some(app));

        // Show the UI
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}
