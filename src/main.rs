use std::time::Duration;
use std::env;
use gtk::prelude::*;
use gtk::{Window, WindowType, Image, Orientation};
use gtk::glib::Continue;
use std::process::Command;
use gdk::WindowTypeHint;

// TODO: should I even use these svgs?
// TODO: run in background and open upon detecting change
// TODO: floating achieved?
// TODO: add volume bar

fn main() {
    // Initialize GTK.
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window.
    let window = Window::new(WindowType::Toplevel);
    window.set_title("all_is_kitty");
    window.set_default_size(200, 200);
    window.set_type_hint(WindowTypeHint::Notification);

    // Create a vertical box container to pack the image into.
    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&vbox);

    // Create the image widget and add it to the vbox.
    let image = Image::from_file("images/0.png");
    vbox.pack_start(&image, true, true, 0);

    let window_clone = window.clone();
    gtk::glib::timeout_add_local(Duration::from_millis(1000), {
        let image = image.clone();
        move || {
            // Execute the bash command.
            let output = Command::new("bash")
                .arg("-c")
                .arg("pactl list sinks | grep '^[[:space:]]Volume:' | grep -o '[0-9]*%' | sed -n '1p'")
                .output()
                .expect("Failed to execute command");

            // Pop off % sign
            let mut outputvec = output.stdout;
            let _ = outputvec.pop();
            let _ = outputvec.pop();

            // Convert the output to a string.
            let output_str = String::from_utf8(outputvec).unwrap();

            let volume = output_str.parse::<i32>().unwrap();

            let goober: i32 = env::var("GOOBER").unwrap_or_else(|_| "0".to_string()).parse().unwrap_or(0);

             // If the volume has changed
            if volume != goober {
                // Set the new volume to GOOBER environment variable
                env::set_var("GOOBER", volume.to_string());

                if volume == 0       { image.set_from_file(Some("images/0.png")); }
                else if volume < 25  { image.set_from_file(Some("images/1.png")); }
                else if volume < 50  { image.set_from_file(Some("images/2.png")); }
                else if volume < 75  { image.set_from_file(Some("images/3.png")); }
                else if volume < 100 { image.set_from_file(Some("images/4.png")); }
                else                 { image.set_from_file(Some("images/5.png")); }

                window_clone.show_now();

                // Setup another timeout to hide the window after 2 seconds
                let window_clone_2 = window_clone.clone();
                gtk::glib::timeout_add_local(Duration::from_secs(2), move || {
                    window_clone_2.hide();
                    Continue(false)
                });
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

