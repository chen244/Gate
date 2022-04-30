//use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use steinsgate::gatewidgets::{GateLabel, GateWidget};
//use gtk::subclass::*;
mod todopage;
mod videopage;
fn main() {
    let app = Application::builder()
        .application_id("org.hack.SteinGate")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hack to the Gate!!")
        .build();
    let window = Rc::new(window);
    let window_trans = window.clone();
    let notebook = gtk::Notebook::builder().build();
    let overlay = gtk::Notebook::builder().build();
    //let overlay = gtk::Overlay::builder().build();
    let videoshow = videopage::videopage(window_trans);
    notebook.append_page(
        &overlay,
        Some(
            &GateLabel {
                text: "Home",
                ..Default::default()
            }
            .prebuild()
            .build(),
        ),
    );
    notebook.append_page(
        &*videoshow,
        Some(
            &GateLabel {
                text: "Video",
                ..Default::default()
            }
            .prebuild()
            .build(),
        ),
    );
    window.set_child(Some(&notebook));
    let overlayrc = Rc::new(overlay);
    let overlayrc1 = overlayrc.clone();
    overlayrc.set_show_tabs(false);
    //let overlayrc2 = overlayrc.clone();
    overlayrc.append_page(
        &*todopage::todo_page(overlayrc1),
        Some(&GateLabel::default().prebuild().build()),
    );

    // Show the window.
    window.show();
}
