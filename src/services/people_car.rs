use rand::Rng;

pub fn calculate_dissel_usage_for_distance(distance: usize, fuel_usage_per_100_km: u8) -> f32 {
    return distance as f32 * fuel_usage_per_100_km as f32 / 100 as f32;
}

pub fn probability_of_unit_injector_fail() -> f32 {
    let random_value = rand::thread_rng().gen_range(0..101);
    return random_value as f32 / 100 as f32;
}
