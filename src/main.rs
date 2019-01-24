#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate reqwest;

use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotCredentials {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotTokenRequest {
    user: FarmbotCredentials,
}

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotTokenUnencoded {
    aud: String,
    sub: u64,
    iat: u64,
    jti: String,
    iss: String,
    exp: u64,
    mqtt: String,
    bot: String,
    vhost: String,
    mqtt_ws: String,
    interim_email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotToken {
    encoded: String,
    unencoded: FarmbotTokenUnencoded,
}

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotTokenResponse {
    token: FarmbotToken,
}

#[derive(Serialize, Deserialize, Debug)]
struct FarmbotUserResponse {
    id: u64,
    created_at: String,
    updated_at: String,
    name: String,
    email: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let email = &args[1];
    let password = &args[2];

    let client = reqwest::Client::new();
    let token: FarmbotTokenResponse =
        request_token(&client, email.to_string(), password.to_string()).unwrap();
    let users: Vec<FarmbotUserResponse> = get_user(&client, token.token.encoded).unwrap();
    println!("users={:?}", users);
}

fn get_user(
    client: &reqwest::Client,
    token: String,
) -> Result<Vec<FarmbotUserResponse>, reqwest::Error> {
    let mut response = client
        .get("https://my.farm.bot/api/users")
        .bearer_auth(token)
        .send()?;

    let users: Vec<FarmbotUserResponse> = response.json()?;
    Ok(users)
}

fn request_token(
    client: &reqwest::Client,
    email: String,
    password: String,
) -> Result<FarmbotTokenResponse, reqwest::Error> {
    let payload = FarmbotTokenRequest {
        user: FarmbotCredentials {
            email: email,
            password: password,
        },
    };
    let mut response = client
        .post("https://my.farm.bot/api/tokens")
        .json(&payload)
        .send()?;
    let token: FarmbotTokenResponse = response.json()?;
    Ok(token)
}
