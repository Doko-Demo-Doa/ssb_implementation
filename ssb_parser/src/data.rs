// Imports
use std::collections::HashMap;
use std::fmt;


// Sub types
#[derive(Debug, PartialEq, Clone)]
pub enum View {
    Perspective,
    Orthogonal
}
impl View {
    pub fn from_str(str: &str) -> Result<Self,()> {
        match str {
            "perspective" => Ok(View::Perspective),
            "orthogonal" => Ok(View::Orthogonal),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub struct Event {
    pub trigger: EventTrigger,
    pub macro_: Option<String>,
    pub note: Option<String>,
    pub data: String,
    pub data_location: (usize,usize)
}
#[derive(Debug)]
pub struct EventRender {
    pub trigger: EventTrigger,
    pub objects: Vec<EventObject>
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventTrigger {
    Id(String),
    Time((u32,u32))
}
#[derive(Debug)]
pub enum EventObject {
    Geometry(EventGeometry),
    Tag(EventTag)
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontFace {
    pub family: String,
    pub style: FontStyle
}
impl fmt::Display for FontFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.family, self.style)
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic
}
impl FontStyle {
    pub fn from_str(str: &str) -> Result<Self,()> {
        match str {
            "regular" => Ok(FontStyle::Regular),
            "bold" => Ok(FontStyle::Bold),
            "italic" => Ok(FontStyle::Italic),
            "bold-italic" => Ok(FontStyle::BoldItalic),
            _ => Err(())
        }
    }
}
pub type FontData = String;
pub type FontDataRender = Vec<u8>;
pub type TextureId = String;
#[derive(Debug)]
pub enum TextureData {
    Url(String),
    Raw(String)
}
pub type TextureDataRender = Vec<u8>;

// Object types
#[derive(Debug)]
pub enum EventGeometry {
    Shape(Vec<ShapeSegment>),
    Points(Vec<Point2D>),
    Text(String)
}
#[derive(Debug)]
pub enum ShapeSegment {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo(Point2D, Point2D, Point2D),
    ArcBy(Point2D, f64),
    Close
}
#[derive(Debug)]
pub struct Point2D {
    pub x: Coordinate,
    pub y: Coordinate
}
pub type Coordinate = f32;
#[derive(Debug)]
pub enum EventTag {
    Font(String),
    Size(f32),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Strikeout(bool),
    Position(Point3D),
    Alignment(Alignment)

    // TODO

}
#[derive(Debug)]
pub struct Point3D {
    pub x: Coordinate,
    pub y: Coordinate,
    pub z: Coordinate
}
#[derive(Debug)]
pub enum Alignment {
    Numpad(u8),
    Offset(Point2D)
}

// Raw data
#[derive(Debug)]
pub struct Ssb {
    // Info section
    pub info_title: Option<String>,
    pub info_author: Option<String>,
    pub info_description: Option<String>,
    pub info_version: Option<String>,
    pub info_custom: HashMap<String, String>,
    // Target section
    pub target_width: Option<u16>,
    pub target_height: Option<u16>,
    pub target_depth: u16,
    pub target_view: View,
    // Macros section
    pub macros: HashMap<String, String>,
    // Events section
    pub events: Vec<Event>,
    // Resources section
    pub fonts: HashMap<FontFace, FontData>,
    pub textures: HashMap<TextureId, TextureData>
}
impl Default for Ssb {
    fn default() -> Self {
        Self {
            info_title: None,
            info_author: None,
            info_description: None,
            info_version: None,
            info_custom: HashMap::default(),
            target_width: None,
            target_height: None,
            target_depth: 1000,
            target_view: View::Perspective,
            macros: HashMap::default(),
            events: Vec::default(),
            fonts: HashMap::default(),
            textures: HashMap::default()
        }
    }
}

// Processed data
#[derive(Debug)]
pub struct SsbRender {
    // Target section
    pub target_width: Option<u16>,
    pub target_height: Option<u16>,
    pub target_depth: u16,
    pub target_view: View,
    // Events section
    pub events: Vec<EventRender>,
    // Resources section
    pub fonts: HashMap<FontFace, FontDataRender>,
    pub textures: HashMap<TextureId, TextureDataRender>
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{View, FontStyle};
        assert_eq!(View::from_str("orthogonal").expect("View instance expected!"), View::Orthogonal);
        assert_eq!(FontStyle::from_str("bold-italic").expect("FontStyle instance expected!"), FontStyle::BoldItalic);
    }
}