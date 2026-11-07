use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Friend {
    pub username: String,
    pub current_server: String,
    pub is_online: bool,
}

pub struct FriendNetwork {
    pub client: Client,
    pub base_url: String,
}

impl FriendNetwork {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://rilex-server.onrender.com/api".to_string(),
        }
    }
}

pub struct FriendGui {
    pub is_open: bool,
    pub add_friend_input: String,
    pub friends_list: Vec<Friend>,
}

impl FriendGui {
    pub fn new() -> Self {
        Self {
            is_open: true,
            add_friend_input: String::new(),
            friends_list: vec![
                Friend { username: "Notch_2026".to_string(), current_server: "play.hypixel.net".to_string(), is_online: true },
            ],
        }
    }
}

lazy_static! {
    pub static ref FRIENDS_GUI: Mutex<FriendGui> = Mutex::new(FriendGui::new());
    pub static ref NETWORK: FriendNetwork = FriendNetwork::new();
}

pub fn on_game_tick() {
    println!("[RILEX FRIENDS] Mixin injected successfully! The mod is working.");
}
