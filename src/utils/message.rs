use crate::models::plat::Plat;

pub fn message(plats: Vec<Plat>) -> String {
    let mut message = format!("# Menu du jour\n");
    for plat in plats {
        message.push_str(&format!(" - Nom: `{}`\n", plat.name));
        message.push_str(&format!("Resaurant: `{}`\n", plat.restaurant));
        message.push_str(&format!("Type: `{}`\n", plat.category));
    }
    message
}
