extern crate reqwest;
extern crate serde_json;

pub mod oxford {
    use reqwest::Client;
    use serde_json::Value;
    mod app {
        pub const APP_ID: &str = "249987b5";
        pub const APP_KEY: &str = "67c16e668b0ad5ea13d2b555a74e633d";
        pub const APP_BASE: &str = "https://od-api.oxforddictionaries.com/api/v2";
    }
    pub struct OxfordClient {
        pub client: Client
    }
    impl OxfordClient {
        pub fn get_word(&mut self, word: &str) -> Result<String, Box<dyn std::error::Error>> {
            let res = self.client.get(&format!("{}/entries/en-us/{}", app::APP_BASE, word))
                .header("Accept", "application/json")
                .header("app_id", app::APP_ID)
                .header("app_key", app::APP_KEY)
                .query(&[("fields", "pronunciations")])
                .send()?
                .text()?;
            let mut index: usize = 0;
            let v: Value = serde_json::from_str(&res)?;
                if let Value::Array(arr) = &v["results"][0]["lexicalEntries"][0]["pronunciations"] {
                for i in 0..arr.len() {
                    if arr[i]["phoneticNotation"].to_string() == String::from("IPA") {
                        index = i;
                        break;
                    }
                }
            }
            Ok(v["results"][0]["lexicalEntries"][0]["pronunciations"][index]["phoneticSpelling"].to_string())
        }
    }
}