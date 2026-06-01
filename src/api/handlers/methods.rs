use axum::response::IntoResponse;
use serde_json::json;

pub async fn get_methods() -> impl IntoResponse {
    let methods = serde_json::json!({
        "methods": [
            {
                "name": "isna",
                "display_name": "Islamic Society of North America (ISNA)",
                "description": "Standard method used in North America",
                "params": { "fajr": 15.0, "isha": 15.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "mwl",
                "display_name": "Muslim World League (MWL)",
                "description": "Used in Europe, Far East, and parts of USA",
                "params": { "fajr": 18.0, "isha": 17.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "egypt",
                "display_name": "Egyptian General Authority of Survey",
                "description": "Used in Africa, Syria, Iraq, Lebanon, Malaysia",
                "params": { "fajr": 19.5, "isha": 17.5, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "makkah",
                "display_name": "Umm al-Qura University (Makkah)",
                "description": "Used in Saudi Arabia",
                "params": { "fajr": 18.5, "isha": "90 minutes", "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "karachi",
                "display_name": "University of Islamic Sciences, Karachi",
                "description": "Used in Pakistan, Afghanistan, Bangladesh, India",
                "params": { "fajr": 18.0, "isha": 18.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "tehran",
                "display_name": "Institute of Geophysics, University of Tehran",
                "description": "Shia method used in Iran",
                "params": { "fajr": 17.7, "isha": 14.0, "maghrib": 4.5, "midnight": "Jafari" }
            },
            {
                "name": "jafari",
                "display_name": "Jafari / Shia Ithna-Ashari",
                "description": "Shia method used in Lebanon, Iraq, Bahrain, Kuwait",
                "params": { "fajr": 16.0, "isha": 14.0, "maghrib": 4.0, "midnight": "Jafari" }
            },
            {
                "name": "gulf",
                "display_name": "Gulf Region",
                "description": "Used in UAE, Qatar, Bahrain, Kuwait, Oman",
                "params": { "fajr": 19.5, "isha": "90 minutes", "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "kuwait",
                "display_name": "Kuwait",
                "description": "Official method of Kuwait",
                "params": { "fajr": 18.0, "isha": 17.5, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "qatar",
                "display_name": "Qatar",
                "description": "Official method of Qatar",
                "params": { "fajr": 18.0, "isha": "90 minutes", "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "singapore",
                "display_name": "Singapore",
                "description": "Used in Singapore, Malaysia, Indonesia",
                "params": { "fajr": 20.0, "isha": 18.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "france",
                "display_name": "France (UOIF - 12°)",
                "description": "Union of Islamic Organizations of France",
                "params": { "fajr": 12.0, "isha": 12.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "turkey",
                "display_name": "Turkey (Diyanet)",
                "description": "Presidency of Religious Affairs of Turkey",
                "params": { "fajr": 18.0, "isha": 17.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "russia",
                "display_name": "Russia",
                "description": "Spiritual Administration of Muslims of Russia",
                "params": { "fajr": 16.0, "isha": 15.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "dubai",
                "display_name": "Dubai",
                "description": "General Authority of Islamic Affairs & Endowments (Dubai)",
                "params": { "fajr": 18.2, "isha": 18.2, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "jakim",
                "display_name": "JAKIM (Malaysia)",
                "description": "Jabatan Kemajuan Islam Malaysia",
                "params": { "fajr": 20.0, "isha": 18.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "tunisia",
                "display_name": "Tunisia",
                "description": "Ministry of Religious Affairs of Tunisia",
                "params": { "fajr": 18.0, "isha": 18.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "algeria",
                "display_name": "Algeria",
                "description": "Ministry of Religious Affairs of Algeria",
                "params": { "fajr": 18.0, "isha": 17.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "kemenag",
                "display_name": "KEMENAG (Indonesia)",
                "description": "Ministry of Religious Affairs of Indonesia",
                "params": { "fajr": 20.0, "isha": 18.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "morocco",
                "display_name": "Morocco",
                "description": "Ministry of Habous and Islamic Affairs of Morocco",
                "params": { "fajr": 19.0, "isha": 17.0, "maghrib": 0.0, "midnight": "Standard" }
            },
            {
                "name": "portugal",
                "display_name": "Portugal",
                "description": "Islamic Community of Lisbon (Mesquita Central de Lisboa)",
                "params": { "fajr": 18.0, "isha": "77 minutes", "maghrib": "3 minutes", "midnight": "Standard" }
            },
            {
                "name": "jordan",
                "display_name": "Jordan",
                "description": "General Ifta' Department of Jordan",
                "params": { "fajr": 18.0, "isha": 18.0, "maghrib": "5 minutes", "midnight": "Standard" }
            }
        ],
        "default": "isna",
        "categories": {
            "north_america": ["isna"],
            "europe": ["mwl", "france"],
            "middle_east": ["makkah", "gulf", "kuwait", "qatar", "dubai", "jordan"],
            "asia": ["karachi", "singapore", "jakim", "kemenag"],
            "africa": ["egypt", "tunisia", "algeria", "morocco"],
            "shia": ["tehran", "jafari"],
            "eastern_europe": ["turkey", "russia"],
            "southern_europe": ["portugal"]
        },
        "usage_note": "Use the 'method' query parameter with any of these names (e.g., ?method=isna)"
    });
    
    (StatusCode::OK, Json(methods))
}