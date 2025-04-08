use nova_forms::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

// Define the form data structure.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DemoForm {
    datatypes: Datatypes,
    address: Address,
    #[serde(default)]
    children: Vec<ChildData>,
    #[serde(default)]
    files: Vec<FileId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Datatypes {
    string: String,
    non_empty_string: NonEmptyString,
    email: Email,
    phone: Phone,
    number: u64,
    date: Date,
    time: Time,
    date_time: DateTime,
    #[serde(default)]
    bool: bool,
    accept: Accept,
    radio: RadioValue,
    select: RadioValue,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    street: String,
    zip: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChildData {
    name: NonEmptyString,
}

// Define an enum for the radio button and the select field.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display, IntoStaticStr, Default)]
pub enum RadioValue {
    #[default]
    A,
    B,
    C,
}
