use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;

pub type Welcome = fn() -> WelcomeInformation<'static>;
