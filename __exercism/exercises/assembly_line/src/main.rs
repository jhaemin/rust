pub fn production_rate_per_hour(speed: u8) -> f64 {
    let items_per_hour: f64 = 221.0;
    let success_rate: f64 = if speed == 0 {
        0.0
    } else if speed >= 1 && speed <= 4 {
        1.0
    } else if speed >= 5 && speed <= 8 {
        0.9
    } else if speed >= 9 && speed <= 10 {
        0.77
    } else {
        0.0
    };
    return (speed as f64 * items_per_hour) * success_rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

fn main() {}
