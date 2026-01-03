use iced::advanced::mouse::Click;
use iced::widget::canvas::{self, Frame, Path, Text};
use iced::{Color, Rectangle, Renderer, Theme};
use iced::{Point, mouse};

use crate::message::Message;
use crate::shape::Shape;
use crate::tool::Tool;

// First, we define the data we need for drawing
#[derive(Debug)]
pub struct Canvas<'a> {
    pub(crate) shapes: &'a Vec<Shape>,
    pub(crate) coords: Point,
    pub(crate) panning_starting_pos: &'a Option<iced::Point>,
    pub(crate) drawing_starting_pos: &'a Option<iced::Point>,
    pub(crate) drawing_ending_pos: &'a Option<iced::Point>,
    pub(crate) selected_tool: &'a Tool,
    pub(crate) previous_click: &'a Option<Click>,
    pub(crate) current_text_field: Option<Text>,
}

// Then, we implement the `Program` trait
impl<'a> canvas::Program<Message> for Canvas<'a> {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();

        frame.translate(iced::Vector::new(
            center.x + self.coords.x,
            center.y + self.coords.y,
        ));

        if let Some(draw_start) = self.drawing_starting_pos {
            if let Some(draw_end) = self.drawing_ending_pos {
                let path = Path::line(*draw_start, *draw_end);
                let stroke = canvas::Stroke {
                    style: canvas::Style::Solid(Color::BLACK),
                    width: 5.0,
                    line_cap: canvas::LineCap::Round,
                    ..Default::default()
                };
                frame.stroke(&path, stroke);
            }
        }

        match &self.current_text_field {
            Some(current_text) => {
                current_text.draw_with(|path, color| frame.fill(&path, color));
            }
            _ => (),
        }

        for shape in self.shapes {
            match shape {
                Shape::Point(point) => {
                    let path = Path::circle(iced::Point::new(point.x as f32, point.y as f32), 5.0);
                    frame.fill(&path, Color::BLACK);
                }
                Shape::Line(line) => {
                    let path = Path::line(
                        iced::Point::new(line.p0.x as f32, line.p0.y as f32),
                        iced::Point::new(line.p1.x as f32, line.p1.y as f32),
                    );
                    let stroke = canvas::Stroke {
                        style: canvas::Style::Solid(Color::BLACK),
                        width: 5.0,
                        line_cap: canvas::LineCap::Round,
                        ..Default::default()
                    };
                    frame.stroke(&path, stroke);
                }
                Shape::Text(text) => text.draw_with(|path, color| frame.fill(&path, color)),
            }
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match self.selected_tool {
            Tool::Pick => {
                // Only process events if the cursor is within the canvas bounds
                let Some(cursor_position) = cursor.position_in(bounds) else {
                    return None;
                };

                match event {
                    canvas::Event::Mouse(mouse_event) => match mouse_event {
                        // 1. Detect Middle Click Press
                        mouse::Event::ButtonPressed(mouse::Button::Middle) => Some(
                            canvas::Action::publish(Message::StartedPanning(cursor_position)),
                        ),

                        // 2. Detect Middle Click Release
                        mouse::Event::ButtonReleased(mouse::Button::Middle) => {
                            Some(canvas::Action::publish(Message::StoppedPanning))
                        }

                        // 3. Handle Movement while pressed
                        mouse::Event::CursorMoved { .. } => {
                            if let Some(start_pos) = self.panning_starting_pos {
                                // Calculate how much the mouse moved
                                let delta = iced::Vector::new(
                                    cursor_position.x - start_pos.x,
                                    cursor_position.y - start_pos.y,
                                );

                                // Update the start position for the next movement event
                                // Return an Action that sends a message to your App
                                Some(canvas::Action::publish(Message::CanvasMoved(
                                    *start_pos + delta,
                                    delta,
                                )))
                            } else {
                                None
                            }
                        }
                        // TODO: find a way to stop panning when cursor exits the canvas
                        _ => None,
                    },
                    _ => None,
                }
            }
            Tool::Draw => {
                // Only process events if the cursor is within the canvas bounds
                let Some(cursor_position) = cursor.position_in(bounds) else {
                    return None;
                };

                let normalized_position = iced::Point::new(
                    (cursor_position.x - bounds.width / 2.0) - self.coords.x,
                    (cursor_position.y - bounds.height / 2.0) - self.coords.y,
                );

                match event {
                    canvas::Event::Mouse(mouse_event) => match mouse_event {
                        // 1. Detect Left Click Press
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            println!("previous click: {:?}", self.previous_click);
                            let new_click = Click::new(
                                normalized_position,
                                mouse::Button::Left,
                                *self.previous_click,
                            );
                            match new_click.kind() {
                                iced::advanced::mouse::click::Kind::Single => {
                                    Some(canvas::Action::publish(Message::StartedDrawing(
                                        normalized_position,
                                        new_click,
                                    )))
                                }
                                iced::advanced::mouse::click::Kind::Double => {
                                    Some(canvas::Action::publish(Message::StartedTyping(
                                        normalized_position,
                                    )))
                                }
                                iced::advanced::mouse::click::Kind::Triple => {
                                    Some(canvas::Action::publish(Message::StartedTyping(
                                        normalized_position,
                                    )))
                                }
                            }
                        }

                        // 2. Detect Left Click Release
                        mouse::Event::ButtonReleased(mouse::Button::Left) => {
                            Some(canvas::Action::publish(Message::StoppedDrawing))
                        }

                        // 3. Handle Movement while pressed
                        mouse::Event::CursorMoved { .. } => {
                            if let Some(start_pos) = self.drawing_starting_pos {
                                // Calculate how much the mouse moved
                                let delta = iced::Vector::new(
                                    normalized_position.x - start_pos.x,
                                    normalized_position.y - start_pos.y,
                                );

                                // Update the start position for the next movement event
                                // Return an Action that sends a message to your App
                                Some(canvas::Action::publish(Message::DrawingMoved(
                                    *start_pos, delta,
                                )))
                            } else {
                                None
                            }
                        }

                        // send text message on

                        // TODO: find a way to stop panning when cursor exits the canvas
                        _ => None,
                    },
                    _ => None,
                }
            }
            Tool::Typing => match event {
                iced::Event::Keyboard(event) => match event {
                    iced::keyboard::Event::KeyPressed {
                        key,
                        modified_key: _,
                        physical_key: _,
                        location: _,
                        modifiers: _,
                        text: _,
                        repeat: _,
                    } => match key {
                        iced::keyboard::Key::Named(nk) => match nk {
                            iced::keyboard::key::Named::Escape => {
                                Some(canvas::Action::publish(Message::Escape))
                            }
                            iced::keyboard::key::Named::Backspace => {
                                Some(canvas::Action::publish(Message::Backspace))
                            }
                            iced::keyboard::key::Named::Space => {
                                Some(canvas::Action::publish(Message::Whitespace))
                            }
                            _ => None,
                        },
                        iced::keyboard::Key::Character(c) => Some(canvas::Action::publish(
                            Message::Typing(c.chars().nth(0).unwrap()),
                        )),
                        iced::keyboard::Key::Unidentified => None,
                    },
                    _ => None,
                },
                _ => None,
            },
        }
    }
}
