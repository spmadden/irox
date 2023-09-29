Raymarine / Navionics SDF Format
---------------------------------

The SDF file that's created and stored on the SD card is a SQLite Database.
This information is based on Lighthouse 4.4.80 and has been reverse engineered
entirely by myself by looking at the generated data.

There are three tables that are created:
  * `GlobalProp` - Global properties that apply to all entries.
  * `Tracks` - 
  * `Subtracks` - 

The equivalent sql statements are:

```sqlite
CREATE TABLE GlobalProp
(
    cartogrProj               TEXT,
    verticalUnits             INTEGER,
    zIsDepth                  INTEGER DEFAULT 1,
    localTimezone             INTEGER,
    blbMaxPointCount          INTEGER DEFAULT 3000,
    userId                    TEXT,
    userNotes                 TEXT,
    plotterType               TEXT,
    sonarType                 TEXT,
    sonarOffsetMeters         REAL,
    sonarFrequencyKHz         REAL,
    sonarApertureDeg          REAL,
    trackCustomFieldSchemaXml TEXT
);
CREATE TABLE Tracks
(
    libVersion           TEXT    DEFAULT '010010',
    cartogrProj          TEXT,
    verticalUnits        TEXT    DEFAULT feet,
    zIsDepth             INTEGER DEFAULT 1,
    creationDateUtcStr   TEXT    DEFAULT CURRENT_TIMESTAMP,
    creationDateLocalStr TEXT,
    startTimeUtc         INTEGER,
    endTimeUtc           INTEGER,
    startTimeUtcStr      TEXT,
    endTimeUtcStr        TEXT,
    duration             INTEGER,
    pointCount           INTEGER,
    name                 TEXT,
    notes                TEXT,
    userId               TEXT,
    userNotes            TEXT,
    plotterType          TEXT,
    sonarType            TEXT,
    sonarOffsetMeters    REAL,
    sonarFrequencyKHz    REAL,
    sonarApertureDeg     REAL,
    exported             INTEGER DEFAULT 0,
    exportDate           TEXT,
    blbSchemaXml         TEXT,
    blbCmprFlag          INTEGER,
    blbRowFlag           INTEGER,
    blbMaxPointCount     INTEGER,
    pointRecordBytes     INTEGER,
    depthInfoSwVersion   TEXT    DEFAULT 1.1,
    gpsModel             TEXT,
    producerBrand        TEXT,
    bounds               BLOB
);
CREATE TABLE Subtracks
(
    PK             INTEGER PRIMARY KEY ASC NOT NULL,
    trackId        INTEGER,
    firstPointTime INTEGER,
    lastPointTime  INTEGER,
    data           BLOB,
    blobSize       INTEGER,
    firstPointIdx  INTEGER,
    lastPointIdx   INTEGER,
    bounds         BLOB
);
```

The encoded data within the `BLOB` sections are zlib/deflate encoded binary
data.  The field order is described by the XML in the `blbSchemaXml` field.
To improve the compression ratio for the binary field data, the bytes are
sorted "vertically" by field.  The first byte of every field is printed
(in order), then the 2nd byte, then the 3rd byte, and so on.  This is then
deflated to become the final compressed format.

For example, the following schema:
```xml
<?xml?>
<SdfPointSchema libVersion="010010" schemaVersion="010001">
	<SdfFieldDefinition type="Fixed32" name="x" size="4" offset="0" param="4"/>
	<SdfFieldDefinition type="Fixed32" name="y" size="4" offset="4" param="4"/>
	<SdfFieldDefinition type="Fixed32" name="z" size="4" offset="8" param="10"/>
	<SdfFieldDefinition type="UInt32" name="t" size="4" offset="12"/>
	<SdfFieldDefinition type="Float" name="SOG_kn" size="4" offset="16"/>
	<SdfFieldDefinition type="Float" name="Water_speed_kn" size="4" offset="20"/>
</SdfPointSchema>
```
Might look like:
```
struct blbSchema {
    x: fixed32,
    y: fixed32,
    z: fixed32,
    t: u32,
    sog_kn: f32,
    water_speed_kn: f32
}
```
Three repeated structs will be encoded as:
```
x1[0],x2[0],x3[0],x1[1],x2[1],x3[1],x1[2],x2[2],x3[2],x1[3],x2[3],x3[3],
y1[0],y2[0],y3[0],y1[1],y2[1],y3[1],y1[2],y2[2],y3[2],y1[3],y2[3],y3[3],
z1[0],z2[0],z3[0],z1[1],z2[1],z3[1],z1[2],z2[2],z3[2],z1[3],z2[3],z3[3],
t1[0],t2[0],t3[0],t1[1],t2[1],t3[1],t1[2],t2[2],t3[2],t1[3],t2[3],t3[3],
...
```