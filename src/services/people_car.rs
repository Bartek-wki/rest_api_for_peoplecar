use rand::Rng;

pub fn calculate_dissel_usage_for_distance(distance: usize, fuel_usage_per_100_km: u8) -> String {
    let fuel_usage: f32 = distance as f32 * fuel_usage_per_100_km as f32 / 100 as f32;
    format!("{}", fuel_usage)
}

pub fn probability_of_unit_injector_fail() -> String {
    let random_value = rand::thread_rng().gen_range(0..101);
    let fail_probability: f32 = random_value as f32 / 100 as f32;
    format!("{}", fail_probability)
}
