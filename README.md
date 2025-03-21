# Web Page Scheduler - Multi

Web Page Scheduler - Multi is a Rust-based application that allows you to schedule web pages to open at specific times. This can be useful for automating tasks, reminders, or simply ensuring you visit certain websites at regular intervals.

## Features

- **Schedule Web Pages**: Enter a time and URL to schedule a web page to open at that specific time.
- **Quick Time Add**: Quickly add 1 or 5 minutes to the current time for scheduling.
- **View Current Time**: Displays the current time in HH:MM:SS format.
- **Manage Schedules**: View and remove scheduled tasks.
- **Automatic URL Normalization**: Automatically adds `https://` to URLs if not provided.

## How to Use

1. **Enter Time**: Input the time in HH:MM 24-hour format.
2. **Enter URL**: Input the URL of the web page you want to schedule.
3. **Schedule**: Click the "Schedule" button to add the task.
4. **Quick Add**: Use the "+1 min" or "+5 min" buttons to quickly set the time.
5. **View Schedules**: See the list of scheduled tasks and their times.
6. **Remove Schedules**: Click "Cancel" next to a scheduled task to remove it.

## Running the Application

To run the application, use the following command:

```sh
cargo run --release
```

## Dependencies

- **iced**: For the graphical user interface.
- **chrono**: For date and time handling.
- **webbrowser**: To open URLs in the default web browser.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

