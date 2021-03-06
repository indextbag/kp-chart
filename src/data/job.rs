use std::fmt::{self, Display, Formatter};

use crate::data::Ability;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Job {
    name: String,
    people: Vec<Ability>,
}

impl Job {
    pub fn new(name: &'static str, people: Vec<Ability>) -> Self {
        Self {
            name: name.to_string(),
            people,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn people(&self) -> &[Ability] {
        &self.people
    }
}

impl Display for Job {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}
