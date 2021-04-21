use crate::figure::{FigureImpl, Figure};
use crate::state::State;
use crate::canvas::{CairoCanvas, Canvas};
use crate::transformations::TransformMatrix;
use crate::view::View;
use std::cmp::min;

use std::cell::Ref;

use std::f64::consts::PI;
fn to_radians(angle: f64) -> f64 {
    angle / 180.0 * PI
}

pub fn handle_draw(canvas: &mut CairoCanvas, state: &Ref<State>) {
    let width = canvas.width();
    let height = canvas.height();

    let coefficient = min(width, height) as f64 / 600.0;

    let mut cone: Figure = Figure::new_truncated_cone(state.approx as i32, 1.0, 2.0, 1.0);

    let mut cone_transformation =
        TransformMatrix::new()
        //.move_by_vector([-0.5, -0.5, -1.0, 1.0])
        .stretch(state.parameter_a, state.parameter_b, state.parameter_c);

    let mut axes: Figure = Figure::new_axes();

    let mut axes_transformation = TransformMatrix::new();

    println!("View {:?}", state.view);
    println!("Drawing method {:?}", state.drawing_method);

    match state.view {
        View::Isometric => {
            cone_transformation = cone_transformation
                .rotate_ox(to_radians(state.rotation_x))
                .rotate_oy(to_radians(state.rotation_y))
                .rotate_oz(to_radians(state.rotation_z));
            axes_transformation = axes_transformation
                .rotate_ox(-to_radians(state.rotation_x))
                .rotate_oy(to_radians(state.rotation_y))
                .rotate_oz(-to_radians(state.rotation_z));
        },
        View::Front => {
            cone_transformation = cone_transformation;
                //.rotate_oz(to_radians(state.rotation_z));
            axes_transformation = axes_transformation;
        },
        View::Above => {
            cone_transformation = cone_transformation
                .rotate_ox(to_radians(90.0));
                //.rotate_oy(to_radians(state.rotation_x));
            axes_transformation = axes_transformation
                .rotate_ox(to_radians(90.0));
        },
        View::Side => {
            cone_transformation = cone_transformation
                //.rotate_oz(to_radians(state.rotation_z))
                .rotate_oz(to_radians(90.0));
            axes_transformation = axes_transformation
                .rotate_oz(to_radians(90.0));
        }
    }

    cone_transformation = cone_transformation
        .move_by_vector([state.move_x, state.move_y, state.move_z, 1.0])
        .zoom(coefficient);
    axes_transformation = axes_transformation
        .zoom(40.0 * coefficient);

    cone.transform(cone_transformation.mtx);
    cone.draw(canvas, ((width / 2), (height / 2)), &state);

    axes.transform(axes_transformation.mtx);
    axes.draw_axes(canvas, ((width as f64 * 0.9) as i32,
                       (height as f64 * 0.9) as i32));
}
