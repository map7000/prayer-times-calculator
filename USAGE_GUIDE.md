# Usage Guide
## API Endpoints
### 1. Health Check
Check if the service is running:
```
curl http://localhost:3000/health
```
Response
```
{
  "status": "healthy",
  "service": "prayer-times-api",
  "version": "0.1.0"
}
```
### 2. Get Available Methods
List all supported calculation methods:
```
curl http://localhost:3000/api/methods
``` 
Response
```
{
  "methods": [
    {
      "name": "isna",
      "display_name": "Islamic Society of North America (ISNA)",
      "description": "Standard method used in North America",
      "params": {"fajr": 15.0, "isha": 15.0, "maghrib": 0.0, "midnight": "Standard"}
    },
    ...
  ],
  "default": "isna",
  "categories": {
    "north_america": ["isna"],
    "europe": ["mwl", "france"],
    "middle_east": ["makkah", "gulf", "kuwait", "qatar", "dubai", "jordan"],
    ...
  }
}
```
### 3. Get Prayer Times (Single Date)
Endpoint: GET /api/prayer-times

Query Parameters:

- lat (required): Latitude (-90 to 90)
- lng (required): Longitude (-180 to 180)
- date (optional): Date in YYYY-MM-DD format (defaults to today)
- method (optional): Calculation method name (defaults to "MWL")
- timezone (optional): Timezone offset in hours (defaults to 0)

Example Request:
```
# Get prayer times for Mecca today
curl "http://localhost:3000/api/prayer-times?lat=21.4225&lng=39.8262&method=makkah&timezone=3"

# Get prayer times for a specific date
curl "http://localhost:3000/api/prayer-times?lat=40.7128&lng=-74.0060&date=2026-06-14&method=isna&timezone=-4"
```
Example Response:
```
{
  "status": "success",
  "data": {
    "imsak": "04:45",
    "fajr": "04:55",
    "sunrise": "06:30",
    "dhuhr": "13:00",
    "asr": "16:45",
    "sunset": "19:30",
    "maghrib": "19:40",
    "isha": "21:00",
    "midnight": "00:30"
  },
  "request": {
    "date": "2026-06-14",
    "coordinates": [21.4225, 39.8262],
    "method": "makkah",
    "timezone": 3.0
  }
}
```
### 4. Get Prayer Times (Date Range)
Endpoint: GET /api/prayer-times/range

Query Parameters:

- lat (required): Latitude
- lng (required): Longitude
- start_date (required): Start date (YYYY-MM-DD)
- end_date (required): End date (YYYY-MM-DD)
- method (optional): Calculation method
- timezone (optional): Timezone offset

Example Request:
```
curl "http://localhost:3000/api/prayer-times/range?lat=51.5074&lng=-0.1278&start_date=2026-06-14&end_date=2026-06-20&method=mwl&timezone=1"
```
Example Response:
```
{
  "status": "success",
  "data": [
    {
      "date": "2026-06-14",
      "fajr": "03:15",
      "sunrise": "04:45",
      "dhuhr": "13:00",
      "asr": "17:30",
      "maghrib": "21:15",
      "isha": "22:45"
    },
    {
      "date": "2026-06-15",
      "fajr": "03:15",
      "sunrise": "04:45",
      "dhuhr": "13:00",
      "asr": "17:30",
      "maghrib": "21:16",
      "isha": "22:46"
    },
    ...
  ],
  "request": {
    "date": "2026-06-14 to 2026-06-20",
    "coordinates": [51.5074, -0.1278],
    "method": "mwl",
    "timezone": 1.0
  }
}
```
## Using as a Rust Library
```
use prayer_times_calculator::{PrayerCalculator, presets, models::*};

fn main() {
    let request = CalculationRequest {
        year: 2026,
        month: 6,
        day: 14,
        lat: 21.4225,      // Mecca
        lng: 39.8262,
        elv: 0.0,
        timezone: 3.0,
        params: presets::makkah(),
        settings: Settings::default(),
        offsets: TimeOffsets::default(),
    };

    match PrayerCalculator::calculate(&request) {
        Ok(times) => {
            println!("Fajr: {}", times.fajr);
            println!("Dhuhr: {}", times.dhuhr);
            println!("Asr: {}", times.asr);
            println!("Maghrib: {}", times.maghrib);
            println!("Isha: {}", times.isha);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

