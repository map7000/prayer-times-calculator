// Reference harness: drives the original PrayTimes.js v2.3 (PrayTimes.org)
// from Node and prints Rust-comparable Float times for selected cases.
const fs = require('fs');
const src = fs.readFileSync(__dirname + '/PrayTimes.js', 'utf8') + '\nmodule.exports = PrayTimes;';
const PrayTimes = new Function(src.replace(/module\.exports = PrayTimes;/g, '') + '; return PrayTimes;')();

const cases = [
  // [name, year, month, day, lat, lng, elv, tz, params, settings, offsets]
  ['mecca_makkah', 2026, 6, 14, 21.4225, 39.8262, 0, 3,
    { fajr: 18.5, isha: '90 min', maghrib: '0 min' }, {}, {}],
  ['newyork_isna_hanafi', 2026, 6, 14, 40.7128, -74.006, 0, -4,
    { fajr: 15, isha: 15, asr: 'Hanafi' }, {}, {}],
  ['london_mwl', 2026, 6, 14, 51.5074, -0.1278, 0, 1,
    { fajr: 18, isha: 17 }, {}, {}],
  ['gunzburg_france', 2026, 1, 15, 48.4502, -1.4902, 0, 1,
    { fajr: 12, isha: 12 }, {}, {}],
  ['seoul_france_winter', 2026, 12, 10, 37.5665, 126.978, 0, 9,
    { fajr: 12, isha: 12, asr: '1.5' }, {}, {}],
  ['jakarta_karachi', 2026, 6, 14, -6.2088, 106.8456, 0, 7,
    { fajr: 18, isha: 18 }, {}, {}],
  ['singapore_singapore', 2026, 6, 14, 1.3521, 103.8198, 0, 8,
    { fajr: 20, isha: 18 }, {}, {}],
  ['toronto_egypt_winter', 2026, 12, 21, 43.6532, -79.3832, 0, -5,
    { fajr: 19.5, isha: 17.5 }, {}, {}],
  ['reykjavik_anglebased', 2026, 6, 14, 64.1466, -21.9426, 0, 0,
    { fajr: 18, isha: 17 }, { highLats: 'AngleBased' }, {}],
  ['tehran_jafari', 2026, 6, 14, 35.6892, 51.389, 0, 3.5,
    { fajr: 16, isha: 14, maghrib: 4, midnight: 'Jafari' }, {}, {}],
  ['mecca_offsets', 2026, 6, 14, 21.4225, 39.8262, 0, 3,
    { fajr: 18.5, isha: '90 min', maghrib: '0 min' }, {},
    { imsak: -10, fajr: 10, sunrise: -2, dhuhr: 3, asr: -7, sunset: 5, maghrib: 5, isha: -5, midnight: 2 }],
  ['dubai_elevation', 2026, 6, 14, 25.2048, 55.2708, 200, 4,
    { fajr: 18.2, isha: 18.2 }, {}, {}],
  ['mecca_dhuhr_imsak', 2026, 6, 14, 21.4225, 39.8262, 0, 3,
    { fajr: 18.5, isha: '90 min', maghrib: '0 min', dhuhr: '5 min' }, {}, {}],
  ['berlin_oneseventh', 2026, 6, 1, 52.52, 13.405, 0, 2,
    { fajr: 18, isha: 17 }, { highLats: 'OneSeventh' }, {}],
  ['murmansk_highlat_none', 2026, 3, 20, 68.9585, 33.0827, 0, 3,
    { fajr: 18, isha: 17 }, { highLats: 'None' }, {}],
  ['equinox_nightmiddle', 2026, 3, 20, 21.4225, 39.8262, 0, 3,
    { fajr: 18.5, isha: '90 min', maghrib: '0 min' }, {}, {}],
];

const timeNames = ['imsak', 'fajr', 'sunrise', 'dhuhr', 'asr', 'sunset', 'maghrib', 'isha', 'midnight'];

const out = {};
for (const [name, year, month, day, lat, lng, elv, tz, params, settings, offsets] of cases) {
  const pt = new PrayTimes('MWL');
  const adjusted = { ...params, ...settings };
  pt.adjust(adjusted);
  pt.tune(offsets);
  const times = pt.getTimes([year, month, day], [lat, lng, elv], tz, 0, 'Float');
  out[name] = { date: `${year}-${month}-${day}`, lat, lng, elv, tz, params, settings, offsets, times };
}
fs.writeFileSync(__dirname + '/reference.json', JSON.stringify(out, null, 2));
console.log(Object.keys(out).map(k => {
  const t = out[k].times;
  return k + ':\n  ' + timeNames.map(n => `${n}=${n === 'imsak' || n === 'midnight' ? t[n] : (Math.round(t[n] * 60) / 60).toFixed(3)}`).join(' ');
}).join('\n'));
