use crate::acs_type::*;
use crate::data_node::*;
use crate::device::*;
use crate::parameter_value::*;
use crate::request::add_delete_object::*;
use crate::request::download_command::*;
use crate::request::refresh_object::*;
use crate::request::set_parameter_values::*;
use crate::request::simple_command::*;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;

pub struct AcsConnection {
    pub addr: String,
    pub acs_type: AcsType,
    pub debug_log: bool,
}

impl AcsConnection {
    pub fn new(acs_type: AcsType, addr: String) -> Self {
        return Self {
            acs_type: acs_type,
            addr: addr,
            debug_log: false,
        };
    }

    fn encode_device(&self, device_id: &str) -> String {
        return encode(device_id).to_string();
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
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn set_parameter_values(
        &self,
        device_id: String,
        parameter_values: Vec<ParameterValue>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        if self.debug_log {
            eprintln!("URL: {}", url);
        }
        let req = SetParameterValues::new(parameter_values.clone());
        if self.debug_log {
            eprintln!("Request: {}", serde_json::to_string(&req).unwrap());
        }

        // Send a POST request
        let response = client.post(&url).json(&req).send();

        match response {
            Ok(ref _resp) => {
            }
            Err(err) => {
                if self.debug_log {
                    eprintln!("HTTP error while sending request: {:?}", err);
                }
                return Err(Box::from(err))
            }
        };

        let response = response.unwrap();
        if self.debug_log {
            eprintln!("Response: {:?}", response);
            eprintln!("Status: {:?}", response.status());
        }

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    fn parse_device_tree(&self, json: &Value) -> DataNode {
        let mut root = DataNode::new();

        if let Some(obj) = json.as_object() {
            for (key, value) in obj {
                let mut child_node = DataNode::new();

                if let Some(sub_obj) = value.as_object() {
                    if sub_obj.contains_key("_value") && sub_obj.contains_key("_type") {
                        child_node.value = match &sub_obj["_value"] {
                            Value::Bool(b) => b.to_string(),
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            _ => "".to_string(),
                        };
                        child_node.value_type = sub_obj["_type"]
                            .as_str()
                            .map(String::from)
                            .unwrap_or("".to_string());
                        if sub_obj.contains_key("_writable") {
                            child_node.writable = sub_obj
                                .get("_writable")
                                .and_then(|w| w.as_bool())
                                .unwrap_or(false);
                        } else {
                            /* Should it be considered writable? */
                            child_node.writable = false;
                        }
                    } else {
                        child_node = self.parse_device_tree(value);
                    }
                }

                root.subnodes.insert(key.clone(), child_node);
            }
        }

        root
    }

    pub fn get_parameter_values(
        &self,
        device_id: String,
        parameter_names: Vec<String>,
    ) -> Result<DataNode, Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices?query=%7B%22_id%22%3A%22"
            + &self.encode_device(&device_id)
            + "%22%7D&projection="
            + &parameter_names.join(",");

        // Send a GET request
        let response = client.get(&url).send()?;

        if response.status().is_success() {
            let s = response.text()?.clone();
            let json: Value = serde_json::from_str(&s)?;
            let root_device_array = json.as_array().unwrap();
            if root_device_array.len() > 0 {
                let root_device = &root_device_array[0].clone();
                if let Some(root_device_obj) = root_device.as_object() {
                    if root_device_obj.contains_key("Device") {
                        let device_node = &root_device_obj["Device"].clone();
                        let device_node = self.parse_device_tree(&device_node);
                        let mut root_node = DataNode {
                            value: "".to_string(),
                            value_type: "".to_string(),
                            writable: false,
                            subnodes: HashMap::new(),
                        };
                        root_node.subnodes.insert("Device".to_string(), device_node);
                        return Ok(root_node);
                    }
                }
            }

            return Err(Box::from("Bad response"));
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn refresh_object(
        &self,
        device_id: String,
        object: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        let req = RefreshObject::new(object);
        // Send a POST request
        let response = client.post(&url).json(&req).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn reboot(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        let req = SimpleCommand::new("reboot");
        // Send a POST request
        let response = client.post(&url).json(&req).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn factory_reset(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        let req = SimpleCommand::new("factoryReset");
        // Send a POST request
        let response = client.post(&url).json(&req).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn add_del_object(
        &self,
        device_id: String,
        add: bool,
        object_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        if self.debug_log {
            eprintln!("URL: {}", url);
        }
        let req = AddDeleteObject::new(add, &object_name);
        if self.debug_log {
            eprintln!("Request: {}", serde_json::to_string(&req).unwrap());
        }

        // Send a POST request
        let response = client.post(&url).json(&req).send();
        match response {
            Ok(ref _resp) => {
            }
            Err(err) => {
                if self.debug_log {
                    eprintln!("HTTP error while sending request: {:?}", err);
                }
                return Err(Box::from(err))
            }
        };

        let response = response.unwrap();
        if self.debug_log {
            eprintln!("Response: {:?}", response);
            eprintln!("Status: {:?}", response.status());
        }

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn del_device(&self, device_id: String) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/devices/" + &self.encode_device(&device_id);

        // Send a DELETE request
        let response = client.delete(&url).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn add_del_tag(
        &self,
        device_id: String,
        add: bool,
        tag: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url =
            self.addr.clone() + "/devices/" + &self.encode_device(&device_id) + "/tags/" + &tag;

        // Send a POST/DELETE request
        let response = if add {
            client.post(&url).send()?
        } else {
            client.delete(&url).send()?
        };

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn upload_file(
        &self,
        name: &str,
        path: &str,
        file_type: &str,
        oui: &str,
        product_class: &str,
        version: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone() + "/files/" + &name;

        // Create set of headers
        let mut headers = HeaderMap::new();
        headers.insert("fileType", HeaderValue::from_str(file_type)?);
        headers.insert("oui", HeaderValue::from_str(oui)?);
        headers.insert("productClass", HeaderValue::from_str(product_class)?);
        headers.insert("version", HeaderValue::from_str(version)?);

        // Read file
        let file_bytes = std::fs::read(path)?;

        // Send request
        let response = client.put(&url).headers(headers).body(file_bytes).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }

    pub fn download(
        &self,
        device_id: String,
        filename: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.acs_type, AcsType::GenieAcs) {
            return Err(Box::from("Unknown ACS type"));
        }

        let client = Client::new();

        // Define the URL
        let url = self.addr.clone()
            + "/devices/"
            + &self.encode_device(&device_id)
            + "/tasks?connection_request";

        let req = DownloadCommand::new(&filename);

        // Send a POST request
        let response = client.post(&url).json(&req).send()?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(Box::from(format!(
                "Response indicates failure: {}",
                response.status()
            )));
        }
    }
}
