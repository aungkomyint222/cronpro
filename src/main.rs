#![windows_subsystem = "windows"]
use iced::{
    widget::{Button, Column, Row, Text, TextInput},
    Application, Command, Element, Settings, Subscription, Theme,
    time,
};
use chrono::{DateTime, Local, NaiveTime, Duration as ChronoDuration, Timelike};
use std::time::Duration;
use webbrowser;

#[derive(Default)]
struct SchedulerApp {
    time_input: String,
    url_input: String,
    status: String,
    schedules: Vec<(DateTime<Local>, String)>,
    current_time: String,
}

#[derive(Debug, Clone)]
enum Message {
    TimeInputChanged(String),
    UrlInputChanged(String),
    SchedulePressed,
    Tick,
    RemoveSchedule(usize),
    QuickTimeAdd(i64),
}

impl Application for SchedulerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = SchedulerApp::default();
        app.current_time = Local::now().format("%H:%M:%S").to_string();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Web Page Scheduler - Multi")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TimeInputChanged(val) => {
                self.time_input = val;
            }
            Message::UrlInputChanged(val) => {
                self.url_input = val;
            }
            Message::SchedulePressed => {
                if self.url_input.trim().is_empty() {
                    self.status = "Please enter a valid URL".to_string();
                    return Command::none();
                }
                
                let mut normalized_url = self.url_input.clone();
                if !normalized_url.starts_with("http://") && !normalized_url.starts_with("https://") {
                    normalized_url = format!("https://{}", normalized_url);
                }

                match NaiveTime::parse_from_str(&self.time_input, "%H:%M") {
                    Ok(schedule_time) => {
                        let now = Local::now();
                        let mut target = now.with_hour(schedule_time.hour())
                                           .unwrap()
                                           .with_minute(schedule_time.minute())
                                           .unwrap()
                                           .with_second(0)
                                           .unwrap();
                        
                        if target <= now {
                            target = target + ChronoDuration::days(1);
                        }
                        
                        let seconds_until = target.signed_duration_since(now).num_seconds();
                        if seconds_until <= 0 {
                            self.status = "Time calculation error".to_string();
                            return Command::none();
                        }

                        // Find the correct position to insert the new schedule
                        let new_schedule = (target, normalized_url);
                        let insert_pos = self.schedules.iter()
                            .position(|(time, _)| time > &target)
                            .unwrap_or(self.schedules.len());
                        
                        self.schedules.insert(insert_pos, new_schedule);
                        self.status = format!("Added schedule for {} ({} seconds)", 
                                            target.format("%H:%M"), seconds_until);
                        self.time_input.clear();
                        self.url_input.clear();
                    }
                    Err(e) => {
                        self.status = format!("Invalid time format (HH:MM): {}", e);
                    }
                }
            }
            Message::Tick => {
                self.current_time = Local::now().format("%H:%M:%S").to_string();
                
                let now = Local::now();
                let mut completed = Vec::new();

                for (i, (target, url)) in self.schedules.iter().enumerate() {
                    if now >= *target {
                        if let Err(e) = webbrowser::open(url) {
                            self.status = format!("Failed to open URL {}: {}", url, e);
                        } else {
                            self.status = format!("Opened URL: {}", url);
                        }
                        completed.push(i);
                    }
                }

                for i in completed.into_iter().rev() {
                    self.schedules.remove(i);
                }

                if !self.schedules.is_empty() {
                    let next = &self.schedules[0];
                    let remaining = next.0.signed_duration_since(now).num_seconds();
                    self.status = format!("Next opening in {} seconds", remaining);
                } else if self.status.contains("Opened URL") {
                    self.status = "All schedules completed!".to_string();
                }
            }
            Message::RemoveSchedule(index) => {
                if index < self.schedules.len() {
                    self.schedules.remove(index);
                    self.status = "Schedule removed".to_string();
                }
            }
            Message::QuickTimeAdd(minutes) => {
                let now = Local::now();
                let target_time = now + ChronoDuration::minutes(minutes);
                self.time_input = target_time.format("%H:%M").to_string();
                self.status = format!("Set time to {} minutes from now", minutes);
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        let mut column = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new(format!("Current time: {}", self.current_time)))
            .push(Text::new("Enter time (HH:MM 24-hour format):"))
            .push(
                TextInput::new("e.g., 14:30", &self.time_input)
                    .on_input(Message::TimeInputChanged)
                    .padding(10),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        Button::new(Text::new("+1 min"))
                            .on_press(Message::QuickTimeAdd(1))
                            .padding(5),
                    )
                    .push(
                        Button::new(Text::new("+5 min"))
                            .on_press(Message::QuickTimeAdd(5))
                            .padding(5),
                    ),
            )
            .push(Text::new("Enter URL:"))
            .push(
                TextInput::new("e.g., google.com", &self.url_input)
                    .on_input(Message::UrlInputChanged)
                    .padding(10),
            )
            .push(
                Button::new(Text::new("Schedule"))
                    .on_press(Message::SchedulePressed)
                    .padding(10),
            )
            .push(Text::new(&self.status));

        if !self.schedules.is_empty() {
            column = column.push(Text::new("Scheduled Tasks:"));
            for (i, (time, url)) in self.schedules.iter().enumerate() {
                let row = Row::new()
                    .spacing(10)
                    .push(Text::new(format!("{} - {}", time.format("%H:%M"), url)))
                    .push(
                        Button::new(Text::new("Cancel"))
                            .on_press(Message::RemoveSchedule(i))
                            .padding(5),
                    );
                column = column.push(row);
            }
        }

        column.into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

fn main() -> iced::Result {
    SchedulerApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}