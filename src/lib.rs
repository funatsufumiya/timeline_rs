pub mod easing;
pub mod loader;
mod xml_to_json;

#[cfg(feature="bevy")]
use bevy::math::{Vec2, Vec3};
#[cfg(feature="bevy")]
use bevy::render::color::Color;
#[cfg(feature="bevy")]
use bevy::render::render_graph::DynEq;
use easing::{EasingFunction, EasingType};
use serde::de::DeserializeOwned;

use std::any::Any;
use std::collections::HashMap;
use std::time::Duration;

type MyVec2 = (f32, f32);
type MyVec3 = (f32, f32, f32);
type MyVec4 = (f32, f32, f32, f32);

#[derive(Debug)]
pub enum TrackVariant {
    BoolTrack(Track<bool>),
    IntTrack(Track<i32>),
    FloatTrack(Track<f32>),
    DoubleTrack(Track<f64>),
    LongTrack(Track<i64>),
    Vec2Track(Track<(f32, f32)>),
    Vec3Track(Track<(f32, f32, f32)>),
    Vec4Track(Track<(f32, f32, f32, f32)>),
}

macro_rules! impl_from_track_variant {
    ($tp:ident, $name:ident) => {
        impl From<Track<$tp>> for TrackVariant {
            fn from(track: Track<$tp>) -> Self {
                TrackVariant::$name(track)
            }
        }
    };
}

impl_from_track_variant!(bool, BoolTrack);
impl_from_track_variant!(f32, FloatTrack);
impl_from_track_variant!(f64, DoubleTrack);
impl_from_track_variant!(i32, IntTrack);
impl_from_track_variant!(i64, LongTrack); 
impl_from_track_variant!(MyVec2, Vec2Track);
impl_from_track_variant!(MyVec3, Vec3Track);
impl_from_track_variant!(MyVec4, Vec4Track);

pub enum TrackValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    Vec2(MyVec2),
    Vec3(MyVec3),
    Vec4(MyVec4),
}

macro_rules! impl_from_track_value {
    ($tp:ident, $name:ident) => {
        impl From<$tp> for TrackValue {
            fn from(value: $tp) -> Self {
                TrackValue::$name(value)
            }
        }

        impl From<TrackValue> for $tp {
            fn from(value: TrackValue) -> Self {
                match value {
                    TrackValue::$name(v) => v,
                    _ => panic!("Invalid conversion"),
                }
            }
        }
    };
}

impl_from_track_value!(bool, Bool);
impl_from_track_value!(i32, Int);
impl_from_track_value!(f32, Float);
impl_from_track_value!(f64, Double);
impl_from_track_value!(i64, Long);
impl_from_track_value!(MyVec2, Vec2);
impl_from_track_value!(MyVec3, Vec3);
impl_from_track_value!(MyVec4, Vec4);

pub trait TrackValueGetter {
    fn get_value(&self, time: Duration) -> TrackValue;
    fn get_duration(&self) -> Duration;
}

impl TrackValueGetter for TrackVariant {
    fn get_value(&self, time: Duration) -> TrackValue {
        match self {
            TrackVariant::BoolTrack(track) => track.get_value(time).into(),
            TrackVariant::IntTrack(track) => track.get_value(time).into(),
            TrackVariant::FloatTrack(track) => track.get_value(time).into(),
            TrackVariant::DoubleTrack(track) => track.get_value(time).into(),
            TrackVariant::LongTrack(track) => track.get_value(time).into(),
            TrackVariant::Vec2Track(track) => track.get_value(time).into(),
            TrackVariant::Vec3Track(track) => track.get_value(time).into(),
            TrackVariant::Vec4Track(track) => track.get_value(time).into(),
        }
         
    }

    fn get_duration(&self) -> Duration {
        match self {
            TrackVariant::BoolTrack(track) => track.get_duration(),
            TrackVariant::IntTrack(track) => track.get_duration(),
            TrackVariant::FloatTrack(track) => track.get_duration(),
            TrackVariant::DoubleTrack(track) => track.get_duration(),
            TrackVariant::LongTrack(track) => track.get_duration(),
            TrackVariant::Vec2Track(track) => track.get_duration(),
            TrackVariant::Vec3Track(track) => track.get_duration(),
            TrackVariant::Vec4Track(track) => track.get_duration(),
        }
    }
}

pub trait TrackGetter {
    fn as_float_track(&self) -> &Track<f32>;
    fn as_int_track(&self) -> &Track<i32>;
    fn as_bool_track(&self) -> &Track<bool>;
    fn as_double_track(&self) -> &Track<f64>;
    fn as_long_track(&self) -> &Track<i64>;
    fn as_vec2_track(&self) -> &Track<MyVec2>;
    fn as_vec3_track(&self) -> &Track<MyVec3>;
    fn as_vec4_track(&self) -> &Track<MyVec4>;

    fn as_float_truck_mut(&mut self) -> &mut Track<f32>;
    fn as_int_track_mut(&mut self) -> &mut Track<i32>;
    fn as_bool_track_mut(&mut self) -> &mut Track<bool>;
    fn as_double_track_mut(&mut self) -> &mut Track<f64>;
    fn as_long_track_mut(&mut self) -> &mut Track<i64>;
    fn as_vec2_track_mut(&mut self) -> &mut Track<MyVec2>;
    fn as_vec3_track_mut(&mut self) -> &mut Track<MyVec3>;
    fn as_vec4_track_mut(&mut self) -> &mut Track<MyVec4>;
}

macro_rules! track_getter_method {
    ($id:ident, $tp:ident, $id2:ident) => {
        fn $id(&self) -> &Track<$tp> {
            match self {
                TrackVariant::$id2(track) => track,
                _ => panic!("Invalid conversion"),
            }
        } 
    };
}

macro_rules! track_getter_mut_method {
    ($id:ident, $tp:ident, $id2:ident) => {
        fn $id(&mut self) -> &mut Track<$tp> {
            match self {
                TrackVariant::$id2(track) => track,
                _ => panic!("Invalid conversion"),
            }
        } 
    };
}

impl TrackGetter for TrackVariant {
    track_getter_method!(as_float_track, f32, FloatTrack);
    track_getter_method!(as_int_track, i32, IntTrack);
    track_getter_method!(as_bool_track, bool, BoolTrack);
    track_getter_method!(as_double_track, f64, DoubleTrack);
    track_getter_method!(as_long_track, i64, LongTrack);
    track_getter_method!(as_vec2_track, MyVec2, Vec2Track);
    track_getter_method!(as_vec3_track, MyVec3, Vec3Track);
    track_getter_method!(as_vec4_track, MyVec4, Vec4Track);

    track_getter_mut_method!(as_float_truck_mut, f32, FloatTrack);
    track_getter_mut_method!(as_int_track_mut, i32, IntTrack);
    track_getter_mut_method!(as_bool_track_mut, bool, BoolTrack);
    track_getter_mut_method!(as_double_track_mut, f64, DoubleTrack);
    track_getter_mut_method!(as_long_track_mut, i64, LongTrack);
    track_getter_mut_method!(as_vec2_track_mut, MyVec2, Vec2Track);
    track_getter_mut_method!(as_vec3_track_mut, MyVec3, Vec3Track);
    track_getter_mut_method!(as_vec4_track_mut, MyVec4, Vec4Track);
}

#[derive(Debug, Default)]
pub struct Timeline {
    pub tracks: HashMap<String, TrackVariant>,
}

impl Timeline
{
    pub fn new() -> Timeline {
        Timeline {
            tracks: HashMap::new(),
        }
    }

    // pub fn new_track<T>(&mut self, name: &str)
    // where T: Into<TrackVariant> + Default
    // {
    //     self.tracks.insert(name.to_string(), T::default().into());
    // }

    pub fn add<T>(&mut self, name: &str, track: T)
    where T: Into<TrackVariant>
    {
        self.tracks.insert(name.to_string(), track.into());
    }
    
    pub fn get<'a, T>(&'a self, name: &str) -> Option<&'a T>
    where &'a T: From<&'a TrackVariant>
    {
        self.tracks.get(name).map(|track| track.into())
    }

    pub fn get_mut<'a, T>(&'a mut self, name: &str) -> Option<&'a mut T>
    where &'a mut T: From<&'a mut TrackVariant>
    {
        self.tracks.get_mut(name).map(|track| track.into())
    }

    pub fn get_value(&self, name: &str, time: Duration) -> TrackValue {
        self.get(name).unwrap().get_value(time)
    }

    // pub fn get_value(&self, name: &str, time: Duration) -> T {
    //     self.get_track(name).unwrap().get_value(time)
    // }

    /// returns max duration of all tracks
    pub fn get_max_duration(&self) -> Duration {
        let mut max_duration = Duration::from_secs(0);
        for (_, track) in &self.tracks {
            let duration = track.get_duration();
            if duration > max_duration {
                max_duration = duration;
            }
        }
        max_duration
    }
}

#[derive(Debug)]
pub struct Track<T>
where T : Copy + DeserializeOwned
{
   pub keyframes: Vec<Keyframe<T>>,
}

impl<T> Default for Track<T>
where T : Copy + DeserializeOwned
{
    fn default() -> Self {
        Track {
            keyframes: vec![],
        }
    }
}

trait InteroperableValue<T> {
    fn get_easing_value(
        time: f32, start_value: T, next_value: T,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> T;
    fn get_self(&self) -> T;
}

impl InteroperableValue<f32> for f32 {
    fn get_easing_value(
        time: f32, start_value: f32, next_value: f32,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> f32
    {
        easing::easing(time, start_value, next_value - start_value, duration, easing_function, easing_type)
    }
    
    fn get_self(&self) -> f32 {
        *self
    }
}

impl InteroperableValue<f64> for f64 {
    fn get_easing_value(
        time: f32, start_value: f64, next_value: f64,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> f64
    {
        easing::easing(time, start_value as f32, (next_value - start_value) as f32, duration, easing_function, easing_type) as f64
    }

    fn get_self(&self) -> f64 {
        *self
    }
}

impl InteroperableValue<i32> for i32 {
    fn get_easing_value(
        time: f32, start_value: i32, next_value: i32,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> i32
    {
        let changing_value = next_value - start_value;
        easing::easing(
            time, start_value as f32, changing_value as f32, duration, easing_function, easing_type
        ) as i32
    }

    fn get_self(&self) -> i32 {
        *self
    }
}

impl InteroperableValue<i64> for i64 {
    fn get_easing_value(
        time: f32, start_value: i64, next_value: i64,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> i64
    {
        let changing_value = next_value - start_value;
        easing::easing(
            time, start_value as f32, changing_value as f32, duration, easing_function, easing_type
        ) as i64
    }

    fn get_self(&self) -> i64 {
        *self
    }
}

impl InteroperableValue<bool> for bool {
    fn get_easing_value(
        time: f32, start_value: bool, next_value: bool,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> bool
    {
        // WORKAROUND:
        start_value
    }

    fn get_self(&self) -> bool {
        *self
    }
}

impl InteroperableValue<(f32, f32)> for (f32, f32) {
    fn get_easing_value(
        time: f32, start_value: (f32, f32), next_value: (f32, f32),
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> (f32, f32)
    {
        (
            easing::easing(time, start_value.0, next_value.0 - start_value.0, duration, easing_function, easing_type),
            easing::easing(time, start_value.1, next_value.1 - start_value.1, duration, easing_function, easing_type),
        )
    }

    fn get_self(&self) -> (f32, f32) {
        *self
    }
}

impl InteroperableValue<(f32, f32, f32)> for (f32, f32, f32) {
    fn get_easing_value(
        time: f32, start_value: (f32, f32, f32), next_value: (f32, f32, f32),
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> (f32, f32, f32)
    {
        (
            easing::easing(time, start_value.0, next_value.0 - start_value.0, duration, easing_function, easing_type),
            easing::easing(time, start_value.1, next_value.1 - start_value.1, duration, easing_function, easing_type),
            easing::easing(time, start_value.2, next_value.2 - start_value.2, duration, easing_function, easing_type),
        )
    }

    fn get_self(&self) -> (f32, f32, f32) {
        *self
    }
}

impl InteroperableValue<(f32, f32, f32, f32)> for (f32, f32, f32, f32) {
    fn get_easing_value(
        time: f32, start_value: (f32, f32, f32, f32), next_value: (f32, f32, f32, f32),
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> (f32, f32, f32, f32)
    {
        (
            easing::easing(time, start_value.0, next_value.0 - start_value.0, duration, easing_function, easing_type),
            easing::easing(time, start_value.1, next_value.1 - start_value.1, duration, easing_function, easing_type),
            easing::easing(time, start_value.2, next_value.2 - start_value.2, duration, easing_function, easing_type),
            easing::easing(time, start_value.3, next_value.3 - start_value.3, duration, easing_function, easing_type),
        )
    }

    fn get_self(&self) -> (f32, f32, f32, f32) {
        *self
    }
}

pub fn get_easing_value<T>(
    time: f32, start_value: T, next_value: T,
    duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> T
where T: InteroperableValue<T>
{
    T::get_easing_value(time, start_value.get_self(), next_value.get_self(), duration, easing_function, easing_type)
}

pub trait TimelineTrackImpl<T>  {
    fn get_easing_value_wrap(
        &self,
        time: f32, start_value: T, next_value: T,
        duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> T;

    fn get_keyframes(&self) -> &Vec<Keyframe<T>>;
    fn get_keyframes_mut(&mut self) -> &mut Vec<Keyframe<T>>;
}

pub trait TimelineTrack<T> {
    fn add_keyframe(&mut self, keyframe: Keyframe<T>) -> &mut Self;
    fn add_keyframes(&mut self, keyframes: Vec<Keyframe<T>>) -> &mut Self;
    fn sort_keyframes(&mut self) -> &mut Self;
    fn get_keyframe(&self, index: usize) -> Option<&Keyframe<T>>;
    fn get_keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe<T>>;
    fn get_duration(&self) -> Duration;
    fn get_value(&self, time: Duration) -> T;
}

impl<U, T> TimelineTrack<T> for U
where
    U: TimelineTrackImpl<T>,
    T: Copy
{
    fn add_keyframe(&mut self, keyframe: Keyframe<T>) -> &mut Self {
        self.get_keyframes_mut().push(keyframe);
        self.sort_keyframes();
        self
    }

    fn add_keyframes(&mut self, keyframes: Vec<Keyframe<T>>) -> &mut Self {
        for keyframe in keyframes {
            self.add_keyframe(keyframe);
        }

        self
    }

    fn sort_keyframes(&mut self) -> &mut Self {
        self.get_keyframes_mut().sort_by(|a, b| a.time.cmp(&b.time));
        self
    }

    fn get_keyframe(&self, index: usize) -> Option<&Keyframe<T>> {
        self.get_keyframes().get(index)
    }

    fn get_keyframe_mut(&mut self, index: usize) -> Option<&mut Keyframe<T>> {
        self.get_keyframes_mut().get_mut(index)
    }

    fn get_duration(&self) -> Duration {
        if self.get_keyframes().len() > 0 {
            self.get_keyframes()[self.get_keyframes().len() - 1].time
        } else {
            Duration::from_secs(0)
        }
    }

    fn get_value(&self, time: Duration) -> T {
        let mut value = None;
        let mut prev_keyframe = None;
        let mut next_keyframe = None;
        let n = self.get_keyframes().len();

        if n == 0 {
            panic!("No keyframes");
        } else if n == 1 {
            return self.get_keyframes()[0].value;
        } else {
            // if before first keyframe time
            if time < self.get_keyframes()[0].time {
                // WORKAROUND: return first keyframe value
                return self.get_keyframes()[0].value;
            }
        }

        for i in 0..n {
            let keyframe = &self.get_keyframes()[i];
            if keyframe.time == time {
                value = Some(keyframe.value);
                break;
            }
            if keyframe.time < time {
                prev_keyframe = Some(keyframe);
                // if has next
                if i + 1 < n {
                    // and in range
                    if self.get_keyframes()[i + 1].time > time {
                        next_keyframe = Some(&self.get_keyframes()[i + 1]);
                        break;
                    }
                } else {
                    next_keyframe = None;
                    break;
                }
            }
        }
        if value.is_none() {

            // if exceed last keyframe time
            if next_keyframe.is_none() {
                // WORKAROUND: return last keyframe value
                value = Some(prev_keyframe.unwrap().value);

            // if before first keyframe time
            } else if prev_keyframe.is_none() {
                unreachable!();
            
            // if normal
            } else if let Some(prev_keyframe) = prev_keyframe {
                if let Some(next_keyframe) = next_keyframe {
                    let duration = (next_keyframe.time - prev_keyframe.time).as_secs_f32();
                    let dt = (time - prev_keyframe.time).as_secs_f32();
                    // value = Some(prev_keyframe.value + (next_keyframe.value - prev_keyframe.value) * t);
                    // use easing
                    let v1 = prev_keyframe.value;
                    let v2 = next_keyframe.value;
                    // println!("dt, duration: {}, {}", dt, duration);
                    // println!("t: {}", time.as_secs_f32());
                    // println!("t0, t1: {}, {}", prev_keyframe.time.as_secs_f32(), next_keyframe.time.as_secs_f32());
                    // println!("v1, v2: {}, {}", v1, v2);
                    // println!("easing({}, {}, {}, {}, {:?}, {:?})", dt, v1, v2 - v1, duration, prev_keyframe.easing_function, prev_keyframe.easing_type);
                    // value = Some(easing::easing(
                    //     dt,
                    //     v1,
                    //     v2 - v1,
                    //     duration,
                    //     prev_keyframe.easing_function,
                    //     prev_keyframe.easing_type,
                    // ).into());

                    value = Some(self.get_easing_value_wrap(
                        dt,
                        v1,
                        v2,
                        duration,
                        prev_keyframe.easing_function,
                        prev_keyframe.easing_type,
                    ));
                    // let v: f32 = value.unwrap().into();
                    // println!("value: {}", v);
                }else{
                    unreachable!();
                }
            }
        }
        value.unwrap()
    }
}

macro_rules! timeline_track_impl {
    ($tp:ident) => {
        impl TimelineTrackImpl<$tp> for Track<$tp> {
            fn get_easing_value_wrap(
                &self,
                time: f32, start_value: $tp, next_value: $tp,
                duration: f32, easing_function: EasingFunction, easing_type: EasingType) -> $tp
            {
                <$tp>::get_easing_value(time, start_value, next_value, duration, easing_function, easing_type)
            }

            fn get_keyframes(&self) -> &Vec<Keyframe<$tp>> {
                &self.keyframes
            }

            fn get_keyframes_mut(&mut self) -> &mut Vec<Keyframe<$tp>> {
                &mut self.keyframes
            }
        }
    };
}

timeline_track_impl!(f32);
timeline_track_impl!(f64);
timeline_track_impl!(i32);
timeline_track_impl!(i64);
timeline_track_impl!(bool);
timeline_track_impl!(MyVec2);
timeline_track_impl!(MyVec3);
timeline_track_impl!(MyVec4);

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
        let tl = Timeline::new();
        assert_eq!(tl.tracks.len(), 0);
    }

    #[test]
    fn new_track() {
        let mut tl = Timeline::new();
        tl.add("test", Track::<f32>::default());
        assert_eq!(tl.tracks.len(), 1);
        assert_eq!(tl.tracks.contains_key("test"), true);
    }

    #[test]
    fn new_keyframe() {
        let mut t = Track::<f32>::default();
        t.add_keyframe(Keyframe::<f32> {
            time: Duration::from_secs(1),
            value: 0.0,
            easing_function: EasingFunction::Linear,
            easing_type: EasingType::In,
        });
        assert_eq!(t.keyframes.len(), 1);
        assert_eq!(t.keyframes[0].time, Duration::from_secs(1));
        assert_eq!(t.keyframes[0].value, 0.0);
    }

    #[test]
    fn new_keyframes() {
        let mut t = Track::<f32>::default();
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
        let mut t = Track::<f32>::default();
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
        let mut t = Track::<f32>::default();
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
    fn get_value_from_timeline_test() {
        let mut tl = Timeline::new();
        let mut t = Track::<f32>::default();
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
        tl.add("test", t);
        let v: f32 = tl.get_value("test", Duration::from_secs(1)).into();
        assert_eq!(v, 0.0);
        let v: f32 = tl.get_value("test", Duration::from_secs(2)).into();
        assert_eq!(v, 1.0);
        let v: f32 = tl.get_value("test", Duration::from_secs(1) + Duration::from_millis(500)).into();
        assert_eq!(v, 0.5);
    }

    #[test]
    fn get_duration_test () {
        let mut t = Track::<f32>::default();
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
        assert_eq!(t.get_duration(), Duration::from_secs(2));
    }

    #[test]
    fn get_value_exceeded_duration_test() {
        let mut t = Track::<f32>::default();
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
        assert_eq!(t.get_value(Duration::from_secs(3)), 1.0);
        assert_eq!(t.get_value(Duration::from_secs(4)), 1.0);
    }

    #[test]
    fn get_value_before_zero_test() {
        let mut t = Track::<f32>::default();
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
        assert_eq!(t.get_value(Duration::from_secs(0)), 0.0);
    }

    #[test]
    fn get_value_easing_sine_test() {
        let mut t = Track::<f32>::default();
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