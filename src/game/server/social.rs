use super::Server;
use session::game::chunk::SocialState;
use session::game;
use shared::chunk;
use character::CharacterMinimal;

impl Server {
    pub fn update_social(&self, ch: &CharacterMinimal, state: SocialState) {
        for chunk in &self.base.main_chunks {
            let ch = ch.clone();
            chunk::send(chunk, move |chunk| {
                game::chunk::update_social(chunk, ch, state);
            });
        }
    }
}
