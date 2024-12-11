use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CVData {
    pub name: String,
    pub initials: String,
    pub location: String,
    pub location_link: String,
    pub about: String,
    pub summary: String,
    pub avatar_url: String,
    pub personal_website_url: String,
    pub contact: Contact,
    pub education: Vec<Education>,
    pub work: Vec<Work>,
    pub skills: Vec<String>,
    pub projects: Vec<Project>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Contact {
    pub email: String,
    pub tel: String,
    pub social: Vec<Social>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Social {
    pub name: String,
    pub url: String,
    pub icon: String, // Assuming icon is a string representation
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub start: String,
    pub end: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Work {
    pub company: String,
    pub link: Option<String>,
    pub badges: Vec<String>,
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub description: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Project {
    pub title: String,
    pub tech_stack: Option<Vec<String>>,
    pub description: String,
    pub logo: String, // Assuming logo is a string representation
    pub link: Option<Link>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Link {
    pub label: String,
    pub href: String,
}
