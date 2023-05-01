pub fn adsense_publisher_id() -> Option<String> {
    std::env::var("ADSENSE_PUBLISHER_ID").ok()
}

pub fn google_analytics_tag() -> Option<String> {
    std::env::var("ANALYTICS_TAG").ok()
}
