use std::f64::consts::PI;

#[derive(Debug)]
struct GPSPos {
    latitude: f64,
    longitude: f64,
    description: String,
}

impl GPSPos {
    fn new(latitude: f64, longitude: f64, description: &str) -> Self {
        GPSPos {
            latitude,
            longitude,
            description: description.to_string(),
        }
    }
}

#[derive(Debug)]
struct GPSRoute {
    route: Vec<(GPSPos, String)>, // A vector of GPS positions with timings
}

impl GPSRoute {
    fn new() -> Self {
        GPSRoute {
            route: Vec::new(),
        }
    }

    fn add_position(&mut self, pos: GPSPos, time: &str) {
        self.route.push((pos, time.to_string()));
    }

    fn calculate_length(&self) -> f64 {
        self.route.windows(2).fold(0.0, |acc, window| {
            let (pos1, _) = &window[0];
            let (pos2, _) = &window[1];
            acc + haversine_distance(pos1.latitude, pos1.longitude, pos2.latitude, pos2.longitude)
        })
    }
}

trait Plot {
    fn plot(&self);
}

impl Plot for GPSPos {
    fn plot(&self) {
        println!(
            "Plotting GPS Position: {} at (latitude: {}, longitude: {})",
            self.description, self.latitude, self.longitude
        );
    }
}

impl Plot for GPSRoute {
    fn plot(&self) {
        for (pos, time) in &self.route {
            println!(
                "Plotting GPS Position: {} at (latitude: {}, longitude: {}) with time {}",
                pos.description, pos.latitude, pos.longitude, time
            );
        }
    }
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let to_radians = |degrees: f64| degrees * PI / 180.0;

    let dlat = to_radians(lat2 - lat1);
    let dlon = to_radians(lon2 - lon1);

    let lat1 = to_radians(lat1);
    let lat2 = to_radians(lat2);

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    // Radius of the Earth in kilometers
    let r = 6371.0;

    r * c
}

pub fn plot() {
    // Create a single GPS position
    let pos1 = GPSPos::new(37.7749, -122.4194, "San Francisco");
    pos1.plot();

    // Create a route between Madrid and Valencia
    let mut madrid_valencia_route = GPSRoute::new();
    madrid_valencia_route.add_position(GPSPos::new(40.4168, -3.7038, "Madrid"), "9:00 AM");
    madrid_valencia_route.add_position(GPSPos::new(39.4699, -0.3763, "Valencia"), "11:30 AM");
    madrid_valencia_route.plot();
    println!("Total length of Madrid to Valencia route: {:.2} km", madrid_valencia_route.calculate_length());

    // Create a route between Coruña and Madrid
    let mut coruna_madrid_route = GPSRoute::new();
    coruna_madrid_route.add_position(GPSPos::new(43.3623, -8.4115, "Coruña"), "7:00 AM");
    coruna_madrid_route.add_position(GPSPos::new(40.4168, -3.7038, "Madrid"), "12:00 PM");
    coruna_madrid_route.plot();
    println!("Total length of Coruña to Madrid route: {:.2} km", coruna_madrid_route.calculate_length());
}
