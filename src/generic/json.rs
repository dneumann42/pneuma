use serde_json::Value;

pub trait ToJson {
    fn to_json(&self) -> String;
}

pub trait FromJson {
    fn from_json(v: String) -> Result<Self, String>
    where
        Self: Sized;
}
