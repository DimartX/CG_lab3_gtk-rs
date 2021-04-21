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

    let mut cone: Figure = Figure::new_truncated_cone(6, 1.0, 2.0, 1.0);

    let mut cone_transformation =
        TransformMatrix::new()
        .move_by_vector([-0.5, -0.5, -0.5, 1.0])
        .stretch(state.stretchOx, state.stretchOy, state.stretchOz);

    let mut axes: Figure = Figure::new_axes();

    let mut axes_transformation = TransformMatrix::new();

    match state.view {
        View::Isometric => {
            cone_transformation = cone_transformation
                .rotate_ox(to_radians(state.rotateOx))
                .rotate_oy(to_radians(state.rotateOy))
                .rotate_oz(to_radians(state.rotateOz));
            axes_transformation = axes_transformation
                .rotate_ox(to_radians(state.rotateOx))
                .rotate_oy(to_radians(state.rotateOy))
                .rotate_oz(to_radians(state.rotateOz));
        },
        View::Front => {
            cone_transformation = cone_transformation;
            axes_transformation = axes_transformation;
        },
        View::Above => {
            cone_transformation = cone_transformation
                .rotate_ox(to_radians(90.0));
            axes_transformation = axes_transformation
                .rotate_ox(to_radians(90.0));
        },
        View::Side => {
            cone_transformation = cone_transformation
                .rotate_oz(to_radians(90.0));
            axes_transformation = axes_transformation
                .rotate_oz(to_radians(90.0));
        }
    }

    cone_transformation = cone_transformation
        .move_by_vector([state.moveOx, state.moveOy, state.moveOz, 1.0])
        .zoom(state.zoom / 100.0 * coefficient);
    axes_transformation = axes_transformation
        .zoom(40.0 * coefficient);

    cone.transform(cone_transformation.mtx);
    cone.draw(canvas, ((width / 2), (height / 2)),
              state.hide_lines, state.carcass, state.filling, state.view);

    axes.transform(axes_transformation.mtx);
    axes.draw_axes(canvas, ((width as f64 * 0.9) as i32,
                       (height as f64 * 0.9) as i32));
}
