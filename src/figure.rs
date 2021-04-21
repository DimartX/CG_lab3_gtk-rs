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
}

impl Figure {
    pub fn new_cube() -> Self {
        Self {
            polygons: vec![
                // front
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                    ]
                ),
                // back
                Polygon::from(
                    vec![
                        [0.0, 0.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                    ]
                ),
                // above
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                        [1.0, 0.0, 0.0, 1.0],
                    ]
                ),
                // bottom
                Polygon::from(
                    vec![
                        [0.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                    ]
                ),
                // right
                Polygon::from(
                    vec![
                        [1.0, 0.0, 0.0, 1.0],
                        [1.0, 0.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 0.0, 1.0],
                    ]
                ),
                // left
                Polygon::from(
                    vec![
                        [0.0, 0.0, 0.0, 1.0],
                        [0.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 1.0, 1.0],
                        [0.0, 0.0, 1.0, 1.0],
                    ]
                ),
            ],
        }
    }

    pub fn new_truncated_cone(approx: i32, radius: f64, length: f64, real_length: f64) -> Self {
        let mut polygons: Vec<Polygon> = Vec::new();

        let up_radius = radius;
        let down_radius = radius * length / real_length;

        let real_length = -real_length;
        let mut up_polygon = Polygon::from(vec![[up_radius, real_length, 0.0, 1.0]]);
        let mut down_polygon = Polygon::from(vec![[down_radius, 0.0, 0.0, 1.0]]);

        let step: f64 = 2.0 * std::f64::consts::PI / (approx as f64);
        let mut phi: f64 = 0.0;

        for _ in 0..approx as usize {
            phi += step;

            let x_up = up_radius * phi.cos();
            let y_up = up_radius * phi.sin();

            let x_down = down_radius * phi.cos();
            let y_down = down_radius * phi.sin();

            let point_up = [x_up, real_length, y_up, 1.0];
            let point_down = [x_down, 0.0, y_down, 1.0];

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
            polygons: polygons
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

fn angle_vectors(lhs: [f64; 4], rhs: [f64; 4]) -> f64 {
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

fn draw_tile(canvas: &mut CairoCanvas, pos: (i32, i32), polygon: &Polygon, state: &State) {
    let mut pts: Vec<Point> = Vec::new();

    let normal = cross_product(
        polygon.points[1],
        polygon.points[0],
        polygon.points[2],
    );

    let angle_normal_scene = angle_vectors(norm(normal), [0.0, 0.0, -1.0, 0.0]);
    println!("Normal {:?}", normal[2]);
    println!("angle {:?}", angle_normal_scene);

    if state.hide_lines && angle_normal_scene <= 0.0 {
        return;
    }

    for point in &polygon.points {
        //println!("Point x = {}, y = {}, z = {}", point[0], point[1], point[2]);
        pts.push(Point::new(point[0] as i32 + pos.0,
                            point[1] as i32 + pos.1));
    }

    match state.drawing_method  {
        DrawingMethod::Fill => {
            let alpha = maximum(0.0, angle_normal_scene * 2.0 / 3.0);
            //println!("Alpha {}", alpha);
            canvas.set_draw_color_alpha(Color::dark_green(), maximum(0.0, 1.0 - alpha));
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
    fn transform(&mut self, matrix: [[f64; 4]; 4]) {
        for polygon in &mut self.polygons {
            polygon.points = mult_matrix_on_transform(&polygon.points, matrix);
            //println!("{:?}", polygon.points);
        }
    }

    fn draw(&self, canvas: &mut CairoCanvas, pos: (i32, i32), state: &State) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        for polygon in &self.polygons {
            draw_tile(canvas, pos, &polygon, &state);
        }

    }
}
