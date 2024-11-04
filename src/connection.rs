use reqwest::blocking::Client;
use crate::device::*;
pub struct AcsConnection {
    pub addr: String,
}

impl AcsConnection {
    pub fn new(addr: String) -> Self {
        return Self { addr: addr };
    }

    pub fn list_devices(self: &Self) -> Result<Vec<AcsDevice>, Box<dyn std::error::Error>> {
        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices";

        // Send a GET request
        let response = client.get(&url).send()?;

        // Check if the request was successful
        if response.status().is_success() {
            // Parse the JSON response
            let s = response.json::<String>()?.clone();
            let val: Vec<AcsDevice> = serde_json::from_str(&s)?;
            return Ok(val);
        } else {
            println!("Failed to send request: {}", response.status());
            return Ok(Vec::new());
        }
    }
}
