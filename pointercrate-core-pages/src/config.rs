pub fn google_analytics_tag() -> Option<String> {
    std::env::var("ANALYTICS_TAG").ok()
}
