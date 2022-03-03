pub fn adsense_publisher_id() -> String {
    std::env::var("ADSENSE_PUBLISHER_ID")
        .expect("No google adsense publisher ID configured. Please remove all advertisement from your custom copy of pointercrate")
}

pub fn google_analytics_tag() -> Option<String> {
    std::env::var("ANALYTICS_TAG").ok()
}
