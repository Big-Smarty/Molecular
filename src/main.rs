use std::fs::File;
use std::io::{BufReader, Read, Write};

use iced::widget::canvas::Text;
use iced::widget::{button, column, container, row, space, text};
use iced::{self, Element};
use iced::{Color, Length};

mod app;
mod canvas;
mod concept;
mod message;
mod shape;
mod tool;

use message::Message;
use rfd::FileDialog;
use serde::Serialize;

use crate::shape::Shape;

fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(mut state: &mut app::App, message: Message) {
    match message {
        Message::SelectedTool(t) => {
            println!("Selected tool: {t:?}");
            state.selected_tool = t;
        }
        Message::CanvasMoved(new_point, delta) => {
            state.position += delta;
            state.panning_starting_pos = Some(new_point);
        }
        Message::StartedPanning(point) => state.panning_starting_pos = Some(point),
        Message::StoppedPanning => state.panning_starting_pos = None,
        Message::StartedDrawing(point, click) => {
            state.drawing_starting_pos = Some(point);
            state.previous_click = Some(click);
        }
        Message::DrawingMoved(point, vector) => state.drawing_ending_pos = Some(point + vector),
        Message::StoppedDrawing => {
            if let Some(ending_point) = state.drawing_ending_pos {
                if ending_point.distance(state.drawing_starting_pos.unwrap()) >= 10.0 {
                    state.shape_storage.push(Shape::Line(kurbo::Line {
                        p0: {
                            let point = state.drawing_starting_pos.unwrap();
                            kurbo::Point::new(point.x.into(), point.y.into())
                        },
                        p1: kurbo::Point::new(ending_point.x.into(), ending_point.y.into()),
                    }));
                } else {
                    state.shape_storage.push(Shape::Point(kurbo::Point::new(
                        state.drawing_starting_pos.unwrap().x as f64,
                        state.drawing_starting_pos.unwrap().y as f64,
                    )));
                }
            } else {
                state.shape_storage.push(Shape::Point(kurbo::Point::new(
                    state.drawing_starting_pos.unwrap().x as f64,
                    state.drawing_starting_pos.unwrap().y as f64,
                )));
            }
            state.drawing_starting_pos = None;
            state.drawing_ending_pos = None;
        }
        Message::StartedTyping(point) => {
            state.current_text_field = Some({
                let mut text = Text::default();
                text.position = point;
                text
            });
            state.shape_storage.pop();
            state.selected_tool = tool::Tool::Typing
        }
        Message::DoubleClick(_point) => (),
        Message::Typing(c) => {
            state.current_text_field = Some({
                let mut text = Text::default();
                text.position = state.current_text_field.clone().unwrap().position;
                text.content = state.current_text_field.clone().unwrap().content;
                text.content.push(c);
                text.position -= iced::Vector::new(text.size.0 / 4.0, 0.0);
                text
            })
        }
        Message::Escape => {
            state.selected_tool = tool::Tool::Draw;
            state
                .shape_storage
                .push(Shape::Text(state.current_text_field.clone().unwrap()));
            state.current_text_field = None;
        }
        Message::Backspace => {
            state.current_text_field = Some({
                let mut text = Text::default();
                text.position = state.current_text_field.clone().unwrap().position;
                text.content = state.current_text_field.clone().unwrap().content;
                text.content.pop();
                text
            })
        }
        Message::Whitespace => {
            state.current_text_field = Some({
                let mut text = Text::default();
                text.position = state.current_text_field.clone().unwrap().position;
                text.content = state.current_text_field.clone().unwrap().content;
                text.content.push(' ');
                text
            })
        }
        Message::Save => {
            println!("saving whole state");
            let json = serde_json::to_string_pretty(state).unwrap();
            println!("{json}");
            let mut file = File::create("saved.json").expect("could create file");
            file.write_all(json.as_bytes()).unwrap();
        }
        Message::Load => {
            println!("loading whole state");
            let files = FileDialog::new()
                .add_filter("text", &["json"])
                .set_directory("/")
                .pick_file();

            match files {
                Some(f) => {
                    let serialized = File::open(f).unwrap();
                    let reader = BufReader::new(serialized);
                    let app: app::App = serde_json::from_reader(reader).unwrap();
                    *state = app;
                }
                None => println!("Couldnt find file!"),
            }
        }
    }
}

fn view(state: &app::App) -> Element<'_, Message> {
    // Top Menu
    let top_menu = container(
        row![
            text("Molecular").size(20),
            space(),
            button("save").on_press(Message::Save),
            space(),
            button("load").on_press(Message::Load),
        ]
        .padding(10)
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
        ..Default::default()
    });

    // Sidebar with Hover Listeners
    let sidebar = container(
        column![
            button("P").on_press(Message::SelectedTool(tool::Tool::Pick)),
            button("D").on_press(Message::SelectedTool(tool::Tool::Draw)),
            // Add your tool buttons here
        ]
        .spacing(20)
        .padding(10),
    )
    .width(Length::Fixed(50.0))
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(Color::from_rgb(0.15, 0.15, 0.15).into()),
        ..Default::default()
    });

    let main_content = container(state.view())
        .width(Length::Fill)
        .height(Length::Fill);

    column![top_menu, row![sidebar, main_content]].into()
}
