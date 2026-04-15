use gtk4::glib;
use gtk4::prelude::*;

fn on_activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    let button = gtk4::Button::with_label("Hello World!");
    button.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.close()
    ));
    window.set_child(Some(&button));
    window.present();
}

fn main() {
    let app = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
