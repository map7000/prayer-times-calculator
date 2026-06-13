# Prayer Times Calculator
A high-performance Rust implementation of the Prayer Times Calculator with REST API bindings. Based on PrayTimes.js
 (version 2.3) by Hamid Zarrabi-Zadeh.

## Features

- Accurate Prayer Times: Calculates Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha, Imsak, Sunset, and Midnight times
- Multiple Calculation Methods: Supports 22+ regional and organizational methods (ISNA, MWL, Makkah, Karachi, etc.)
- REST API: Fast, async HTTP API built with Axum
- Date Range Support: Query prayer times for a single date or a range of dates
- Library Usage: Can be used as a Rust crate in other projects
- High Latitude Adjustments: Proper handling of extreme latitudes

## Architecture
```
src/
├── main.rs              # Application entry point & server setup
├── lib.rs               # Library exports
├── calculator.rs        # Core prayer calculation engine
├── models.rs            # Data models (Params, Settings, TimeOffsets)
├── methods.rs           # Calculation method presets (22+ methods)
├── math.rs              # Mathematical utilities
├── formatter.rs         # Time formatting utilities
├── api/
│   ├── routes.rs        # Route definitions
│   ├── handlers/        # Request handlers
│   ├── models/          # Request/Response DTOs
│   └── error.rs         # Error handling
└── services/
    └── prayer_service.rs # Business logic layer
```
 ## Tech Stack

- Language: Rust (Edition 2021)
- Web Framework: Axum 0.7
- Async Runtime: Tokio
- Serialization: Serde + Serde JSON
- Date/Time: Chrono
- Middleware: Tower + Tower-HTTP (CORS, Tracing)
- Logging: Tracing + Tracing Subscriber

