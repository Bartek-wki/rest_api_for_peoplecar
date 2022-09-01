use crate::services;

#[get("/calculateDisselUsageForDistance?<distance>&<year_of_production>&<fuel_usage_per_100_km>")]
pub fn calculate_dissel_usage_for_distance(
    distance: usize,
    year_of_production: u16,
    fuel_usage_per_100_km: u8,
) -> String {
    let fuel_usage =
        services::people_car::calculate_dissel_usage_for_distance(distance, fuel_usage_per_100_km);
    return format!("{}", fuel_usage);
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
pub fn probability_of_unit_injector_fail(VIN: &str) -> String {
    let probability = services::people_car::probability_of_unit_injector_fail();

    fn check_vin(my_vin: &str) -> Result<&str, &str> {
        let error = "Wrong VIN";

        if vin::check_validity(my_vin).is_ok() {
            Ok(my_vin)
        } else {
            Err(error)
        }
    }

    match check_vin(VIN) {
        Ok(_v) => format!("{}", probability),
        Err(_e) => format!("You have probably entered an incorrect VIN number: {}", VIN),
    }
}
