use std::io::{stdin, Read};
use std::collections::HashMap;
use serenity::http;

fn main() {
    let mut name = String::new();
    let mut profile_picture = String::new();
    let mut webhook_link = String::new();
    let mut content = String::new();
    let mut i: isize = 0;
    while (name.trim_end() == "".to_string()) {
        if i > 0 {
            println!("Please enter a name");
        }
        println!("Name:");
        stdin()
            .read_line(&mut name)
            .expect("error reading name");
        i += 1;
    }
    println!("Profile picture url (leave blank if you want default):");
    stdin()
        .read_line(&mut profile_picture)
        .expect("error reading pfp url");
    println!("Webhook url:");
    stdin()
        .read_line(&mut webhook_link)
        .expect("error reading webhook link");
    webhook_link = webhook_link.chars().rev().collect::<String>(); //flip link
    let mut id = String::from(&webhook_link);
    let mut token = String::from(&webhook_link);
    id.truncate(88); //remove the domain
    id = id.chars().rev().collect::<String>(); //reverse back to normal
    id.truncate(18); //remove the token
    token.truncate(69); //remove everything but token
    token = token.chars().rev().collect::<String>(); //reverse back to normal
    let mut webhook = http::get_webhook_with_token(id.parse::<u64>().unwrap(), token.as_str())
        .expect("valid webhook");
    loop {
        println!("Enter content:");
        content = "".to_string();
        stdin()
            .read_line(&mut content)
            .expect("error reading content");
        let _ = webhook.execute(false, |w| w
            .content(&content)
            .username(&name)
            .avatar_url(&profile_picture.as_str()))
            .expect("Error executing");
    }
}
/*
    pure parser

fn url_parser(mut url: String) -> (u64, u64) {
    url = url.chars().rev().collect::<String>();
    let mut id = String::from(&url);
    let mut token = String::from(&url);
    id.truncate(87);
    id = id.chars().rev().collect::<String>();
    id.truncate(18);
    token.truncate(70);
    token = token.chars().rev().collect::<String>();
    return (id, token);
}
*/