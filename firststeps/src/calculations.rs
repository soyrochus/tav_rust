use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Plane { width: f64, height: f64 },
    Sphere { radius: f64 },
    Curve { points: Vec<(f64, f64)> },
}

fn calculate_surface(shape: Shape) -> Option<f64> {
    match shape {
        Shape::Plane { width, height } => Some(plane_surface(width, height)),
        Shape::Sphere { radius } => Some(sphere_surface(radius)),
        Shape::Curve { points } => curve_length(points),
    }
}

fn plane_surface(width: f64, height: f64) -> f64 {
    width * height
}

fn sphere_surface(radius: f64) -> f64 {
    4.0 * PI * radius * radius
}

fn curve_length(points: Vec<(f64, f64)>) -> Option<f64> {
    if points.len() < 2 {
        return None; // Not enough points to form a curve
    }

    let mut length = 0.0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        length += ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    }

    Some(length)
}

pub fn calculate() {
    // Example calculations

    // Plane surface calculation
    let plane = Shape::Plane { width: 5.0, height: 3.0 };
    if let Some(area) = calculate_surface(plane) {
        println!("Plane surface area: {}", area);
    }

    // Sphere surface calculation
    let sphere = Shape::Sphere { radius: 2.0 };
    if let Some(surface_area) = calculate_surface(sphere) {
        println!("Sphere surface area: {}", surface_area);
    }

    // Curve length calculation
    let curve = Shape::Curve {
        points: vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)],
    };
    if let Some(length) = calculate_surface(curve) {
        println!("Curve length: {}", length);
    } else {
        println!("Not enough points to form a curve.");
    }
}
