use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;

pub type Request = for<'welcome> fn(WelcomeInformation<'welcome>);
