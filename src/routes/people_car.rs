use crate::services;

#[get("/calculateDisselUsageForDistance?<distance>&<year_of_production>&<fuel_usage_per_100_km>")]
pub fn calculate_dissel_usage_for_distance(
    distance: usize,
    year_of_production: u16,
    fuel_usage_per_100_km: u8,
) -> String {
    assert!(year_of_production >= 2000 && year_of_production <= 2022);
    let fuel_usage: String =
        services::people_car::calculate_dissel_usage_for_distance(distance, fuel_usage_per_100_km);
    fuel_usage
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
pub fn probability_of_unit_injector_fail(VIN: &str) -> String {
    assert!(vin::check_validity(VIN).is_ok());
    let probability: String = services::people_car::probability_of_unit_injector_fail();
    probability
}
