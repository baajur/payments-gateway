use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Deserialize, Clone)]
pub struct Password(String);

const PASSWORD_MASK: &str = "********";

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(PASSWORD_MASK)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.0)
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(PASSWORD_MASK)
    }
}

impl Password {
    pub fn new(data: String) -> Self {
        Password(data)
    }
}
