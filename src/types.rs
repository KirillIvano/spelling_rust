use std::collections::HashMap;

pub type FreqDict = HashMap<String, u32>;
pub type RelationDict = HashMap<String, HashMap<String, bool>>;