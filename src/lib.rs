//! SteamAPI abstraction, that tries to make bulk requests as fast as possible.
//!
//! # Current state
//!
//! Currently provides abstractions for the following endpoints:
//! - [X] [`api.steampowered.com/ISteamUser/ResolveVanityURL/v1/`]
//! - [X] [`api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/`]
//! - [X] [`api.steampowered.com/ISteamUser/GetFriendList/v1/`]
//! - [X] [`api.steampowered.com/ISteamUser/GetPlayerBans/v1/`]
//! - [X] [`steamcommunity.com/search/SearchCommunityAjax/`]
//!
//! [`api.steampowered.com/ISteamUser/ResolveVanityURL/v1/`]: https://api.steampowered.com/ISteamUser/ResolveVanityURL/v1/
//! [`api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/`]: https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/
//! [`api.steampowered.com/ISteamUser/GetFriendList/v1/`]: https://api.steampowered.com/ISteamUser/GetFriendList/v1/
//! [`api.steampowered.com/ISteamUser/GetPlayerBans/v1/`]: https://api.steampowered.com/ISteamUser/GetPlayerBans/v1/
//! [`steamcommunity.com/search/SearchCommunityAjax/`]: https://steamcommunity.com/search/SearchCommunityAjax/
//!
//! # Other
//!
//! Also provides a class for handling [`SteamId`][crate::steam_id::SteamId]s.

#![feature(int_roundings)]
#![feature(generic_arg_infer)]

pub mod constants;
mod request_helper;

mod enums;
pub use enums::*;

mod steam_id;
pub use steam_id::*;

mod steam_id_ext;
pub use steam_id_ext::SteamIdExt;

#[cfg(feature = "friend_code")]
mod steam_id_friend_code;

#[cfg(feature = "friend_code")]
mod bit_chunks;

mod vanity_url;
pub use vanity_url::*;

mod player_summary;
pub use player_summary::*;

mod player_bans;
pub use player_bans::*;

mod player_friends;
pub use player_friends::*;

#[cfg(feature = "user_search")]
mod user_search;
#[cfg(feature = "user_search")]
pub use user_search::*;
