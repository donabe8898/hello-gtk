use gtk::prelude::*;
use gtk::{glib, Application,ApplicationWindow,Button};

const APP_ID: &str = "org.gtk_rs.Hello-gtk";

fn main() {
   let app = Application::builder().application_id(APP_ID).build();
   app.connect_activate(build_ui);
   app.run();
}

fn build_ui(app:&Application){
    let button = Button::builder()
        .label("Press")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    button.connect_clicked(|button|{
        button.set_label("オオアオ");
    });
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK童貞卒業")
        .child(&button)
        .build();
    window.present();
}