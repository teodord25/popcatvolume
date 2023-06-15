use gtk::prelude::*;
use gtk::{ Window, WindowType };

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello GTK!");
    window.set_default_size(200, 200);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
