use actix_web::{ web, HttpResponse, Responder };
use rand::Rng;
use std::collections::HashMap;

pub async fn generate_password(
    length: web::Path<i32>,
    include_uppercase: web::Query<HashMap<String, bool>>,
    include_special_chars: web::Query<HashMap<String, bool>>
) -> impl Responder {
    let password_length = length.into_inner();
    let include_uppercase = include_uppercase
        .into_inner()
        .get("includeUppercase")
        .cloned()
        .unwrap_or(false);
    let include_special_chars = include_special_chars
        .into_inner()
        .get("includeSpecialChars")
        .cloned()
        .unwrap_or(false);

    let password = generate_random_password(
        password_length,
        include_uppercase,
        include_special_chars
    );

    println!("Generated password: {}", &password);

    HttpResponse::Ok().body(password)
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

    // Variables pour suivre si des majuscules et des caractères spéciaux ont été ajoutés
    let mut has_uppercase = false;
    let mut has_special_char = false;

    for _ in 0..length {
        let random_number = rng.gen_range(0..characters.len());
        let random_char = characters.chars().nth(random_number).unwrap();

        random_password.push(random_char);

        // Vérifier si une majuscule a été ajoutée
        if include_uppercase && !has_uppercase && random_char.is_ascii_uppercase() {
            has_uppercase = true;
        }

        // Vérifier si un caractère spécial a été ajouté
        if include_special_chars && !has_special_char && "!@#$%^&*()".contains(random_char) {
            has_special_char = true;
        }
    }

    // Ajouter une majuscule au hasard si nécessaire
    if include_uppercase && !has_uppercase {
        let random_uppercase = rng.gen_range(0..26);
        random_password.push((('A' as u8) + random_uppercase) as char);
    }

    // Ajouter un caractère spécial au hasard si nécessaire
    if include_special_chars && !has_special_char {
        let random_special_char = rng.gen_range(0..8);
        random_password.push("!@#$%^&*()".chars().nth(random_special_char).unwrap());
    }

    random_password
}