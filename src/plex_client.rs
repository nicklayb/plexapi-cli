use crate::Player;
use log::{debug, info};
use reqwest;
use reqwest::blocking::Response;

pub struct PlexClient {
    pub player: Player,
}

impl PlexClient {
    fn call_client(&self, path: String) -> Response {
        let client_url = self.client_url(path);
        debug!("Calling {}", client_url);
        let result = reqwest::blocking::get(client_url);
        match result {
            Ok(response) => {
                info!("Done.");
                response
            }
            Err(ref response) => panic!("{:#?}", response),
        }
    }
    fn client_url(&self, path: String) -> String {
        format!("{}:{}{}", self.player.host, self.player.port, path)
    }
    pub fn play(&self) -> () {
        self.call_client("/player/playback/play".to_string());
    }
    pub fn pause(&self) -> () {
        self.call_client("/player/playback/pause".to_string());
    }
    pub fn stop(&self) -> () {
        self.call_client("/player/playback/stop".to_string());
    }
    pub fn previous(&self) -> () {
        self.call_client("/player/playback/skipPrevious".to_string());
    }
    pub fn next(&self) -> () {
        self.call_client("/player/playback/skipNext".to_string());
    }
    pub fn forward(&self) -> () {
        self.call_client("/player/playback/stepForward".to_string());
    }
    pub fn backward(&self) -> () {
        self.call_client("/player/playback/stepBack".to_string());
    }
}
