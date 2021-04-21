use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::view::*;
use crate::state::State;
//use glib::clone;

pub fn setup_buttons_events(
    buttons: &HashMap<String, gtk::SpinButton>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 3 buttons parameters
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("parameter_a").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.parameter_a = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("parameter_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.parameter_b = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("parameter_c").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.parameter_c = spin_button.get_value();
            area.queue_draw();
        });
    }

    { // approx
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("approx").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.approx = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons move
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("move_x").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.move_x = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("move_y").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.move_y = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("move_z").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.move_z = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 rotation buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotation_x").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotation_x = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotation_y").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotation_y = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("rotation_z").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.rotation_z = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons color
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_color_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_color_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_color_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_color_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_color_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_color_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons material ka
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ka_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ka_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ka_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ka_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ka_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ka_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons material kd
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_kd_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_kd_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_kd_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_kd_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_kd_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_kd_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons material ks
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ks_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ks_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ks_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ks_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_ks_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_ks_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // button material p
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("material_p").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.material_p = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons lighting ia
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_ia_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_ia_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_ia_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_ia_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_ia_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_ia_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons lighting il
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_il_r").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_il_r = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_il_g").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_il_g = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_il_b").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_il_b = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 3 buttons lighting position
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_pos_x").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_pos_x = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_pos_y").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_pos_y = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("lighting_pos_z").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.lighting_pos_z = spin_button.get_value();
            area.queue_draw();
        });
    }

    // 2 buttons md mk
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("md").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.md = spin_button.get_value();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("mk").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.mk = spin_button.get_value();
            area.queue_draw();
        });
    }
}


pub fn setup_check_button_events(
    check_button: &HashMap<String, gtk::CheckButton>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 3 switch buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        check_button.get("ignore_invisible").unwrap().connect_toggled(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.hide_lines = button.get_active();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        check_button.get("carcass_visualisation").unwrap().connect_toggled(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.carcass = button.get_active();
            area.queue_draw();
        });
    }
}


pub fn setup_combo_box_events(
    combo_box: &HashMap<String, gtk::ComboBoxText>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // 2 combo box
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        combo_box["projection"].connect_changed(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.view = view_from(button
                                       .get_active_id()
                                       .unwrap()
                                       .as_str());

            area.queue_draw();
        });
    }

    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        combo_box["drawing_method"].connect_changed(move |button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.drawing_method = drawing_method_from(button
                                       .get_active_id()
                                       .unwrap()
                                       .as_str());

            area.queue_draw();
        });
    }
}
