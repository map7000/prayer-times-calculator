# Prayer Times Calculator
A high-performance Rust implementation of the Prayer Times Calculator with REST API bindings. Based on PrayTimes.js
 (version 2.3) by Hamid Zarrabi-Zadeh.

## Features

- Accurate Prayer Times: Calculates Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha, Imsak, Sunset, and Midnight times
- Multiple Calculation Methods: Supports 22+ regional and organizational methods (ISNA, MWL, Makkah, Karachi, etc.)
- Qibla Direction: True-north and magnetic-compass bearings to the Kaaba, distance, and cardinal direction
- Magnetic Declination: NOAA World Magnetic Model (WMM2020/WMM2025, coefficients bundled)
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
├── qibla.rs             # Qibla bearing, distance, cardinal direction
├── magnetic.rs          # Magnetic declination (World Magnetic Model)
├── api/
│   ├── routes.rs        # Route definitions
│   ├── handlers/        # Request handlers
│   ├── models/          # Request/Response DTOs
│   └── error.rs         # Error handling
└── services/
    ├── prayer_service.rs# Business logic layer
    └── qibla_service.rs # Qibla computation from request parameters
```
 ## Tech Stack

- Language: Rust (Edition 2021)
- Web Framework: Axum 0.8
- Async Runtime: Tokio
- Serialization: Serde + Serde JSON
- Date/Time: Chrono
- Middleware: Tower-HTTP (CORS, Tracing)
- Geomagnetism: World Magnetic Model (WMM2020/WMM2025)
- Logging: Tracing + Tracing Subscriber

## REST API

| Endpoint | Description |
| --- | --- |
| `GET /health` | Service health check |
| `GET /api/methods` | Available calculation methods |
| `GET /api/prayer-times` | Prayer times for a date (`lat`, `lng`, `method`, `timezone`, `date`) |
| `GET /api/prayer-times/range` | Prayer times over a date range (JSON body) |
| `GET /api/qibla` | Qibla direction from coordinates |

### `GET /api/qibla`

Query parameters: `latitude` (-90..90, required), `longitude` (-180..180, required),
`altitude_m` (height above the WGS-84 ellipsoid, optional, default 0),
`date` (evaluation date for the magnetic model, `YYYY-MM-DD`, default today;
must fall within the WMM validity window 2020-01-01 .. 2029-12-31).

```http
GET /api/qibla?latitude=41.0082&longitude=28.9784&altitude_m=100&date=2026-10-19
```

```json
{
  "status": "success",
  "data": {
    "true_bearing_degrees": 151.62058989805806,
    "magnetic_bearing_degrees": 145.54186503691304,
    "magnetic_declination_degrees": 6.0787248611450195,
    "true_cardinal_direction": "SSE",
    "distance_km": 2405.068178458808
  },
  "request": {
    "date": "2026-10-19",
    "coordinates": [41.0082, 28.9784]
  }
}
```

`true_bearing_degrees` is the great-circle bearing clockwise from true north;
`magnetic_bearing_degrees` is what a corrected magnetic compass reads
(`true − declination`).


