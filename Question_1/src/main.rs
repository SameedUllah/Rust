pub mod music {
    pub mod folk_music {
        pub fn play(name: String) {
            println!("Playing music {}", name)
        }
    }
}

use crate::music::folk_music::play;
fn main() {
    play("Sheykh-e-Shangar".to_string());
}
