#[derive(Copy, Clone, Debug)]
pub enum View {
    Isometric,
    Side,
    Above,
    Front,
}

#[derive(Copy, Clone, Debug)]
pub enum DrawingMethod {
    InvisiblePolygons,
    Fill,
    FlatShading,
    GouraudShading,
}

pub fn view_from(name: &str) -> View {
    println!("name {}", name);
    match name {
        "Isometric" => View::Isometric,
        "Side"      => View::Side,
        "Above"     => View::Above,
        _           => View::Front,
    }
}

pub fn drawing_method_from(name: &str) -> DrawingMethod {
    match name {
        "Invisible" => DrawingMethod::InvisiblePolygons,
        "Fill"              => DrawingMethod::Fill,
        "FlatShading"       => DrawingMethod::FlatShading,
        _                   => DrawingMethod::GouraudShading,
    }
}
