#[derive(Clone)]
pub struct Text {
    len: u8,
    val: String,
}

impl Text {
    pub fn from_str(val: &str) -> Self {
        let len = val.len() as u8;
        Text { len, val: val.to_string() }
    }

    pub fn get_len(&self) -> u8 {
        self.len
    }

    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn clone(&self) -> Self {
        Text { len: self.len, val: self.val.clone() }
    }
}
