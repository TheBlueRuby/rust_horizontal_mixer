use std::{error::Error, time::Duration};

use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::{ModulatorMapping, Tween, Value},
    Volume,
};

pub struct HorizontalMixer {
    manager: AudioManager<DefaultBackend>,
    vol_tween: TweenerHandle,
    track_1: StaticSoundData,
    track_2: StaticSoundData,
    current_track: i8,
}

impl HorizontalMixer {
    pub fn new(path_1: &str, path_2: &str, loop_mus: bool) -> Self {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        let vol_tween = manager
            .add_modulator(TweenerBuilder { initial_value: 0.0 })
            .unwrap();

        if loop_mus {
            let track_1 = StaticSoundData::from_file(
                path_1,
                StaticSoundSettings::new()
                    .volume(Value::from_modulator(
                        &vol_tween,
                        ModulatorMapping {
                            input_range: (0.0, 1.0),
                            output_range: (Volume::Amplitude(1.0), Volume::Amplitude(0.0)),
                            ..Default::default()
                        },
                    ))
                    .loop_region(0.0..),
            )
            .unwrap();

            let track_2 = StaticSoundData::from_file(
                path_2,
                StaticSoundSettings::new()
                    .volume(Value::from_modulator(
                        &vol_tween,
                        ModulatorMapping {
                            input_range: (0.0, 1.0),
                            output_range: (Volume::Amplitude(0.0), Volume::Amplitude(1.0)),
                            ..Default::default()
                        },
                    ))
                    .loop_region(0.0..),
            )
            .unwrap();

            return HorizontalMixer {
                manager,
                vol_tween,
                track_1,
                track_2,
                current_track: 0,
            }
        }

        let track_1 = StaticSoundData::from_file(
            path_1,
            StaticSoundSettings::new().volume(Value::from_modulator(
                &vol_tween,
                ModulatorMapping {
                    input_range: (0.0, 1.0),
                    output_range: (Volume::Amplitude(1.0), Volume::Amplitude(0.0)),
                    ..Default::default()
                },
            )),
        )
        .unwrap();

        let track_2 = StaticSoundData::from_file(
            path_2,
            StaticSoundSettings::new().volume(Value::from_modulator(
                &vol_tween,
                ModulatorMapping {
                    input_range: (0.0, 1.0),
                    output_range: (Volume::Amplitude(0.0), Volume::Amplitude(1.0)),
                    ..Default::default()
                },
            )),
        )
        .unwrap();

        HorizontalMixer {
            manager,
            vol_tween,
            track_1,
            track_2,
            current_track: 0,
        }
    }

    pub fn switch_track(&mut self, which_track: i8) -> Result<(), Box<dyn Error>> {
        if which_track < 2 {
            self.current_track = which_track;
        } else {
            self.current_track = 0;
        }

        println!("{}", self.current_track);

        self.vol_tween.set(
            self.current_track as f64,
            Tween {
                duration: Duration::from_secs_f32(0.25),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    pub fn toggle_track(&mut self) {
        self.switch_track(self.current_track + 1).unwrap();
    }

    pub fn play(&mut self) {
        self.manager.play(self.track_1.clone()).unwrap();
        self.manager.play(self.track_2.clone()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn no_loop() {
        let mut horizontal_mixer = HorizontalMixer::new(
            "test_resources/track_1.mp3",
            "test_resources/track_2.mp3",
            false,
        );

        horizontal_mixer.play();
        for _ in 0..2 {
            thread::sleep(time::Duration::from_secs(1));
            horizontal_mixer.toggle_track();
        }
    }

    #[test]
    fn looping() {
        let mut horizontal_mixer = HorizontalMixer::new(
            "test_resources/track_1.mp3",
            "test_resources/track_2.mp3",
            true,
        );

        horizontal_mixer.play();
        for _ in 0..16 {
            thread::sleep(time::Duration::from_secs(1));
            horizontal_mixer.toggle_track();
        }
    }
}
