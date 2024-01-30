use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}


pub fn get_timezone_from_location(lat: f64, lng: f64) -> &'static str {
    FINDER.get_tz_name(lng, lat)
}
