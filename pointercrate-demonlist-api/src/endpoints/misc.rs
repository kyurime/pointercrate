use pointercrate_demonlist::config;
use serde_json::json;

#[rocket::get("/")]
pub fn list_information() -> String {
    let data = json! {
        {
            "list_size": config::list_size(),
            "extended_list_size": config::extended_list_size()
        }
    };

    data.to_string()
}
