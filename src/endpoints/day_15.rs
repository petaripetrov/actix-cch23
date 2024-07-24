use actix_web::{post, web, HttpResponse};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::common::{EndpointRet, PasswordErrors, ServerError};

#[derive(Deserialize, Serialize)]
struct PasswordBody {
    #[serde(rename(serialize = "result"))]
    input: String,
}

#[post("15/nice")]
async fn password_nice(body: web::Json<Value>) -> EndpointRet {
    let input = match serde_json::from_value(body.into_inner()) {
        Ok(PasswordBody { input }) => input,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(PasswordBody {
                input: "naughty".to_owned(),
            }))
        }
    };

    const EXCLUDE: &[&str; 4] = &["ab", "cd", "pq", "xy"];
    const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

    let mut consec = false;
    let mut vowel_cnt = 0;

    for i in 0..input.len() - 1 {
        // TODO refactor with iter and window(2)
        let window = &input[i..i + 2];
        let mut c = window.chars();
        let (left, right) = (c.next().unwrap(), c.next().unwrap());

        if left.is_alphabetic() && left == right {
            consec = true;
        }

        if VOWELS.iter().any(|&x| x == left) {
            vowel_cnt += 1;
        }

        if EXCLUDE.iter().any(|&x| x == window) {
            return Ok(HttpResponse::BadRequest().json(PasswordBody {
                input: "naughty".to_owned(),
            }));
        }
    }

    let res = if consec && vowel_cnt >= 3 {
        HttpResponse::Ok().json(PasswordBody {
            input: "nice".to_owned(),
        })
    } else {
        HttpResponse::BadRequest().json(PasswordBody {
            input: "naughty".to_owned(),
        })
    };

    Ok(res)
}

#[post("15/game")]
async fn password_game(body: web::Json<PasswordBody>) -> EndpointRet {
    let int_re = Regex::new(r"\d+").unwrap();
    let ioy_re = Regex::new(r"(j (.*) o (.*) y)").unwrap();
    let range = '\u{2980}'..='\u{2BFF}';

    let input = body.into_inner().input;

    if input.len() < 8 {
        // TODO Refactor this to look like the rest (bool flag and then ret check)
        return Err(ServerError::PasswordError(PasswordErrors::LessEightChars));
    }

    if !(input.chars().any(|x| x.is_uppercase())
        && input.chars().any(|x| x.is_lowercase())
        && input.chars().any(|x| x.is_digit(10)))
    {
        // TODO refacor this to look like the rest and use the main loop (for speed)
        return Err(ServerError::PasswordError(
            PasswordErrors::MissingCharacterTypes,
        ));
    }

    let mut digit_c: usize = 0;
    let int_vec: Vec<i32> = int_re
        .find_iter(&input)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();
    let mut is_sandwich = false;
    let mut in_range = false;
    let mut has_emoji = false;
    let sha_ends_on_a = sha256::digest(&input).ends_with('a');

    let vec: Vec<char> = input.chars().collect();
    // _x is necessary because the first value (None) is never actually read
    // and the compiler throws a warning
    let (mut _x, mut y, mut z) = (None, None, None);

    for c in vec {
        if let Some(_) = c.to_digit(10) {
            digit_c += 1;
        }

        _x = y;
        y = z;
        z = Some(c);

        if [_x, y, z].iter().all(|f| f.is_some()) && (_x == z && _x != y && _x.unwrap().is_alphabetic())
        {
            is_sandwich = true;
        }

        if range.contains(&c) {
            in_range = true;
        }

        if unic_emoji_char::is_emoji_presentation(c) {
            has_emoji = true;
        }
    }

    if digit_c < 5 {
        return Err(ServerError::PasswordError(PasswordErrors::LessFiveDigits));
    }

    if int_vec.iter().sum::<i32>() != 2023 {
        return Err(ServerError::PasswordError(PasswordErrors::MathIsHard));
    }

    if !ioy_re.is_match(&input) {
        return Err(ServerError::PasswordError(PasswordErrors::IOYOutOrder));
    }

    if !is_sandwich {
        return Err(ServerError::PasswordError(PasswordErrors::MissingSandwich));
    }

    if !in_range {
        return Err(ServerError::PasswordError(
            PasswordErrors::UnicodeOutOfRange,
        ));
    }

    if !has_emoji {
        return Err(ServerError::PasswordError(PasswordErrors::MissingEmoji));
    }

    if !sha_ends_on_a {
        return Err(ServerError::PasswordError(PasswordErrors::ShaNotEndWithA));
    }

    Ok(HttpResponse::Ok().json(json!({
        "result": "nice",
        "reason": "that's a nice password"
    })))
}
