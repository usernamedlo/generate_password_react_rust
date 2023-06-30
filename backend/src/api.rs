use actix_web::{ web, HttpResponse, Responder };
use rand::Rng;
use std::collections::HashMap;

pub async fn generate_password(
    length: web::Path<i32>,
    query: web::Query<HashMap<String, String>>
) -> impl Responder {
    let password_length = length.into_inner();
    let num_passwords = query.get("numPasswords").map_or(1, |v| v.parse().unwrap_or(1));
    let include_uppercase = query
        .get("includeUppercase")
        .map_or(false, |v| v.parse().unwrap_or(false));
    let include_special_chars = query
        .get("includeSpecialChars")
        .map_or(false, |v| v.parse().unwrap_or(false));

    let mut passwords = Vec::new();

    for _ in 0..num_passwords {
        let password = generate_random_password(
            password_length,
            include_uppercase,
            include_special_chars
        );
        passwords.push(password);
    }

    let passwords = passwords.join("\n");
    HttpResponse::Ok().body(passwords)
}

fn generate_random_password(
    length: i32,
    include_uppercase: bool,
    include_special_chars: bool
) -> String {
    let mut rng = rand::thread_rng();
    let mut characters = String::from("abcdefghijklmnopqrstuvwxyz0123456789");

    if include_uppercase {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if include_special_chars {
        characters.push_str("!@#$%^&*()");
    }

    let mut random_password = String::new();

    let mut has_uppercase = false;
    let mut has_special_char = false;

    for _ in 0..length {
        let random_number = rng.gen_range(0..characters.len());
        let random_char = characters.chars().nth(random_number).unwrap();

        random_password.push(random_char);

        if include_uppercase && !has_uppercase && random_char.is_ascii_uppercase() {
            has_uppercase = true;
        }

        if include_special_chars && !has_special_char && "!@#$%^&*()".contains(random_char) {
            has_special_char = true;
        }
    }

    if include_uppercase && !has_uppercase {
        let random_uppercase = rng.gen_range(0..26);
        random_password.push((('A' as u8) + random_uppercase) as char);
    }

    if include_special_chars && !has_special_char {
        let random_special_char = rng.gen_range(0..8);
        random_password.push("!@#$%^&*()".chars().nth(random_special_char).unwrap());
    }

    random_password
}
