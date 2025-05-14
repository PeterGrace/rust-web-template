pub const METRIC_APP_INFO: &str = "app_info";

pub fn register_metrics() {
    describe_counter!(METRIC_APP_INFO, "Information regarding version and git build of this instantiaton");
}