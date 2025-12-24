use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub github: String,
    pub about: About,
    pub links: Links,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct About {
    pub name: String,
    pub location: Option<String>,
    pub intro: String,
    pub langs: Vec<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub projects: Option<Vec<Project>>,
    pub skills: Option<Skills>,
    pub achievements: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Experience {
    pub title: String,
    pub organization: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub description: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Education {
    pub institute: String,
    pub course: String,
    pub duration: String,
    pub cgpa: Option<String>,
    pub location: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    pub name: String,
    pub stack: Vec<String>,
    pub description: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Skills {
    pub cs_fundamentals: Vec<String>,
    pub frameworks_tools: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Links {
    pub github: String,
    pub email: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Profile {
    pub username: String,
    pub langs: Vec<String>,
    pub info: UserInfo,
    pub stats: UserStats,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u16,
    pub company: Option<String>,
    pub location: Option<String>,
    pub followers: u16,
    pub following: u16,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserStats {
    pub stars: u16,
    pub forks: u16,
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub response: Vec<Repository>,
}

#[derive(Deserialize, Serialize)]
pub struct Repos {
    pub repos: Vec<Repository>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Repository {
    pub author: String,
    pub name: String,
    pub description: String,
    pub stars: u16,
    pub forks: u16,
    pub language: String,
}
