mod reddit_service;

use reddit_service::RedditService;
use iced::{
    Application, Command, Element, Length, Settings, Subscription, Theme,
    widget::{column, container, image, text},
    time, ContentFit,
};
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    FetchCompleted,
}

enum State {
    Splash { elapsed: f32 },
    Running,
    Finished,
}

struct App {
    state: State,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                state: State::Splash { elapsed: 0.0 },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Reddit TTS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if let State::Splash { elapsed } = &mut self.state {
                    *elapsed += 0.1;
                    if *elapsed > 2.0 {
                        self.state = State::Running;
                        return Command::perform(run_fetch(), |_| Message::FetchCompleted);
                    }
                }
            }
            Message::FetchCompleted => {
                self.state = State::Finished;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.state {
            State::Splash { .. } => {
                let img = image("splash.png")
                    .content_fit(ContentFit::Contain)
                    .width(Length::Fill)
                    .height(Length::Fill);
                container(img)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            State::Running => {
                container(
                    column![
                        text("Running Reddit TTS...").size(24),
                        text("Fetching and speaking top threads.").size(16),
                    ]
                    .spacing(20)
                    .align_items(iced::Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
            State::Finished => {
                container(
                    text("Finished!").size(24)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Splash { .. } => time::every(Duration::from_millis(500)).map(|_| Message::Tick), // Adjust timing
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn run_fetch() {
    let service = RedditService::new();
    if let Err(e) = service.fetch_and_speak_top_threads().await {
        eprintln!("Error: {}", e);
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
