pub const WEEKS_PER_YEAR: f64 = 52.1429;

pub fn hourly_to_weekly(hours: f64, hourly_rate: f64) -> f64 {
    hours * hourly_rate
}

pub fn weekly_to_hourly(hours: f64, weekly_rate: f64) -> f64 {
    weekly_rate / hours
}

pub fn weekly_to_annual(weekly: f64) -> f64 {
    weekly * WEEKS_PER_YEAR
}

pub fn annual_to_weekly(annual: f64) -> f64 {
    annual / WEEKS_PER_YEAR
}
