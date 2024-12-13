// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::tiff::{TiffTagFormat, TiffTagType};
use irox_tools::static_init;
use std::collections::BTreeMap;

pub const NEW_SUBFILE_TYPE: TiffTagType =
    TiffTagType::new("NewSubfileType", 254, TiffTagFormat::Long, 1);
pub const SUBFILE_TYPE: TiffTagType = TiffTagType::new("SubfileType", 255, TiffTagFormat::Short, 1);
pub const IMAGE_WIDTH: TiffTagType = TiffTagType::new("ImageWidth", 256, TiffTagFormat::Long, 1);
pub const IMAGE_LENGTH: TiffTagType = TiffTagType::new("ImageLength", 257, TiffTagFormat::Long, 1);
pub const BITS_PER_SAMPLE: TiffTagType =
    TiffTagType::new("BitsPerSample", 258, TiffTagFormat::Short, 0);
pub const COMPRESSION: TiffTagType = TiffTagType::new("Compression", 259, TiffTagFormat::Short, 1);
pub const PHOTOMETRIC_INTERPRETATION: TiffTagType =
    TiffTagType::new("PhototricInterpretation", 262, TiffTagFormat::Short, 1);
pub const THRESHOLDING: TiffTagType =
    TiffTagType::new("Thresholding", 263, TiffTagFormat::Short, 1);
pub const CELL_WIDTH: TiffTagType = TiffTagType::new("CellWidth", 264, TiffTagFormat::Short, 1);
pub const CELL_HEIGHT: TiffTagType = TiffTagType::new("CellHeight", 265, TiffTagFormat::Short, 1);
pub const FILL_ORDER: TiffTagType = TiffTagType::new("FillOrder", 266, TiffTagFormat::Short, 1);

pub const STRIP_OFFSETS: TiffTagType =
    TiffTagType::new("StripOffsets", 273, TiffTagFormat::Long, 0);
pub const ORIENTATION: TiffTagType = TiffTagType::new("Orientation", 274, TiffTagFormat::Short, 1);
pub const SAMPLES_PER_PIXEL: TiffTagType =
    TiffTagType::new("SamplesPerPixel", 277, TiffTagFormat::Short, 1);
pub const ROWS_PER_STRIP: TiffTagType =
    TiffTagType::new("RowsPerStrip", 278, TiffTagFormat::Long, 1);
pub const STRIP_BYTE_COUNTS: TiffTagType =
    TiffTagType::new("StripByteCounts", 279, TiffTagFormat::Long, 0);
pub const X_RESOLUTION: TiffTagType =
    TiffTagType::new("XResolution", 282, TiffTagFormat::Rational, 1);
pub const Y_RESOLUTION: TiffTagType =
    TiffTagType::new("YResolution", 283, TiffTagFormat::Rational, 1);
pub const PLANAR_CONFIGURATION: TiffTagType =
    TiffTagType::new("PlanarConfiguration", 284, TiffTagFormat::Short, 1);
pub const X_POSITION: TiffTagType = TiffTagType::new("XPositive", 286, TiffTagFormat::Rational, 1);
pub const Y_POSITION: TiffTagType = TiffTagType::new("YPosition", 287, TiffTagFormat::Rational, 1);

pub const RESOLUTION_UNIT: TiffTagType =
    TiffTagType::new("ResolutionUnit", 296, TiffTagFormat::Short, 1);

pub const SOFTWARE: TiffTagType = TiffTagType::new("Software", 305, TiffTagFormat::Ascii, 0);
pub const DATE_TIME: TiffTagType = TiffTagType::new("DateTime", 306, TiffTagFormat::Ascii, 20);

pub const PREDICTOR: TiffTagType = TiffTagType::new("Predictor", 317, TiffTagFormat::Short, 1);
pub const COLOR_MAP: TiffTagType = TiffTagType::new("ColorMap", 320, TiffTagFormat::Short, 0);
pub const TILE_WIDTH: TiffTagType = TiffTagType::new("TileWidth", 322, TiffTagFormat::Long, 1);
pub const TILE_LENGTH: TiffTagType = TiffTagType::new("TileLength", 323, TiffTagFormat::Long, 1);
pub const TILE_OFFSETS: TiffTagType = TiffTagType::new("TileOffsets", 324, TiffTagFormat::Long, 0);
pub const TILE_BYTE_COUNTS: TiffTagType =
    TiffTagType::new("TileByteCounts", 325, TiffTagFormat::Long, 0);

pub const SAMPLE_FORMAT: TiffTagType =
    TiffTagType::new("SampleFormat", 339, TiffTagFormat::Short, 0);

//Geotiff tags below
pub const MODEL_TIEPOINT: TiffTagType =
    TiffTagType::new("ModelTiepoint", 33922, TiffTagFormat::Double, 0);
pub const MODEL_PIXEL_SCALE: TiffTagType =
    TiffTagType::new("ModelPixelScale", 33550, TiffTagFormat::Double, 3);
pub const MODEL_TRANSFORMATION: TiffTagType =
    TiffTagType::new("ModelTransformation", 34264, TiffTagFormat::Double, 16);
pub const GEO_KEY_DIRECTORY: TiffTagType =
    TiffTagType::new("GeoKeyDirectory", 34735, TiffTagFormat::Long, 0);
pub const GEO_DOUBLE_PARAMS: TiffTagType =
    TiffTagType::new("GeoDoubleParams", 34736, TiffTagFormat::Double, 0);
pub const GEO_ASCII_PARAMS: TiffTagType =
    TiffTagType::new("GeoAsciiParams", 34737, TiffTagFormat::Ascii, 0);

pub static KNOWN_TAG_TYPES: &[TiffTagType] = &[
    NEW_SUBFILE_TYPE,
    SUBFILE_TYPE,
    IMAGE_WIDTH,
    IMAGE_LENGTH,
    BITS_PER_SAMPLE,
    COMPRESSION,
    PHOTOMETRIC_INTERPRETATION,
    THRESHOLDING,
    CELL_WIDTH,
    CELL_HEIGHT,
    FILL_ORDER,
    STRIP_OFFSETS,
    ORIENTATION,
    SAMPLES_PER_PIXEL,
    ROWS_PER_STRIP,
    STRIP_BYTE_COUNTS,
    X_RESOLUTION,
    Y_RESOLUTION,
    PLANAR_CONFIGURATION,
    X_POSITION,
    Y_POSITION,
    RESOLUTION_UNIT,
    PREDICTOR,
    SOFTWARE,
    DATE_TIME,
    COLOR_MAP,
    TILE_WIDTH,
    TILE_LENGTH,
    TILE_OFFSETS,
    TILE_BYTE_COUNTS,
    SAMPLE_FORMAT,
    MODEL_TIEPOINT,
    MODEL_PIXEL_SCALE,
    MODEL_TRANSFORMATION,
    GEO_KEY_DIRECTORY,
    GEO_DOUBLE_PARAMS,
    GEO_ASCII_PARAMS,
];

pub const GT_MODEL_TYPE: TiffTagType =
    TiffTagType::new("GTModelType", 1024, TiffTagFormat::Short, 1);
pub const GT_RASTER_TYPE: TiffTagType =
    TiffTagType::new("GTRasterType", 1025, TiffTagFormat::Short, 1);
pub const GT_CITATION: TiffTagType = TiffTagType::new("GTCitation", 1026, TiffTagFormat::Short, 1);
pub const GEOGRAPHIC_TYPE: TiffTagType =
    TiffTagType::new("GeographicType", 2048, TiffTagFormat::Short, 1);
pub const GEOG_CITATION: TiffTagType =
    TiffTagType::new("GeogCitation", 2049, TiffTagFormat::Ascii, 1);
pub const GEOG_GEODETIC_DATUM: TiffTagType =
    TiffTagType::new("GeogGeodeticDatum", 2050, TiffTagFormat::Short, 1);
pub const GEOG_PRIME_MERIDIAN: TiffTagType =
    TiffTagType::new("GeogPrimeMeridian", 2051, TiffTagFormat::Short, 1);
pub const GEOG_PRIME_MERIDIAN_LONG: TiffTagType =
    TiffTagType::new("GeogPrimeMeridianLong", 2061, TiffTagFormat::Double, 1);
pub const GEOG_LINEAR_UNITS: TiffTagType =
    TiffTagType::new("GeogLinearUnits", 2052, TiffTagFormat::Double, 1);
pub const GEOG_LINEAR_UNIT_SIZE: TiffTagType =
    TiffTagType::new("GeogLinearUnitSize", 2053, TiffTagFormat::Double, 1);
pub const GEOG_ANGULAR_UNITS: TiffTagType =
    TiffTagType::new("GeogAngleUnits", 2054, TiffTagFormat::Short, 1);
pub const GEOG_ANGULAR_UNIT_SIZE: TiffTagType =
    TiffTagType::new("GeogAngleUnitSize", 2055, TiffTagFormat::Double, 1);
pub const GEOG_ELLIPSOID: TiffTagType =
    TiffTagType::new("GeogEllipsoid", 2056, TiffTagFormat::Short, 1);
pub const GEOG_SEMI_MAJOR_AXIS: TiffTagType =
    TiffTagType::new("GeogSemiMajorAxis", 2057, TiffTagFormat::Double, 1);
pub const GEOG_SEMI_MINOR_AXIS: TiffTagType =
    TiffTagType::new("GeogSemiMinorAxis", 2058, TiffTagFormat::Double, 1);
pub const GEOG_INV_FLATTENING: TiffTagType =
    TiffTagType::new("GeogInvFlattening", 2059, TiffTagFormat::Double, 1);
pub const GEOG_AZIMUTH_UNITS: TiffTagType =
    TiffTagType::new("GeogAzimAuthUnits", 2060, TiffTagFormat::Short, 1);
pub const PROJECTEDCS_TYPE: TiffTagType =
    TiffTagType::new("ProjectedCSType", 3072, TiffTagFormat::Short, 1);
pub const PROJECTEDCS_CITATION: TiffTagType =
    TiffTagType::new("ProjectedCSCitation", 3073, TiffTagFormat::Ascii, 1);
pub const PROJECTION: TiffTagType = TiffTagType::new("Projection", 3074, TiffTagFormat::Short, 1);
pub const PROJ_COORD_TRANSFORM: TiffTagType =
    TiffTagType::new("ProjectionTransform", 3075, TiffTagFormat::Short, 1);
pub const PROJ_LINEAR_UNITS: TiffTagType =
    TiffTagType::new("ProjectionLinearUnits", 3076, TiffTagFormat::Short, 1);
pub const PROJ_LINEAR_UNITS_SIZE: TiffTagType =
    TiffTagType::new("ProjectionLinearUnitSize", 3077, TiffTagFormat::Double, 1);
pub const PROJ_STDPARALLEL1: TiffTagType =
    TiffTagType::new("ProjectionStdParallel1", 3078, TiffTagFormat::Double, 1);
pub const PROJ_STDPARALLEL2: TiffTagType =
    TiffTagType::new("ProjectionStdParallel2", 3079, TiffTagFormat::Double, 1);
pub const PROJ_NAT_ORIGIN_LONG: TiffTagType = TiffTagType::new(
    "ProjectionNaturalOriginLongitude",
    3080,
    TiffTagFormat::Double,
    1,
);
pub const PROJ_NAT_ORIGIN_LAT: TiffTagType = TiffTagType::new(
    "ProjectionNaturalOriginLatitude",
    3081,
    TiffTagFormat::Double,
    1,
);
pub const PROJ_FALSE_EASTING: TiffTagType =
    TiffTagType::new("ProjectionFalseEasting", 3082, TiffTagFormat::Double, 1);
pub const PROJ_FALSE_NORTHING: TiffTagType =
    TiffTagType::new("ProjectionFalseNorthing", 3083, TiffTagFormat::Double, 1);
pub const PROJ_FALSE_ORIGIN_LONG: TiffTagType =
    TiffTagType::new("ProjectionFalseOriginLon", 3084, TiffTagFormat::Double, 1);
pub const PROJ_FALSE_ORIGIN_LAT: TiffTagType =
    TiffTagType::new("ProjectionFalseOriginLat", 3085, TiffTagFormat::Double, 1);
pub const PROJ_FALSE_ORIGIN_EASTING: TiffTagType = TiffTagType::new(
    "ProjectionFalseOriginEasting",
    3086,
    TiffTagFormat::Double,
    1,
);
pub const PROJ_FALSE_ORIGIN_NORTHING: TiffTagType = TiffTagType::new(
    "ProjectionFalseOriginNorthing",
    3087,
    TiffTagFormat::Double,
    1,
);
pub const PROJ_CENTER_LONG: TiffTagType =
    TiffTagType::new("ProjectionCenterLon", 3088, TiffTagFormat::Double, 1);
pub const PROJ_CENTER_LAT: TiffTagType =
    TiffTagType::new("ProjectionCenterLat", 3089, TiffTagFormat::Double, 1);

pub static GEO_KEY_DIR_TAGS: &[TiffTagType] = &[
    GT_MODEL_TYPE,
    GT_RASTER_TYPE,
    GT_CITATION,
    GEOGRAPHIC_TYPE,
    GEOG_CITATION,
    GEOG_GEODETIC_DATUM,
    GEOG_PRIME_MERIDIAN,
    GEOG_PRIME_MERIDIAN_LONG,
    GEOG_LINEAR_UNITS,
    GEOG_LINEAR_UNIT_SIZE,
    GEOG_ANGULAR_UNITS,
    GEOG_ANGULAR_UNIT_SIZE,
    GEOG_ELLIPSOID,
    GEOG_SEMI_MAJOR_AXIS,
    GEOG_SEMI_MINOR_AXIS,
    GEOG_INV_FLATTENING,
    GEOG_AZIMUTH_UNITS,
    PROJECTEDCS_TYPE,
    PROJECTEDCS_CITATION,
    PROJECTION,
    PROJ_COORD_TRANSFORM,
    PROJ_LINEAR_UNITS,
    PROJ_LINEAR_UNITS_SIZE,
    PROJ_STDPARALLEL1,
    PROJ_STDPARALLEL2,
    PROJ_NAT_ORIGIN_LONG,
    PROJ_NAT_ORIGIN_LAT,
    PROJ_FALSE_EASTING,
    PROJ_FALSE_NORTHING,
    PROJ_FALSE_ORIGIN_LONG,
    PROJ_FALSE_ORIGIN_LAT,
    PROJ_FALSE_ORIGIN_EASTING,
    PROJ_FALSE_ORIGIN_NORTHING,
    PROJ_CENTER_LONG,
    PROJ_CENTER_LAT,
];

static_init!(get_geokey_directory_tags, BTreeMap<u16, TiffTagType>, {
    let mut out = BTreeMap::new();
    for tag in GEO_KEY_DIR_TAGS {
        out.insert(tag.tag_id, *tag);
    }
    out
});
