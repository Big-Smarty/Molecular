use iced::advanced::mouse::Click;

use crate::tool;

#[derive(Debug, Clone)]
pub enum Message {
    SelectedTool(tool::Tool),

    // PANNING
    CanvasMoved(iced::Point, iced::Vector),
    StartedPanning(iced::Point),
    StoppedPanning,

    // DRAWING
    StartedDrawing(iced::Point, Click),
    DrawingMoved(iced::Point, iced::Vector),
    StoppedDrawing,

    // TEXT
    StartedTyping(iced::Point),
    Typing(char),
    Escape,
    Backspace,
    Whitespace,

    // MISCELLANEOUS
    DoubleClick(iced::Point),

    // MENU
    Save,
    Load,
}
