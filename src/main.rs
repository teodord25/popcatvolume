use std::path::Path;
use gtk::prelude::*;
use gtk::{ Window, WindowType, Image, Orientation, Button };

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello GTK!");
    window.set_default_size(200, 200);

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&vbox);

    let image = Image::from_file("images/0.png");
    vbox.pack_start(&image, true, true, 0);

    let button = Button::with_label("Click me!");
    vbox.pack_start(&button, false, false, 0);

    let image_clone = image.clone();
    button.connect_clicked(move |_| {
        image_clone.set_from_file(Some(Path::new("images/1.png")));
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
