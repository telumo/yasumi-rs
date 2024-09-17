# 🎌 Yasumi - Japanese Holidays for Rust

**Yasumi** is a Rust library inspired by the popular [jpholiday](https://github.com/Lalit73/jpholiday) Python package. It allows you to determine whether a given date is a holiday in Japan and retrieve holiday lists for specific years.

With Yasumi, you get an ergonomic, high-performance Rust library for managing Japanese holidays, while maintaining the familiar interface of jpholiday.

## 🚀 Features

- 🌸 Determine if a date is a Japanese holiday.
- 🎏 Retrieve the name of the holiday on a specific date.
- 📅 Fetch a list of holidays for any given month or year.

## 🔧 Installation

To include Yasumi in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
yasumi = "0.1.0"
```

Then build your project with:

```bash
cargo build
```

## 📖 Usage

Here’s a quick example to get started with Yasumi:

```rust
use yasumi::{is_holiday, holiday_name, year_holidays};

fn main() {
    let date = "2024-01-01";

    if let Some(holiday) = holiday_name(date) {
        println!("{} is a holiday: {}", date, holiday);
    } else {
        println!("{} is not a holiday.", date);
    }

    let holidays = year_holidays(2024);
    for (date, name) in holidays {
        println!("Holiday on {}: {}", date, name);
    }
}
```

### Available Functions

- is_holiday_name<T: DateLike>(date: T) -> Option<String>
Check if the given date is a holiday and get its name, if available.
- holiday_name<T: DateLike>(date: T) -> Option<String>
Get the name of the holiday on the given date, if it’s a holiday.
- is_holiday<T: DateLike>(date: T) -> bool
Check if the given date is a holiday.
- is_no_workday<T: DateLike>(date: T) -> bool
Determine if the given date is a non-working day (including holidays, Saturday and Sunday).
- month_holidays(year: i32, month: u32) -> Vec<(NaiveDate, String)>
Get a list of holidays for a specific month in a given year.
- year_holidays(year: i32) -> Vec<(NaiveDate, String)>
Get a list of all holidays in a given year.
- holidays<T: DateLike>(start_date: T, end_date: T) -> Vec<(NaiveDate, String)>
Get a list of holidays between the specified start and end dates.
- between<T: DateLike>(start_date: T, end_date: T) -> Vec<(NaiveDate, String)>
Same as holidays.

## 💡 Why Rust?

Rust is known for its memory safety, speed, and concurrency support. Yasumi leverages Rust’s strengths to provide a high-performance alternative to jpholiday. You get the reliability of Rust with the simplicity of a familiar API.

## 🛠 Development

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/yasumi.git
cd yasumi
cargo build
```

To run tests:

```bash
cargo test
```

## 🎉 Credits

Yasumi is inspired by the [jpholiday](https://github.com/Lalit73/jpholiday) Python package. Special thanks to the jpholiday community and all contributors who made this project possible.