use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[repr(C)]
pub struct DataNode {
    pub value: String,
    pub value_type: String,
    pub subnodes: HashMap<String, DataNode>,
}

unsafe impl Send for DataNode {}

impl DataNode {
    pub fn new() -> Self {
        Self {
            value: "".to_string(),
            value_type: "".to_string(),
            subnodes: HashMap::new(),
        }
    }

    pub fn get_subnode(&self, name: &str) -> Option<DataNode> {
        if self.subnodes.contains_key(name) {
            return Some(self.subnodes[name].clone());
        }
        return None;
    }

    pub fn merge(&mut self, node: &DataNode) {
        self.value = node.value.clone();
        self.value_type = node.value_type.clone();
        for subnode in &node.subnodes {
            let idx = subnode.0.clone();

            if !self.subnodes.contains_key(&idx) {
                let tmp = DataNode::new();
                self.subnodes.insert(idx.clone(), tmp);
            }

            let new_node = self.subnodes.get_mut(&idx).unwrap();
            new_node.merge(&subnode.1);
        }
    }
}
