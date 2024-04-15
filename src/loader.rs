use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::{xml_to_json, Timeline, TimelineTrack, TimelineTrackImpl, Track, TrackVariant};
use crate::Keyframe;

pub trait TimelineXMLLoader {
    fn load_xml<T>(&mut self, track_name: &str, xml_path: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>;
    fn load_xml_str<T>(&mut self, track_name: &str, xml: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>;
}
pub trait TimelineJsonLoader {
    fn load_json<T>(&mut self, track_name: &str, json_path: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>;
    fn load_json_str<T>(&mut self, track_name: &str, json: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>;
}

pub trait XMLTrackLoader<T>
where
    TrackVariant: From<Track<T>>,
    T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
{
    fn load_xml(xml: &str) -> Result<Track<T>>;
    fn load_xml_str(xml: &str) -> Result<Track<T>>;
}

pub trait JsonTrackLoader<T>
where
    TrackVariant: From<Track<T>>,
    T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
{
    fn load_json(json_path: &str) -> Result<Track<T>>;
    fn load_json_str(json: &str) -> Result<Track<T>>;
}

impl TimelineXMLLoader for Timeline
{
    fn load_xml<T>(&mut self, track_name: &str, xml_path: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
    {
        let json = xml_to_json::xml_str_to_json(xml_path)?;
        self.load_json::<T>(track_name, &json.to_string())
    }

    fn load_xml_str<T>(&mut self, track_name: &str, xml: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
    {
        let json = xml_to_json::xml_str_to_json(xml)?;
        self.load_json_str::<T>(track_name, &json.to_string())
    }
}

impl TimelineJsonLoader for Timeline {
    fn load_json<T>(&mut self, track_name: &str, json_path: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
    {
        let track = <T as JsonLoaderWrapper<T>>::load_json(json_path)?;
        self.add::<Track<T>>(track_name, track.into());
        Ok(())
    }

    fn load_json_str<T>(&mut self, track_name: &str, json: &str) -> Result<()>
    where
        TrackVariant: From<Track<T>>,
        T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
    {
        let track = <T as JsonLoaderWrapper<T>>::load_json_str(json)?;
        self.add::<Track<T>>(track_name, track.into());
        Ok(())
    }
}

impl<T> XMLTrackLoader<T> for Track<T>
where
    TrackVariant: From<Track<T>>,
T: Copy + DeserializeOwned + JsonLoaderWrapper<T>
{
    fn load_xml(xml_path: &str) -> Result<Track<T>>
    {
        let json = xml_to_json::xml_str_to_json(xml_path)?;
        <T as JsonLoaderWrapper<T>>::load_json(&json.to_string())
    }

    fn load_xml_str(xml: &str) -> Result<Track<T>>
    {
        let json = xml_to_json::xml_str_to_json(xml)?;
        <T as JsonLoaderWrapper<T>>::load_json_str(&json.to_string())
    }
}

trait JsonLoaderWrapper<T>
where 
    T: Copy + DeserializeOwned
{
    fn load_json(json_path: &str) -> Result<Track<T>>;
    fn load_json_str(json: &str) -> Result<Track<T>>;
}
        
macro_rules! impl_json_track_loader {
    ($($t:ty),*) => {
        $(
            impl JsonTrackLoader<$t> for Track<$t>
                where
                    TrackVariant: From<Track<$t>>,
            {
                fn load_json(json: &str) -> Result<Track<$t>>
                {
                    let mut track = Track::<$t>::default();
                    let file = File::open(json)?;
                    let json: KeyframesEntity<$t> = serde_json::from_reader(file)?;
                    for keyframe in json.keyframes.get("key").unwrap() {
                        track.add_keyframe(Keyframe {
                            time: timecode_to_duration(&keyframe.time)?,
                            value: keyframe.value,
                            easing_function: keyframe.easefunc.into(),
                            easing_type: keyframe.easetype.into(),
                        });
                    }
                    Ok(track)
                }

                fn load_json_str(json: &str) -> Result<Track<$t>>
                {
                    let mut track = Track::<$t>::default();
                    let json: KeyframesEntity<$t> = serde_json::from_str(json)?;
                    for keyframe in json.keyframes.get("key").unwrap() {
                        track.add_keyframe(Keyframe {
                            time: timecode_to_duration(&keyframe.time)?,
                            value: keyframe.value,
                            easing_function: keyframe.easefunc.into(),
                            easing_type: keyframe.easetype.into(),
                        });
                    }
                    Ok(track)
                }
            }

            impl JsonLoaderWrapper<$t> for $t {
                fn load_json(json: &str) -> Result<Track<$t>> {
                    Track::<Self>::load_json(json)
                }

                fn load_json_str(json: &str) -> Result<Track<$t>> {
                    Track::<Self>::load_json_str(json)
                }
            }
        )*
    };
}

impl_json_track_loader!(f32, f64, i32, i64);

fn timecode_to_duration(timecode: &str) -> Result<Duration> {
    let parts: Vec<&str> = timecode.split(':').collect();
    if parts.len() != 4 {
        return Err(anyhow::anyhow!("Invalid timecode format"));
    }

    let hours: f64 = parts[0].parse()?;
    let minutes: f64 = parts[1].parse()?;
    let seconds: f64 = parts[2].parse()?;
    let milliseconds: f64 = parts[3].parse()?;

    Ok(Duration::from_secs_f64(hours * 3600.0 + minutes * 60.0 + seconds) + Duration::from_millis(milliseconds as u64))
}

#[derive(serde::Deserialize)]
struct KeyframesEntity<T> {
    keyframes: HashMap<String, Vec<KeyframeEntity<T>>>,
}

// #[derive(serde::Deserialize)]
// struct KeyEntity<T> {
//     key: KeyframeEntity<T>,
// }

#[derive(serde::Deserialize)]
struct KeyframeEntity<T> {
    easefunc: u8,
    easetype: u8,
    time: String,
    value: T,
}

#[cfg(test)]
mod tests {
    use crate::TrackGetter;

    use super::*;

    #[test]
    fn xml_load_test() {
        let xml = r#"
<keyframes>
    <key>
        <easefunc>0</easefunc>
        <easetype>0</easetype>
        <time>00:00:00:643</time>
        <value>0.585546851</value>
    </key>
    <key>
        <easefunc>4</easefunc>
        <easetype>2</easetype>
        <time>00:00:00:826</time>
        <value>0.141503930</value>
    </key>
    <key>
        <easefunc>1</easefunc>
        <easetype>1</easetype>
        <time>00:00:01:594</time>
        <value>0.443359375</value>
    </key>
    <key>
        <easefunc>0</easefunc>
        <easetype>2</easetype>
        <time>00:00:02:033</time>
        <value>0.400390625</value>
    </key>
    <key>
        <easefunc>4</easefunc>
        <easetype>0</easetype>
        <time>00:00:02:260</time>
        <value>0.586718738</value>
    </key>
</keyframes>"#;

        let mut tl = Timeline::new();
        tl.load_xml_str::<f32>("test", xml).unwrap();

        let track = tl.get("test").unwrap().as_float_track();
        assert_eq!(track.keyframes.len(), 5);
    }
}