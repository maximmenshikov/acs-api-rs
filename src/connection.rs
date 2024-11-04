use crate::request::set_parameter_values::*;
use reqwest::blocking::Client;
use crate::device::*;
use crate::parameter_value::*;

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
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn enqueue_task(&self, _device_id: String, parameter_values: Vec<ParameterValue>) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices";

        let req = SetParameterValues::new(parameter_values.clone());
        // Send a GET request
        let response = client
            .post(&url)
            .json(&req)  // Automatically serializes the struct to JSON
        .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }
}
