use glib_macros::clone;
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, Orientation};
use gtk::{CssProvider, StyleContext};
use gtk::gdk::Display;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
  // Create a new application
  let app = Application::builder()
    .application_id("org.gtk-rs.example")
    .build();

  // Connect to "activate" signal of `app`
  app.connect_startup(|_| load_css());
  app.connect_activate(build_ui);

  // Run the application
  app.run();
}

fn load_css() {
  // Load the CSS file and add it to the provider
  let provider = CssProvider::new();
  provider.load_from_data(include_bytes!("style.css"));

  // Add the provider to the default screen
  StyleContext::add_provider_for_display(
    &Display::default().expect("Could not connect to display"),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
  );
}

fn build_ui(application: &Application) {
  // Create two buttons
  let button_increase = Button::builder()
    .label("Increase")
    .css_classes(vec![String::from("magenta")])
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();
  let button_decrease = Button::builder()
    .label("Decrease")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

  // More examples for styling with css
  button_decrease.set_widget_name("button-decrease");
  button_decrease.add_css_class("destructive-action");
  button_increase.add_css_class("suggested-action");

  // Reference-counted object with inner-mutability
  let number = Rc::new(Cell::new(0));

  // Connect callbacks
  // When a button is clicked, `number` and label of the other button will be changed
  button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
    move |_| {
        number.set(number.get() + 1);
        button_decrease.set_label(&number.get().to_string());
  }));
  button_decrease.connect_clicked(clone!(@weak button_increase =>
      move |_| {
          number.set(number.get() - 1);
          button_increase.set_label(&number.get().to_string());
  }));

  // Add buttons
  let gtk_box = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
  gtk_box.append(&button_increase);
  gtk_box.append(&button_decrease);

  // Build the application window and set the child box
  let window = ApplicationWindow::builder()
    .application(application)
    .title("My GTK App")
    .child(&gtk_box)
    .build();

  window.present();
}
