use anyhow::Result;

use crate::{xml_to_json, Timeline, TimelineTrack, Track, TrackVariant};

pub trait TimelineXMLLoader {
    fn load_xml<T>(&mut self, track_name: &str, xml: &str) -> Result<()>
    where TrackVariant: From<Track<T>>;
}
pub trait TimelineJsonLoader {
    fn load_json<T>(&mut self, track_name: &str, json: &str) -> Result<()>
    where TrackVariant: From<Track<T>>;
}

pub trait XMLTrackLoader<T> {
    fn load_xml(xml: &str) -> Result<()>
    where TrackVariant: From<Track<T>>;
}

pub trait JsonTrackLoader<T> {
    fn load_json(json: &str) -> Result<Track<T>>
    where TrackVariant: From<Track<T>>;
}

impl TimelineXMLLoader for Timeline
{
    fn load_xml<T>(&mut self, track_name: &str, xml: &str) -> Result<()>
    where TrackVariant: From<Track<T>>
    {
        let json = xml_to_json::xml_str_to_json(xml)?;
        self.load_json::<T>(track_name, &json.to_string())
    }
}

impl TimelineJsonLoader for Timeline {
    fn load_json<T>(&mut self, track_name: &str, json: &str) -> Result<()>
    where TrackVariant: From<Track<T>>
    {
        let track = Track::<T>::load_json(json)?;
        self.add::<Track<T>>(track_name, track.into());
        Ok(())
    }
}

impl<T> XMLTrackLoader<T> for Track<T>
{
    fn load_xml(xml: &str) -> Result<()>
    where TrackVariant: From<Track<T>>
    {
        let json = xml_to_json::xml_str_to_json(xml)?;
        Self::load_json(&json.to_string())?;
        Ok(())
    }
}
        

impl<T> JsonTrackLoader<T> for Track<T>
{
    fn load_json(json: &str) -> Result<Track<T>>
    where TrackVariant: From<Track<T>>
    {
        // FIXME: Implement this
        // let track = Track::<T>::default();
        // Ok(track)
        unimplemented!()
    }
}