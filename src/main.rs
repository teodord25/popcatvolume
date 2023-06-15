use std::time::Duration;
use gtk::prelude::*;
use gtk::{Window, WindowType, Image, Orientation};
use gtk::glib::Continue;
use std::process::Command;
use std::path::Path;

fn main() {
    // Initialize GTK.
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window.
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Volume Monitor");
    window.set_default_size(200, 200);

    // Create a vertical box container to pack the image into.
    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&vbox);

    // Create the image widget and add it to the vbox.
    let image = Image::from_file("images/0.png");
    vbox.pack_start(&image, true, true, 0);

    // Set up a timeout function to periodically poll the volume.
    // This function is called every 1000 milliseconds (1 second).
    gtk::glib::timeout_add_local(Duration::from_millis(1000), {
        let image = image.clone();
        move || {
            // Execute the bash command.
            let output = Command::new("bash")
                .arg("-c")
                .arg("pactl list sinks | grep '^[[:space:]]Volume:' | grep -o '[0-9]*%' | sed -n '1p'")
                .output()
                .expect("Failed to execute command");

            let volume = output.stdout[0] as i32;
            println!("{}", volume);

            // If the volume is over 50, change the image.
            if volume > 50 {
                image.set_from_file(Some("images/1.png"));
            } else {
                image.set_from_file(Some("images/0.png"));
            }

            // We return `Continue(true)` so that the timeout function is called again after the timeout period.
            // If we returned `Continue(false)`, the function would not be called again.
            Continue(true)
        }
    });

    // Set up an event handler to quit the GTK main loop when the window is closed.
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show the window and all its children.
    window.show_all();

    // Start the GTK main loop.
    gtk::main();
}

