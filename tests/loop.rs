use horizontal_mixer::HorizontalMixer;
use std::{thread, time::Duration};

#[test]
fn looping() {
    let mut horizontal_mixer = HorizontalMixer::new(
        "test_resources/track_1.mp3",
        "test_resources/track_2.mp3",
        Duration::from_secs_f32(0.15),
        true,
    );

    horizontal_mixer.play();
    for _ in 0..16 {
        thread::sleep(Duration::from_secs(1));
        horizontal_mixer.toggle_track();
    }
}