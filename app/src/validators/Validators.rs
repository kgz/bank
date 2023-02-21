use serde::{Serialize, Deserialize};

use crate::routes::{error::Error};

use super::forms::{self};

pub trait Form<'a> {
    type Inputs: Inputs<'a>;
    fn validate(data: &dyn Inputs) -> Result<String, Error>;

}

// pub static ValidationTypes : &[(&str, Types)] = &[
//     ("email" , forms::VBasicInfo::EmailTypes),
//     // auto load all in migrations folder
// ];
pub trait Inputs<'a> {}
pub trait Input<'a> {
    fn validate(data: &'a str) -> Result<String, Error>;
}









