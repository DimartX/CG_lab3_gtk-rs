use crate::canvas::{CairoCanvas, Canvas};
use std::vec::Vec;
use crate::transformations::*;
use crate::point::Point;
use crate::color::Color;
use crate::view::*;
use crate::state::State;

struct Polygon {
    points: Vec<[f64; 4]>,
}

impl Polygon {
    fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
    fn from(pts: Vec<[f64; 4]>) -> Self {
        Self {
            points: pts,
        }
    }
}

pub trait FigureImpl {
    fn transform(&mut self, matrix: [[f64; 4]; 4]);
    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32), state: &State);
}

pub struct Figure {
    polygons: Vec<Polygon>,
    lighting_pos: [f64; 4],
}

impl Figure {
    pub fn new_truncated_cone(approx: i32, radius: f64, length: f64, real_length: f64,
                              lighting_pos: [f64; 4],) -> Self {
        let mut polygons: Vec<Polygon> = Vec::new();

        let up_radius = radius;
        let down_radius = radius * length / real_length;

        let real_length = -real_length;

        let top = real_length / 2.0;
        let bottom = -real_length / 2.0;

        let mut up_polygon = Polygon::from(vec![[up_radius, top, 0.0, 1.0]]);
        let mut down_polygon = Polygon::from(vec![[down_radius, bottom, 0.0, 1.0]]);

        let step: f64 = 2.0 * std::f64::consts::PI / (approx as f64);
        let mut phi: f64 = 0.0;


        for _ in 0..approx as usize {
            phi += step;

            let x_up = up_radius * phi.cos();
            let y_up = up_radius * phi.sin();

            let x_down = down_radius * phi.cos();
            let y_down = down_radius * phi.sin();

            let point_up = [x_up, top, y_up, 1.0];
            let point_down = [x_down, bottom, y_down, 1.0];

            polygons.push(Polygon::from(vec![
                point_up,
                point_down,
                down_polygon.points.last().unwrap().clone(),
                up_polygon.points.last().unwrap().clone(),
            ]));

            //println!("Side polygon N {} is {:?}", i, polygons[i].points);
            up_polygon.points.push(point_up);
            down_polygon.points.push(point_down);
        }

        up_polygon.points.reverse();
        polygons.push(up_polygon);
        polygons.push(down_polygon);

        Self {
            polygons: polygons,
            lighting_pos:     [
                lighting_pos[0],
                lighting_pos[1],
                lighting_pos[2],
                1.0,
            ],

        }
    }

    pub fn new_axes() -> Self {
        Self {
            polygons: vec![
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, -1.0, 0.0, 1.0],
                    ]
                ),
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                    ]
                ),
            ],
            lighting_pos: [0.0; 4],
        }
    }

    pub fn draw_axes(&self, canvas: &mut CairoCanvas, pos: (i32, i32)) {
        let array_colors = [Color::red(), Color::green(), Color::blue()];
        for i in 0..3 {
            let mut pts: Vec<Point> = Vec::new();
            canvas.set_draw_color(array_colors[i].clone());
            for point in &self.polygons[i].points {
                //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
                pts.push(Point::new(point[0] as i32 + pos.0,
                                    point[1] as i32 + pos.1));
            }
            canvas.draw_polygon(&pts);
        }
    }
}

fn cross_product(a: [f64; 4], b: [f64; 4], c: [f64; 4]) -> [f64; 4] {
    [
        (a[1] - c[1]) * (b[2] - c[0]) - (a[2] - c[2]) * (b[1] - c[1]),
        (a[2] - c[2]) * (b[0] - c[0]) - (a[0] - c[0]) * (b[2] - c[2]),
        (a[0] - c[0]) * (b[1] - c[1]) - (a[1] - c[1]) * (b[0] - c[0]),
        1.0,
    ]
}

fn dot_product(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn cos_vectors(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
    dot_product(lhs, rhs) /
        (dot_product(lhs, lhs).sqrt() * dot_product(rhs, rhs).sqrt())
}

fn norm(vector: [f64; 4]) -> [f64; 4] {
    let norma = dot_product(vector, vector).sqrt();
    [
        vector[0] / norma,
        vector[1] / norma,
        vector[2] / norma,
        1.0,
    ]
}

fn maximum(a: f64, b: f64) -> f64{
    if a > b {
        a
    }
    else {
        b
    }
}

fn ambient_component(state: &State) -> [f64; 3] {
    [
        state.lighting_ia_r * state.material_ka_r,
        state.lighting_ia_g * state.material_ka_g,
        state.lighting_ia_b * state.material_ka_b,
    ]
}

fn diffuse_component(state: &State, vector_l: [f64; 4], normal: [f64; 4]) -> [f64; 3] {
    [
        state.lighting_il_r * state.material_kd_r *
            maximum(dot_product(vector_l, normal), 0.0),
        state.lighting_il_g * state.material_kd_g *
            maximum(dot_product(vector_l, normal), 0.0),
        state.lighting_il_b * state.material_kd_b *
            maximum(dot_product(vector_l, normal), 0.0),
    ]
}

fn specular_component(state: &State, vector_r: [f64; 4], vector_s: [f64; 4]) -> [f64; 3] {
    [
        state.lighting_il_r * state.material_ks_r *
            maximum(cos_vectors(vector_r, vector_s), 0.0).powf(state.material_p),
        state.lighting_il_g * state.material_ks_g *
            maximum(cos_vectors(vector_r, vector_s), 0.0).powf(state.material_p),
        state.lighting_il_b * state.material_ks_b *
            maximum(cos_vectors(vector_r, vector_s), 0.0).powf(state.material_p),
    ]
}

fn mult(vector: [f64; 4], num: f64) -> [f64; 4] {
    [
        vector[0] * num,
        vector[1] * num,
        vector[2] * num,
        1.0,
    ]
}

fn minus(lhs: [f64; 4], rhs: [f64; 4]) -> [f64; 4] {
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1],
        lhs[2] - rhs[2],
        1.0,
    ]
}

fn draw_tile(canvas: &mut CairoCanvas, pos: (i32, i32), polygon: &Polygon, state: &State,
             lighting_pos: [f64; 4]) {
    let mut pts: Vec<Point> = Vec::new();

    let normal = cross_product(
        polygon.points[1],
        polygon.points[0],
        polygon.points[2],
    );

    let my_eye = [0.0, 0.0, -1.0, 1.0];

    let angle_normal_scene = cos_vectors(norm(normal), my_eye);

    if state.hide_lines && angle_normal_scene <= 0.0 {
        return;
    }

    let mut center = [0.0; 4];
    for point in &polygon.points {
        //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
        pts.push(Point::new(point[0] as i32 + pos.0,
                            point[1] as i32 + pos.1));
        for i in 0..3 {
            center[i] += point[i];
        }
    }
    for i in 0..3 {
        center[i] /= polygon.points.len() as f64;
    }

    let mut vector_l = lighting_pos;
    for i in 0..3 {
        vector_l[i] -= center[i];
    }
    let dist = dot_product(vector_l, vector_l).sqrt();

    let vector_r = norm(minus(vector_l, mult(normal, 2.0 * dot_product(normal, vector_l))));

    vector_l = norm(vector_l);

    match state.drawing_method  {
        DrawingMethod::Fill => {
            let alpha = maximum(0.0, angle_normal_scene * 2.0 / 3.0);
            //println!("Alpha {}", alpha);
            canvas.set_draw_color_alpha(Color::dark_green(), maximum(0.0, 1.0 - alpha));
            canvas.fill_polygon(&pts);
        }
        DrawingMethod::FlatShading => {
            let amb = ambient_component(state);
            let dif = diffuse_component(state, vector_l, norm(normal));
            let spec = specular_component(state, vector_r, my_eye);



            let mut result = [0.0; 3];
            for i in 0..3 {
                result[i] = amb[i] + (dif[i] + spec[i]) / (state.md * dist + state.mk);
            }

            //println!("Colors {:?}", result);
            canvas.set_draw_color_rgb(
                result[0] * state.material_color_r / 255.0,
                result[1] * state.material_color_g / 255.0,
                result[2] * state.material_color_b / 255.0,
            );
            canvas.fill_polygon(&pts);
        }
        _ => {}
    }

    if state.carcass {
        canvas.set_draw_color(Color::black());
        canvas.draw_polygon(&pts);
    }
}

impl FigureImpl for Figure {
    // transform all that stuff
    fn transform(&mut self, matrix: [[f64; 4]; 4]) {
        for polygon in &mut self.polygons {
            polygon.points = mult_matrix_on_transform(&polygon.points, matrix);
            //println!("{:?}", polygon.points);
        }
    }

    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32), state: &State) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        for polygon in &self.polygons {
            draw_tile(canvas, pos, &polygon, &state, self.lighting_pos);
        }

        canvas.set_draw_color(Color::red());
        canvas.draw_filled_circle(&Point::new(
            self.lighting_pos[0] as i32 + pos.0,
            self.lighting_pos[1] as i32 + pos.1,
        ), 5.0);
    }
}
