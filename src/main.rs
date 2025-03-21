use iced::{
    widget::{Button, Column, Text, TextInput},
    Application, Command, Element, Settings, Subscription, Theme,
    time,
};
use chrono::{DateTime, Local, NaiveTime, Duration as ChronoDuration, Timelike}; // Added Timelike trait
use std::time::Duration;
// Removed unused Instant import
use webbrowser;

#[derive(Default)]
struct SchedulerApp {
    time_input: String,
    url_input: String,
    status: String,
    target_time: Option<DateTime<Local>>,  // Store absolute target time
    scheduled_url: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    TimeInputChanged(String),
    UrlInputChanged(String),
    SchedulePressed,
    Tick,  // Simplified tick that doesn't carry timing info
}

impl Application for SchedulerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (SchedulerApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Web Page Scheduler")
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
                
                // Normalize URL format
                let mut normalized_url = self.url_input.clone();
                if !normalized_url.starts_with("http://") && !normalized_url.starts_with("https://") {
                    normalized_url = format!("https://{}", normalized_url);
                }

                match NaiveTime::parse_from_str(&self.time_input, "%H:%M") {
                    Ok(schedule_time) => {
                        let now = Local::now();
                        
                        // Create a datetime for today with the scheduled time
                        let mut target = now.with_hour(schedule_time.hour())
                                           .unwrap()
                                           .with_minute(schedule_time.minute())
                                           .unwrap()
                                           .with_second(0)
                                           .unwrap();
                        
                        // If the target time is in the past, schedule for tomorrow
                        if target <= now {
                            target = target + ChronoDuration::days(1);
                        }
                        
                        // Calculate seconds until target
                        let duration = target.signed_duration_since(now);
                        let seconds_until = duration.num_seconds();
                        
                        if seconds_until <= 0 {
                            self.status = "Time calculation error".to_string();
                            return Command::none();
                        }

                        self.target_time = Some(target);
                        self.scheduled_url = Some(normalized_url);
                        self.status = format!("Scheduled for {}! Opening in {} seconds", 
                                             target.format("%H:%M"), seconds_until);
                    }
                    Err(e) => {
                        self.status = format!("Invalid time format (HH:MM): {}", e);
                    }
                }
            }
            Message::Tick => {
                if let Some(target) = self.target_time {
                    let now = Local::now();
                    
                    if now >= target {
                        // Time to open the URL
                        if let Some(url) = &self.scheduled_url {
                            if let Err(e) = webbrowser::open(url) {
                                self.status = format!("Failed to open URL: {}", e);
                            } else {
                                self.status = "URL opened successfully!".to_string();
                            }
                        }
                        
                        // Reset scheduling state
                        self.target_time = None;
                        self.scheduled_url = None;
                    } else {
                        // Update countdown display
                        let remaining = target.signed_duration_since(now).num_seconds();
                        self.status = format!("Opening in {} seconds", remaining);
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.target_time.is_some() {
            // Send a tick message exactly once per second
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Enter time (HH:MM 24-hour format):"))
            .push(
                TextInput::new("e.g., 14:30", &self.time_input)
                    .on_input(Message::TimeInputChanged)
                    .padding(10),
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
            .push(Text::new(&self.status))
            .into()
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