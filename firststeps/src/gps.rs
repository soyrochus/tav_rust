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
}
// You can also implement constructor and Impl methods in one block
impl GPSRoute {

    fn add_position(&mut self, pos: GPSPos, time: &str) {
        self.route.push((pos, time.to_string()));
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

pub fn plot(){
    // Create a single GPS position
    let pos1 = GPSPos::new(37.7749, -122.4194, "San Francisco");
    pos1.plot();

    // Create a route between Madrid and Valencia
    let mut madrid_valencia_route: GPSRoute = GPSRoute::new();
    madrid_valencia_route.add_position(GPSPos::new(40.4168, -3.7038, "Madrid"), "9:00 AM");
    madrid_valencia_route.add_position(GPSPos::new(39.4699, -0.3763, "Valencia"), "11:30 AM");
    madrid_valencia_route.plot();

    // Create a route between Coruña and Madrid
    let mut coruna_madrid_route = GPSRoute::new();
    coruna_madrid_route.add_position(GPSPos::new(43.3623, -8.4115, "Coruña"), "7:00 AM");
    coruna_madrid_route.add_position(GPSPos::new(40.4168, -3.7038, "Madrid"), "12:00 PM");
    coruna_madrid_route.plot();

}