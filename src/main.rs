use std::cell::Cell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{glib, glib::Object, Application, ApplicationWindow, Box, Button};
use gtk::{prelude::*, Orientation};

const APP_ID: &str = "org.gtk_rs.Hello-gtk";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(application: &Application) {
    let button_increase = Button::builder()
        .label("increase")
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

    let number = Rc::new(Cell::new(0));

    let _number_copy = number.clone();
    /*
     button.connect_clicked(|_|{
        button.set_label("オオアオ");
    });
    */
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get()+1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
    move |_| {
        number.set(number.get()-1);
        button_increase.set_label(&number.get().to_string())
    }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("GTK童貞卒業")
        .child(&gtk_box)
        .build();
    window.present();
}
