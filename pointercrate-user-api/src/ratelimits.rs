use pointercrate_core::ratelimits;

ratelimits! {
    UserRatelimits {
        registrations[1u32 per 86400 per ip] => "Too many registrations!",
        soft_registrations[5u32 per 21600 per ip] => "Too many failed registration attempts!",
        login_attempts[3u32 per 1800 per ip] => "Too many login attempts!",
        change_email[1u32 per 2592000 per ip] => "Too many attempted email changes",
    }
}
