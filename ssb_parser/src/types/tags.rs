// Imports
use std::convert::TryFrom;
use super::ssb::{Coordinate,Degree,Point2D,Point3D};


// Enums
#[derive(Debug)]
pub enum EventTag {
    Font(String),
    Size(f32),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Strikeout(bool),
    Position(Point3D),
    Alignment(Alignment),
    Margin(Margin),
    WrapStyle(WrapStyle),
    Direction(Direction),
    Space(Space),
    Rotate(Rotate),
    Scale(Scale),
    Translate(Translate),
    Shear(Shear),
    Matrix([Degree;16]),
    Border(Border),
    Join(Join),
    Cap(Cap),
    Texture(String),
    TexFill(TexFill),
    Color(Color),
    BorderColor(Color),
    Alpha(Alpha),
    BorderAlpha(Alpha)

    // TODO

}
#[derive(Debug)]
pub enum Alignment {
    Numpad(Numpad),
    Offset(Point2D)
}
#[derive(Debug, PartialEq)]
pub enum Numpad {
    TopLeft, TopCenter, TopRight,
    MiddleLeft, MiddleCenter, MiddleRight,
    BottomLeft, BottomCenter, BottomRight
}
impl TryFrom<u8> for Numpad {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Numpad::BottomLeft),
            2 => Ok(Numpad::BottomCenter),
            3 => Ok(Numpad::BottomRight),
            4 => Ok(Numpad::MiddleLeft),
            5 => Ok(Numpad::MiddleCenter),
            6 => Ok(Numpad::MiddleRight),
            7 => Ok(Numpad::TopLeft),
            8 => Ok(Numpad::TopCenter),
            9 => Ok(Numpad::TopRight),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub enum Margin {
    All(Coordinate, Coordinate, Coordinate, Coordinate),
    Top(Coordinate),
    Right(Coordinate),
    Bottom(Coordinate),
    Left(Coordinate)
}
#[derive(Debug, PartialEq)]
pub enum WrapStyle {
    Space,
    Character,
    NoWrap
}
impl TryFrom<&str> for WrapStyle {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "space" => Ok(WrapStyle::Space),
            "character" => Ok(WrapStyle::Character),
            "nowrap" => Ok(WrapStyle::NoWrap),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop
}
impl TryFrom<&str> for Direction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ltr" => Ok(Direction::LeftToRight),
            "rtl" => Ok(Direction::RightToLeft),
            "ttb" => Ok(Direction::TopToBottom),
            "btt" => Ok(Direction::BottomToTop),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub enum Space {
    All(Coordinate, Coordinate),
    Horizontal(Coordinate),
    Vertical(Coordinate)
}
#[derive(Debug)]
pub enum Rotate {
    All(Degree, Degree, Degree),
    X(Degree),
    Y(Degree),
    Z(Degree)
}
#[derive(Debug)]
pub enum Scale {
    All(Degree, Degree, Degree),
    X(Degree),
    Y(Degree),
    Z(Degree)
}
#[derive(Debug)]
pub enum Translate {
    All(Coordinate, Coordinate, Coordinate),
    X(Coordinate),
    Y(Coordinate),
    Z(Coordinate)
}
#[derive(Debug)]
pub enum Shear {
    All(Degree, Degree),
    X(Degree),
    Y(Degree)
}
#[derive(Debug)]
pub enum Border {
    All(Coordinate, Coordinate),
    Horizontal(Coordinate),
    Vertical(Coordinate)
}
#[derive(Debug, PartialEq)]
pub enum Join {
    Round,
    Bevel,
    Miter
}
impl TryFrom<&str> for Join {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "round" => Ok(Join::Round),
            "bevel" => Ok(Join::Bevel),
            "miter" => Ok(Join::Miter),
            _ => Err(())
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Cap {
    Round,
    Butt,
    Square
}
impl TryFrom<&str> for Cap {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "round" => Ok(Cap::Round),
            "butt" => Ok(Cap::Butt),
            "square" => Ok(Cap::Square),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub struct TexFill {
    pub x0: Degree,
    pub y0: Degree,
    pub x1: Degree,
    pub y1: Degree,
    pub wrap: TextureWrapping
}
#[derive(Debug, PartialEq)]
pub enum TextureWrapping {
    Pad,
    Clamp,
    Repeat,
    Mirror
}
impl TryFrom<&str> for TextureWrapping {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "pad" => Ok(TextureWrapping::Pad),
            "clamp" => Ok(TextureWrapping::Clamp),
            "repeat" => Ok(TextureWrapping::Repeat),
            "mirror" => Ok(TextureWrapping::Mirror),
            _ => Err(())
        }
    }
}
#[derive(Debug)]
pub enum Color {
    Mono([u8;3]),
    Linear([[u8;3];2]),
    LinearWithStop([[u8;3];3]),
    Corners([[u8;3];4]),
    CornersWithStop([[u8;3];5])
}
#[derive(Debug)]
pub enum Alpha {
    Mono(u8),
    Linear([u8;2]),
    LinearWithStop([u8;3]),
    Corners([u8;4]),
    CornersWithStop([u8;5])
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        use super::{Numpad, WrapStyle, Direction, Join, Cap, TextureWrapping, TryFrom};
        assert_eq!(Numpad::try_from(7u8).expect("Numpad expected!"), Numpad::TopLeft);
        assert_eq!(WrapStyle::try_from("character").expect("Wrap style expected!"), WrapStyle::Character);
        assert_eq!(Direction::try_from("ttb").expect("Direction expected!"), Direction::TopToBottom);
        assert_eq!(Join::try_from("bevel").expect("Join expected!"), Join::Bevel);
        assert_eq!(Cap::try_from("butt").expect("Cap expected!"), Cap::Butt);
        assert_eq!(TextureWrapping::try_from("mirror").expect("TextureWrapping expected!"), TextureWrapping::Mirror);
    }
}