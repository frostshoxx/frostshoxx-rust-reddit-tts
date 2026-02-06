mod reddit_service;

use reddit_service::RedditService;
use iced::{
    Application, Command, Element, Length, Settings, Subscription, Theme,
    widget::{button, column, container, image, text},
    time, ContentFit, keyboard, window,
};
use std::time::Duration;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct ThreadData {
    pub title: String,
    pub thumbnail: String,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    FetchCompleted,
    TogglePause,
    Close,
}

enum State {
    Splash { elapsed: f32 },
    Running,
    Finished,
}

struct App {
    state: State,
    token: Option<CancellationToken>,
    paused_tx: watch::Sender<bool>,
    is_paused: bool,
    threads_tx: watch::Sender<Vec<ThreadData>>,
    threads_rx: watch::Receiver<Vec<ThreadData>>,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let (paused_tx, _) = watch::channel(false);
        let (threads_tx, threads_rx) = watch::channel(vec![]);
        (
            App {
                state: State::Splash { elapsed: 0.0 },
                token: None,
                paused_tx,
                is_paused: false,
                threads_tx,
                threads_rx,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Reddit Top 10 Threads Reader")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if let State::Splash { elapsed } = &mut self.state {
                    *elapsed += 0.1;
                    if *elapsed > 2.0 {
                        self.state = State::Running;
                        let token = CancellationToken::new();
                        self.token = Some(token.clone());
                        let paused_rx = self.paused_tx.subscribe();
                        let threads_tx = self.threads_tx.clone();
                        return Command::perform(run_fetch(token, paused_rx, threads_tx), |_| Message::FetchCompleted);
                    }
                }
            }
            Message::FetchCompleted => {
                self.state = State::Finished;
                self.token = None;
            }
            Message::TogglePause => {
                self.is_paused = !self.is_paused;
                let _ = self.paused_tx.send(self.is_paused);
            }
            Message::Close => {
                if let Some(token) = &self.token {
                    token.cancel();
                }
                self.token = None;
                return window::close(iced::window::Id::MAIN);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.state {
            State::Splash { .. } => {
                let img = image("assets/splash.png")
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
                let threads = self.threads_rx.borrow().clone();
                let mut content_column = vec![
                    text("Running Reddit TTS...").size(24).into(),
                ];
                
                for thread in threads {
                    let title_element = text(&thread.title).size(14).into();
                    let thread_element = if !thread.thumbnail.is_empty() 
                        && thread.thumbnail != "self" 
                        && thread.thumbnail != "default" 
                        && std::fs::metadata(&thread.thumbnail).is_ok() {
                        column(vec![
                            image(&thread.thumbnail)
                                .width(Length::Fixed(80.0))
                                .height(Length::Fixed(80.0))
                                .content_fit(ContentFit::Cover)
                                .into(),
                            title_element,
                        ]).spacing(5).into()
                    } else {
                        column(vec![title_element]).into()
                    };
                    content_column.push(thread_element);
                }
                
                content_column.push(
                    button(if self.is_paused { "Resume" } else { "Pause" })
                        .on_press(Message::TogglePause)
                        .into()
                );
                
                container(
                    column(content_column)
                        .spacing(10)
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
            State::Running => keyboard::on_key_press(|key, _| {
                if key == keyboard::Key::Named(keyboard::key::Named::Escape) {
                    Some(Message::Close)
                } else {
                    None
                }
            }),
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn run_fetch(token: CancellationToken, paused_rx: watch::Receiver<bool>, threads_tx: watch::Sender<Vec<ThreadData>>) {
    let handle = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let service = RedditService::new();
            service.fetch_and_speak_top_threads_with_pause(paused_rx, threads_tx).await.unwrap();
        });
    });

    let cancelled = token.cancelled();
    tokio::select! {
        _ = cancelled => {}
        result = handle => {
            let _: Result<(), _> = result;
            result.unwrap();
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
