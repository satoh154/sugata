use std::collections::HashMap;
use indexmap::IndexMap;

extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
use sheets4::Error;
use sheets4::Sheets;

pub async fn load_player_params(gsheet_id: String) -> HashMap<String, IndexMap<String, usize>> {
    let mut player_params:HashMap<String, IndexMap<String, usize>> = HashMap::new();
    let mut players: Vec<String> = Vec::new();

    let secret = yup_oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("client secret could not be read");

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    let hub = Sheets::new(
        hyper::Client::builder().build(connector),
        auth,
    );
    let result = hub
        .spreadsheets()
        .get(&gsheet_id) 
        .doit()
        .await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => {
            for el in res.1.sheets.unwrap() {
                players.push(el.properties.unwrap().title.unwrap());
            }
        }
    }
    for player in players {
        let range = format!("{}{}", player, "!A25:B113");
        let mut tmp_params: IndexMap<String, usize> = IndexMap::new();
        let p_result = hub
            .spreadsheets()
            .values_get(&gsheet_id, &range)
            .doit()
            .await;

        match p_result {
            Err(e) => match e {
                Error::HttpError(_)
                | Error::Io(_)
                | Error::MissingAPIKey
                | Error::MissingToken(_)
                | Error::Cancelled
                | Error::UploadSizeLimitExceeded(_, _)
                | Error::Failure(_)
                | Error::BadRequest(_)
                | Error::FieldClash(_)
                | Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => {
                for p_param in res.1.values.unwrap() {
                    let skill_name = p_param[0].clone();
                    let skill_val = p_param[1].parse::<usize>().unwrap();
                    tmp_params.insert(skill_name, skill_val);
                }
                tmp_params.sort_by(|_, b, _, d|d.cmp(b));
                player_params.insert(player, tmp_params);
            }
        }
    }
    player_params
}
