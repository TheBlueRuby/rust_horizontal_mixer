# Rust Horizontal Mixer

Simple horizontal mixer in Rust using Kira for audio playback.

## Examples

Mixing between track_1.mp3 and track_2.mp3 with a fade time of 0.15s every 2 seconds, looping

```rust
use horizontal_mixer::HorizontalMixer;
use std::{thread, time::Duration};

fn main() {
    let mut horizontal_mixer = HorizontalMixer::new(
        "track_1.mp3",
        "track_2.mp3",
        Duration::from_secs_f32(0.15),
        true,
    );

    horizontal_mixer.play();
    for _ in 0..16 {
        thread::sleep(Duration::from_secs(2));
        horizontal_mixer.toggle_track();
    }
}
```

## License

This project is licensed under the MIT license.  
Kira, the library this is based on, is licensed under either:

- Apache License, Version 2.0
- MIT License
