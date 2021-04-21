use gtk::prelude::*;
use gtk::Application;
use gio::prelude::*;

use std::collections::HashMap;
use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;
use std;

mod canvas;
mod view;
mod color;
mod point;
mod figure;
mod state;
mod buttons_events;
mod handle_draw;
mod transformations;
use crate::canvas::{CairoCanvas, Canvas};
use crate::state::State;
use gdk::EventMask;

fn build_ui(app: &gtk::Application) {
    // Initialize the UI with Glade XML.
    let glade_src = include_str!("gui_lab3.glade");
    let builder = gtk::Builder::from_string(glade_src);

    // Get handles for the various controls we need to use.
    let window: gtk::Window = builder.get_object("mainWindow")
        .expect("Couldn't get mainWindow");


    // Get handles for all the buttons.
    let mut buttons: HashMap<String, gtk::SpinButton> = HashMap::new();
    for name in &["moveOx", "moveOy", "moveOz",
                  "stretchOx", "stretchOy", "stretchOz",
                  "zoom", "rotateOx", "rotateOy", "rotateOz"] {
        buttons.insert(name.to_string(), builder.get_object(name)
                       .expect(&format!("Couldn't get button {}", name)));
    }

    let mut switch: HashMap<String, gtk::Switch> = HashMap::new();
    for name in &["carcass", "hide_lines", "filling",] {
        switch.insert(name.to_string(), builder.get_object(name)
                       .expect(&format!("Couldn't get switch {}", name)));
    }

    let projection: gtk::ComboBoxText = builder.get_object("projection")
        .expect(&format!("Couldn't get projection ComboBoxText", ));

    // Create state of all buttons and other.
    let state = Rc::new(RefCell::new(State::new(&buttons, &switch)));

    let drawing_area: gtk::DrawingArea = builder.get_object("drawingArea")
        .expect("Couldn't get drawingArea");
    let drawing = Rc::new(RefCell::new(drawing_area));

    setup_canvas_area(&builder, &state, &drawing);
    crate::buttons_events::setup_buttons_events(&buttons, &state, &drawing);
    crate::buttons_events::setup_switchs_events(&switch, &state, &drawing);
    crate::buttons_events::setup_projection_events(&projection, &state, &drawing);

    window.set_application(Some(app));
    window.show_all();
}

fn setup_canvas_area(
    builder: &gtk::Builder,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {
    let draw_box: gtk::Box = builder.get_object("box").expect("Can't get boxx");
    let draw_state = Rc::clone(&state);

    drawing_area.borrow().connect_draw(move |_, cr| {
        let size: (i32, i32) = (draw_box.get_allocated_width(), draw_box.get_allocated_height());
        let mut canvas = CairoCanvas::new(&cr, size);
        canvas.set_line_width(0.002);

        let cur_draw_state = draw_state.borrow();

        crate::handle_draw::handle_draw(&mut canvas, &cur_draw_state);

        Inhibit(false)
    });

    {
        let drawing_area_cloned = Rc::clone(&drawing_area);
        let drawing = drawing_area_cloned.borrow();
        drawing.add_events(
            EventMask::BUTTON_MOTION_MASK |
            EventMask::BUTTON_PRESS_MASK |
            EventMask::BUTTON_RELEASE_MASK
        );
    }

    {
        let button_state = Rc::clone(&state);
        let drawing_area_cloned = Rc::clone(&drawing_area);
        let drawing = drawing_area_cloned.borrow();
        drawing.connect_button_press_event(move |_, event| {
            let mut state = button_state.borrow_mut();
            let (x, y) = event.get_position();
            state.mouse_x = x;
            state.mouse_y = y;
            Inhibit(false)
        });
    }

    {
        let button_state = Rc::clone(&state);
        let drawing_area_cloned = Rc::clone(&drawing_area);
        let drawing = drawing_area_cloned.borrow();
        drawing.connect_button_release_event(move |area, _| {
            let mut state = button_state.borrow_mut();

            area.queue_draw();
            Inhibit(false)
        });
    }

    {
        let button_state = Rc::clone(&state);
        let drawing_area_cloned = Rc::clone(&drawing_area);
        let drawing = drawing_area_cloned.borrow();
        drawing.connect_motion_notify_event(move |area, event| {
            let mut state = button_state.borrow_mut();
            let (x, y) = event.get_position();
            state.rotateOz -= (x - state.mouse_x) / 10.0;
            state.rotateOx += (y - state.mouse_y) / 10.0;
            state.mouse_x = x;
            state.mouse_y = y;
            println!("({}, {})", x, y);

            area.queue_draw();
            Inhibit(false)
        });
    }
}

fn main() {
    // Initializing GTK application
    let application = Application::new(
        Some("src.main"),
        gio::ApplicationFlags::NON_UNIQUE,
    ).expect("failed to initialize GTK application");

    // The activation signal is emitted on the activation occurs
    application.connect_activate(|app| build_ui(app));

    // Run the application
    application.run(&args().collect::<Vec<_>>());
}
