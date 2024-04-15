pub mod easing;

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

impl<T> Track<T> {
    pub fn add_keyframe(&mut self, keyframe: Keyframe<T>) {
        self.keyframes.push(keyframe);
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
        tl.tracks.get_mut("test").unwrap().add_keyframe(Keyframe::<f32> {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        let t = tl.tracks.get("test").unwrap();
        assert_eq!(t.keyframes.len(), 1);
        assert_eq!(t.keyframes[0].time, Duration::from_secs(1));
        assert_eq!(t.keyframes[0].value, 0.0);
    }

    #[test]
    fn new_keyframes() {
        let mut tl = Timeline::<f32>::new();
        tl.new_track::<f32>("test");
        let t = tl.tracks.get_mut("test").unwrap();
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
}