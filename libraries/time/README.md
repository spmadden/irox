IROX-TIME
==========

A date & time library that aims for ease of use using static compile-time types based on the Proleptic Gregorian Calendar. 

Goals:
  * Provide type-safe, correct, easy conversions between [`Epoch`s](https://en.wikipedia.org/wiki/Epoch)
  * Provide type-safe, correct, easy conversions between the [Unix Epoch](https://en.wikipedia.org/wiki/Unix_time) and 
      the associated [Gregorian Date]() and [Time]()

Non-goals:
  * Handle strange edge cases like:
    * [Old Style Dates](https://en.wikipedia.org/wiki/Old_Style_and_New_Style_dates) and the 
        [various 10-14 days that don't exist.](https://en.wikipedia.org/wiki/List_of_adoption_dates_of_the_Gregorian_calendar_by_country#List)
    * [Year Zero](https://en.wikipedia.org/wiki/Year_zero)
    * [Julian Leap Year Error](https://en.wikipedia.org/wiki/Julian_calendar#Leap_year_error)
  * Handle Time Zones and Daylight Savings Time

Eventually:
  * Leap Seconds and UTC Proper

Module Structure
------------------
* [`time`](src/mod.rs) - Contains the base `Time` struct, describing a standard `Hours/minutes/seconds` framework.
* [`datetime`](src/datetime.rs) - Contains `UTCDateTime` structs, describing a `Date` with a `Time`
* [`epoch`](src/epoch.rs) - Contains `Epoch`, `UnixEpoch`, `GPSEpoch`, and others, providing the datum anchor for timestamps 
    `UnixTimestamp`, `GPSTimestamp`, etc.
* [`format`](src/format/) - Date & Time Formatters & Parsers
  * [`iso8601`](src/format/iso8601.rs) - ISO8601 Date Formats
  * [`rfc3339`](src/format/rfc3339.rs) - RFC3339 Date Formats, a slight variation on ISO8601
* [`gregorian`](src/gregorian.rs) - Contains `Date` and `Month`, that describe a gregorian calendar date.