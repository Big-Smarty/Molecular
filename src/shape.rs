use iced::{
    advanced::{graphics::text::cosmic_text::skrifa::raw::types::Offset32, text::Alignment},
    alignment,
    font::{Family, Stretch, Style, Weight},
    widget::{
        canvas::Text,
        text::{LineHeight, Shaping},
    },
};
use kurbo;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "kurbo::Point")]
struct PointDef {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "kurbo::Line")]
struct LineDef {
    #[serde(with = "PointDef")]
    p0: kurbo::Point,
    #[serde(with = "PointDef")]
    p1: kurbo::Point,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "iced::Point")]
struct IcedPointDef {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "iced::Color")]
struct ColorDef {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "iced::Pixels")]
struct PixelsDef(pub f32);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "LineHeight")]
enum LineHeightDef {
    /// A factor of the size of the text.
    Relative(f32),

    /// An absolute height in logical pixels.
    #[serde(with = "PixelsDef")]
    Absolute(iced::Pixels),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Family")]
enum FamilyDef {
    /// The name of a font family of choice.
    #[serde(skip_serializing, skip_deserializing)]
    Name(&'static str),

    /// Serif fonts represent the formal text style for a script.
    Serif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low
    /// contrast and have stroke endings that are plain — without any flaring,
    /// cross stroke, or other ornamentation.
    SansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style, and
    /// the result looks more like handwritten pen or brush writing than printed
    /// letterwork.
    Cursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that contain
    /// decorative or expressive representations of characters.
    Fantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same
    /// fixed width.
    Monospace,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Weight")]
enum WeightDef {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Stretch")]
enum StretchDef {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Style")]
enum StyleDef {
    Normal,
    Italic,
    Oblique,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
//#[serde(bound(deserialize = "'de: 'static"))]
#[serde(remote = "iced::Font")]
struct FontDef {
    /// The [`Family`] of the [`Font`].
    #[serde(skip_serializing, skip_deserializing)]
    pub family: Family,
    /// The [`Weight`] of the [`Font`].
    #[serde(with = "WeightDef")]
    pub weight: Weight,
    /// The [`Stretch`] of the [`Font`].
    #[serde(with = "StretchDef")]
    pub stretch: Stretch,
    /// The [`Style`] of the [`Font`].
    #[serde(with = "StyleDef")]
    pub style: Style,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(remote = "Alignment")]
enum AlignmentDef {
    /// No specific alignment.
    ///
    /// Left-to-right text will be aligned to the left, while
    /// right-to-left text will be aligned to the right.
    Default,
    /// Align text to the left.
    Left,
    /// Center text.
    Center,
    /// Align text to the right.
    Right,
    /// Justify text.
    Justified,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(remote = "alignment::Vertical")]
enum VerticalAlignmentDef {
    /// Align at the start of the axis.
    Top,

    /// Align at the center of the axis.
    Center,

    /// Align at the end of the axis.
    Bottom,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Shaping")]
enum ShapingDef {
    Auto,
    Basic,
    Advanced,
}

fn f32_serializer_special<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f32(if x.is_infinite() || x.is_nan() {
        f32::MAX
    } else {
        *x
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Text")]
struct TextDef {
    /// The contents of the text
    pub content: String,
    /// The position of the text relative to the alignment properties.
    ///
    /// By default, this position will be relative to the top-left corner coordinate meaning that
    /// if the horizontal and vertical alignments are unchanged, this property will tell where the
    /// top-left corner of the text should be placed.
    ///
    /// By changing the horizontal_alignment and vertical_alignment properties, you are are able to
    /// change what part of text is placed at this positions.
    ///
    /// For example, when the horizontal_alignment and vertical_alignment are set to Center, the
    /// center of the text will be placed at the given position NOT the top-left coordinate.
    #[serde(with = "IcedPointDef")]
    pub position: iced::Point,
    /// The maximum horizontal space available for this [`Text`].
    ///
    /// Text will break into new lines when the width is reached.
    #[serde(serialize_with = "f32_serializer_special")]
    pub max_width: f32,
    /// The color of the text
    #[serde(with = "ColorDef")]
    pub color: iced::Color,
    /// The size of the text
    #[serde(with = "PixelsDef")]
    pub size: iced::Pixels,
    /// The line height of the text.
    #[serde(with = "LineHeightDef")]
    pub line_height: LineHeight,
    /// The font of the text
    #[serde(with = "FontDef")]
    pub font: iced::Font,
    /// The horizontal alignment of the text
    #[serde(with = "AlignmentDef")]
    pub align_x: Alignment,
    /// The vertical alignment of the text
    #[serde(with = "VerticalAlignmentDef")]
    pub align_y: alignment::Vertical,
    /// The shaping strategy of the text.
    #[serde(with = "ShapingDef")]
    pub shaping: Shaping,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Shape {
    #[serde(with = "PointDef")]
    Point(kurbo::Point),
    #[serde(with = "LineDef")]
    Line(kurbo::Line),
    #[serde(with = "TextDef")]
    Text(Text),
}
