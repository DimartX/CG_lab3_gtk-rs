use std::collections::HashMap;
use gtk::prelude::*;
use crate::view::View;

// Shared state for communication between buttons and drawingarea
pub struct State {
    pub stretchOx: f64,
    pub stretchOy: f64,
    pub stretchOz: f64,
    pub moveOx:    f64,
    pub moveOy:    f64,
    pub moveOz:    f64,
    pub rotateOx:  f64,
    pub rotateOy:  f64,
    pub rotateOz:  f64,
    pub zoom:      f64,
    pub carcass:    bool,
    pub hide_lines: bool,
    pub filling:    bool,
    pub view:      View,
    pub mouse_x:   f64,
    pub mouse_y:   f64,
}
// And i really sorry about camel case

impl State {
    pub fn new(buttons: &HashMap<String, gtk::SpinButton>,
               switch: &HashMap<String, gtk::Switch>) -> Self {
        State {
            stretchOx: buttons.get("stretchOx").unwrap().get_value(),
            stretchOy: buttons.get("stretchOy").unwrap().get_value(),
            stretchOz: buttons.get("stretchOz").unwrap().get_value(),
            moveOx:    buttons.get("moveOx")   .unwrap().get_value(),
            moveOy:    buttons.get("moveOy")   .unwrap().get_value(),
            moveOz:    buttons.get("moveOz")   .unwrap().get_value(),
            rotateOx:  buttons.get("rotateOx") .unwrap().get_value(),
            rotateOy:  buttons.get("rotateOy") .unwrap().get_value(),
            rotateOz:  buttons.get("rotateOz") .unwrap().get_value(),
            zoom:      buttons.get("zoom")     .unwrap().get_value(),
            carcass:   switch.get("carcass")   .unwrap().get_state(),
            hide_lines:switch.get("hide_lines").unwrap().get_state(),
            filling:   switch.get("filling")   .unwrap().get_state(),
            view:      View::Isometric,
            mouse_x:   0.0,
            mouse_y:   0.0,
        }
    }
}
