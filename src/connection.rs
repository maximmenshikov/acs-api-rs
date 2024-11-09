use crate::acs_type::*;
use crate::device::*;
use crate::parameter_value::*;
use crate::request::add_delete_object::*;
use crate::request::get_parameter_values::*;
use crate::request::refresh_object::*;
use crate::request::set_parameter_values::*;
use crate::request::simple_command::*;
use reqwest::blocking::Client;

pub struct AcsConnection {
    pub addr: String,
    pub acs_type: AcsType,
}

impl AcsConnection {
    pub fn new(acs_type: AcsType, addr: String) -> Self {
        return Self { acs_type: acs_type, addr: addr };
    }

    pub fn list_devices(self: &Self) -> Result<Vec<AcsDevice>, Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices";

        // Send a GET request
        let response = client.get(&url).send()?;

        // Check if the request was successful
        if response.status().is_success() {
            // Parse the JSON response
            let s = response.text()?.clone();
            let val: Vec<AcsDevice> = serde_json::from_str(&s)?;
            return Ok(val);
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn set_parameter_values(&self, device_id: String, parameter_values: Vec<ParameterValue>) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = SetParameterValues::new(parameter_values.clone());
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn get_parameter_values(&self, device_id: String, parameter_names: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = GetParameterValues::new(parameter_names.clone());
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn refresh_object(&self, device_id: String, object: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = RefreshObject::new(object);
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn reboot(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = SimpleCommand::new("reboot");
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn factory_reset(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = SimpleCommand::new("factoryReset");
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

    pub fn add_del_object(&self, device_id: String, add: bool, object_name: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tasks?connection_request";

        let req = AddDeleteObject::new(add, &object_name);
        // Send a POST request
        let response = client
            .post(&url)
            .json(&req)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }

     pub fn del_device(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id;

        // Send a DELETE request
        let response = client
            .delete(&url)
            .send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }


    pub fn add_del_tag(&self, device_id: String, add: bool, tag: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &device_id + "/tags/" + &tag;

        // Send a POST/DELETE request
        let response = if add {
            client.post(&url).send()?
        } else {
            client.delete(&url).send()?
        };

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!("Response indicates failure: {}", response.status())));
        }
    }
}
