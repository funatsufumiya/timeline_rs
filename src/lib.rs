pub mod easing;
pub mod xml_to_json;

use easing::{EasingFunction, EasingType};

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Timeline<T> {
    pub tracks: HashMap<String, Track<T>>
}

impl<T> Timeline<T> {
    pub fn new() -> Timeline<T> {
        Timeline {
            tracks: HashMap::new(),
        }
    }

    pub fn new_track<U>(&mut self, name: &str) {
        self.tracks.insert(name.to_string(), Track::default());
    }
    
    pub fn get_track(&self, name: &str) -> Option<&Track<T>> {
        self.tracks.get(name)
    }

    pub fn get_track_mut(&mut self, name: &str) -> Option<&mut Track<T>> {
        self.tracks.get_mut(name)
    }

    pub fn add_track(&mut self, name: &str, track: Track<T>) {
        self.tracks.insert(name.to_string(), track);
    }
}

#[derive(Debug)]
pub struct Track<T> {
   pub keyframes: Vec<Keyframe<T>>,
}

impl<T> Default for Track<T> {
    fn default() -> Self {
        Track {
            keyframes: vec![],
        }
    }
}

impl<T> Track<T> 
where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<f32, Output = T> + Copy + Into<f32> + From<f32> + Into<T> + std::cmp::PartialOrd
{
    pub fn add_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe);

        // sort keyframes by time
        self.keyframes.sort_by(|a, b| a.time.cmp(&b.time));
    }

    pub fn add_keyframes(&mut self, keyframes: Vec<Keyframe<T>>) {
        for keyframe in keyframes {
            self.add_keyframe(keyframe);
        }
    }

    pub fn get_keyframe(&self, index: usize) -> Option<&Keyframe<T>> {
        self.keyframes.get(index)
    }

    pub fn get_keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe<T>> {
        self.keyframes.get_mut(index)
    }

    pub fn get_value(&self, time: Duration) -> T {
        let mut value = None;
        let mut prev_keyframe = None;
        let mut next_keyframe = None;
        for keyframe in &self.keyframes {
            if keyframe.time == time {
                value = Some(keyframe.value);
                break;
            }
            if keyframe.time < time {
                prev_keyframe = Some(keyframe);
            }
            if keyframe.time > time {
                next_keyframe = Some(keyframe);
                break;
            }
        }
        if value.is_none() {
            if let Some(prev_keyframe) = prev_keyframe {
                if let Some(next_keyframe) = next_keyframe {
                    let duration = (next_keyframe.time - prev_keyframe.time).as_secs_f32();
                    let t = (time - prev_keyframe.time).as_secs_f32() / duration;
                    // value = Some(prev_keyframe.value + (next_keyframe.value - prev_keyframe.value) * t);
                    // use easing
                    let v1: f32 = prev_keyframe.value.into();
                    let v2: f32 = next_keyframe.value.into();
                    println!("t, duration: {}, {}", t, duration);
                    println!("v1, v2: {}, {}", v1, v2);
                    println!("easing({}, {}, {}, {}, {:?}, {:?})", t, v1, v2 - v1, duration, prev_keyframe.easing_function, prev_keyframe.easing_type);
                    value = Some(easing::easing(
                        t,
                        v1,
                        v2 - v1,
                        duration,
                        prev_keyframe.easing_function,
                        prev_keyframe.easing_type,
                    ).into());
                }
            }
        }
        value.unwrap()
    }
}

#[derive(Debug, Default)]
pub struct Keyframe<T> {
    pub time: Duration,
    pub value: T,
    pub easing_function: EasingFunction,
    pub easing_type: EasingType,
}

impl<T> Keyframe<T> {
    pub fn new(time: Duration, value: T) -> Keyframe<T> {
        Keyframe {
            time: time,
            value: value,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_relative_eq;
    use assert_float_eq::afe_is_relative_eq;
    use assert_float_eq::afe_relative_error_msg;
    use assert_float_eq::afe_abs;

    use super::*;

    #[test]
    fn new() {
        let tl = Timeline::<f32>::new();
        assert_eq!(tl.tracks.len(), 0);
    }

    #[test]
    fn new_track() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        assert_eq!(tl.tracks.len(), 1);
        assert_eq!(tl.tracks.contains_key("test"), true);
    }

    #[test]
    fn new_keyframe() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        tl.get_track_mut("test").unwrap().add_keyframe(Keyframe::<f32> {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        let t = tl.get_track("test").unwrap();
        assert_eq!(t.keyframes.len(), 1);
        assert_eq!(t.keyframes[0].time, Duration::from_secs(1));
        assert_eq!(t.keyframes[0].value, 0.0);
    }

    #[test]
    fn new_keyframes() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        let t = tl.get_track_mut("test").unwrap();
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(2),
            value: 1.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        assert_eq!(t.keyframes.len(), 2);
        assert_eq!(t.keyframes[0].time, Duration::from_secs(1));
        assert_eq!(t.keyframes[1].time, Duration::from_secs(2));
        assert_eq!(t.keyframes[0].value, 0.0);
        assert_eq!(t.keyframes[1].value, 1.0);
    }

    #[test]
    fn new_keyframes_need_sortd() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        let t = tl.get_track_mut("test").unwrap();
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(2),
            value: 1.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        assert_eq!(t.keyframes.len(), 2);
        assert_eq!(t.keyframes[0].time, Duration::from_secs(1));
        assert_eq!(t.keyframes[1].time, Duration::from_secs(2));
        assert_eq!(t.keyframes[0].value, 0.0);
        assert_eq!(t.keyframes[1].value, 1.0);
    }

    #[test]
    fn get_value_test() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        let t = tl.get_track_mut("test").unwrap();
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(2),
            value: 1.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        assert_eq!(t.get_value(Duration::from_secs(1)), 0.0);
        assert_eq!(t.get_value(Duration::from_secs(2)), 1.0);
        assert_eq!(t.get_value(Duration::from_secs(1) + Duration::from_millis(500)), 0.5);
    }

    #[test]
    fn get_value_easing_sine_test() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        let t = tl.get_track_mut("test").unwrap();
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::In,
        });
        t.add_keyframe(Keyframe {
            time: Duration::from_secs(2),
            value: 1.0,
            easing_function: EasingFunction::Sine,
            easing_type: EasingType::In,
        });
        assert_eq!(t.get_value(Duration::from_secs(1)), 0.0);
        assert_eq!(t.get_value(Duration::from_secs(2)), 1.0);
        assert_float_relative_eq!(t.get_value(Duration::from_secs(1) + Duration::from_millis(500)), 0.29289323, 0.0001);
    }
}