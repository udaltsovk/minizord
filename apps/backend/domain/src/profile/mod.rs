use lib::domain::Id;

use crate::profile::{
    bio::ProfileBio, city::ProfileCity, name::ProfileName,
    portfolio_url::ProfilePortfolioUrl, surname::ProfileSurname,
    telegram::ProfileTelegram,
};

mod bio;
mod city;
mod name;
mod portfolio_url;
mod surname;
mod telegram;

pub struct Profile {
    pub id: Id<Profile>,
    pub name: ProfileName,
    pub surname: ProfileSurname,
    pub telegram: ProfileTelegram,
    pub city: ProfileCity,
    pub bio: ProfileBio,
    pub portfolio_urls: Vec<ProfilePortfolioUrl>,
    // pub has_avatar: bool,
}

pub struct CreateProfile {
    pub name: ProfileName,
    pub surname: ProfileSurname,
    pub telegram: ProfileTelegram,
    pub city: ProfileCity,
    pub bio: ProfileBio,
    pub portfolio_urls: Vec<ProfilePortfolioUrl>,
    // pub has_avatar: bool,
}

pub struct UpdateProfile {
    pub name: ProfileName,
    pub surname: ProfileSurname,
    pub telegram: ProfileTelegram,
    pub city: ProfileCity,
    pub bio: ProfileBio,
    pub portfolio_urls: Vec<ProfilePortfolioUrl>,
    // pub has_avatar: bool,
}
