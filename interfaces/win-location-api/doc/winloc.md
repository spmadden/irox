


```
Geoposition {
  CivicAddress => {// unsupported}
  VenueData => {// id & level}
  Coordinate => Geocoordinate {
    Accuracy : f64 // meters 
    *Altitude : f64 // meters, see point for reference.
    AltitudeAccuracy : f64 // meters
    Heading : f64 // degrees true north 
    *Latitude : f64  // degrees, see point for reference.
    *Longitude : f64 // degrees, see point for reference.
    Point => Geopoint {
        AltitudeReferenceSystem : i32 enum {
            0 => Unspecified, 
            1 => Terrain, // AGL
            2 => Ellipsoid,  // Ellipsoid defined via EPSG
            3 => Geoid, // MSL
            4 => Surface, // Distance above tallest surface structures, buildings or trees AGL.  This is "Minimum safe aircraft distance".
        }
        GeoshapeType : i32 enum {
            0 => Geopoint, // single point
            1 => Geocircle, // circle, center & radius
            2 => Geopath, // Ordered series of pts
            3 => GeoboundingBox
        }
        Position => BasicGeoposition {
            Altitude : f64, // meters
            Latitude : f64, // degrees [-90.0, +90.0]
            Longitude : f64, // degrees [-180.0, 180.0]
        }
        SpatialReferenceId : u32 // EPSG code
    }
    PositionSource => u32 enum {
        0 => Cellular,
        1 => Satellite,
        2 => WiFi,
        3 => IPAddress,
        4 => Unknown,
        5 => Default,
        6 => Obfuscated,
    } 
    PositionSourceTimestamp : i64 // UTC seconds?
    SatelliteData => GeocoordinateSatelliteData {
        GeometricDilutionOfPrecision : f64,
        HorizontalDilutionOfPrecision : f64, // HDOP
        PositionDilutionOfPrecision: f64, // PDOP
        TimeDilutionOfPrecision : f64, 
        VerticalDilutionOfPrecision : f64 // VDOP
    }
    Speed : f64 // meters per second
    Timestamp : i64 // UTC seconds?
  }
   
}

```