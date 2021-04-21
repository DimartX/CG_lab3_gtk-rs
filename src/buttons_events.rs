use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::view::View;
use crate::state::State;
//use glib::clone;

pub fn setup_buttons_events(
    buttons: &HashMap<String, gtk::SpinButton>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 3 move buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("moveOx").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.moveOx = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("moveOy").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.moveOy = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("moveOz").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.moveOz = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 rotate buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotateOx").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotateOx = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotateOy").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotateOy = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotateOz").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotateOz = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 stretch buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("stretchOx").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.stretchOx = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("stretchOy").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.stretchOy = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("stretchOz").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.stretchOz = spin_button.get_value();
            area.queue_draw();
        });
    }

    // zoom button
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("zoom").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.zoom = spin_button.get_value();
            area.queue_draw();
        });
    }
}


pub fn setup_switchs_events(
    switch: &HashMap<String, gtk::Switch>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 3 switch buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        switch.get("carcass").unwrap().connect_changed_active(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.carcass = button.get_state();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        switch.get("hide_lines").unwrap().connect_changed_active(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.hide_lines = button.get_state();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        switch.get("filling").unwrap().connect_changed_active(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.filling = button.get_state();
            area.queue_draw();
        });
    }
}


pub fn setup_projection_events(
    projection: &gtk::ComboBoxText,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 3 switch buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        projection.connect_changed(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.view =
                match button.get_active_text().unwrap().as_str() {
                    "Side" => View::Side,
                    "Front" => View::Front,
                    "Above" => View::Above,
                    _ => View::Isometric,
                };
            area.queue_draw();
        });
    }
}
