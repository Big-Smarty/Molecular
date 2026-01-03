use core::f64;
use iced::advanced::mouse::Click;
use iced::widget::canvas;
use iced::widget::canvas::Text;

use iced::Element;
use iced::Point;
use serde::{Deserialize, Serialize};

use crate::{Message, canvas::Canvas, concept::Concept, shape::Shape, tool::Tool};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Point")]
pub struct PointDef {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct App {
    pub(crate) selected_tool: Tool,
    pub(crate) shape_storage: Vec<Shape>,
    pub(crate) concept_storage: Vec<Concept>,
    #[serde(with = "PointDef")]
    pub(crate) position: iced::Point,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) panning_starting_pos: Option<iced::Point>,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) drawing_starting_pos: Option<iced::Point>,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) drawing_ending_pos: Option<iced::Point>,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) previous_click: Option<Click>,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) current_text_field: Option<Text>,
}

impl App {
    pub fn view(&'_ self) -> Element<'_, Message> {
        canvas(Canvas {
            shapes: &self.shape_storage,
            coords: self.position,
            panning_starting_pos: &self.panning_starting_pos,
            drawing_starting_pos: &self.drawing_starting_pos,
            drawing_ending_pos: &self.drawing_ending_pos,
            selected_tool: &self.selected_tool,
            previous_click: &self.previous_click,
            current_text_field: self.current_text_field.clone(),
        })
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected_tool: Tool::default(),
            shape_storage: (0..10)
                .map(|i| {
                    Shape::Point(kurbo::Point {
                        x: i as f64 * 10.0,
                        y: i as f64 * 10.0,
                    })
                })
                .collect(),
            concept_storage: Vec::new(),
            position: iced::Point::new(0.0, 0.0),
            panning_starting_pos: None,
            drawing_starting_pos: None,
            drawing_ending_pos: None,
            previous_click: None,
            current_text_field: Some({
                let mut text = Text::default();
                text.content = "penis".to_string();
                text
            }),
        }
    }
}
