use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cafeteria {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub resto_type: i32,
    pub resto_sub_type: i32,
    pub number_servings: i32,
    pub menu_line_type: i32,
    pub customer_group: i32,
    pub entry_unit: i32,
    pub weekly_plan: i32,
    pub daily_plan: i32,
    pub daily_plan_snacks: bool,
    pub allow_change_settings: bool,

    pub opening_hours: Option<String>,

    pub closed_next_monday: bool,
    pub closed_next_tuesday: bool,
    pub closed_next_wednesday: bool,
    pub closed_next_thursday: bool,
    pub closed_next_friday: bool,
    pub closed_next_saturday: bool,
    pub closed_next_sunday: bool,
    pub has_eco_score: bool,
    pub format: i32,
    pub url_site: Option<String>,
    pub url_location: Option<String>,
    pub require_entry_number_of_servings: bool,
    pub organization: Option<String>,
}