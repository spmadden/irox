// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use std::env::VarError;
use std::process::Command;

pub const OSGEO_ROOT: &str = "c:\\OSGeo4W";
pub const OSGEO_FWD_ROOT: &str = "c:/OSGeo4";

fn get_varbl(cmd: &mut Command, varbl: &str) -> String {
    let varbl = cmd
        .get_envs()
        .find(|(v, _z)| v.to_string_lossy() == varbl)
        .unwrap_or_default()
        .1
        .unwrap_or_default();
    varbl.to_string_lossy().to_string()
}
#[allow(non_snake_case)]
pub fn set_osgeo_envs(cmd: &mut Command) -> Result<(), VarError> {
    let WINDIR = std::env::var("WINDIR")?;
    // start with clean path
    cmd.env(
        "PATH",
        format!("{OSGEO_ROOT}\\bin;{WINDIR}\\system32;{WINDIR}\\WBem"),
    );
    cmd.env("OSGEO_ROOT", OSGEO_ROOT);

    envs_gdal(cmd);
    envs_gs(cmd);
    envs_openssl(cmd);
    envs_pdal(cmd);
    envs_proj(cmd);
    envs_python(cmd);
    envs_qt5(cmd);
    envs_qgis(cmd);
    Ok(())
}
fn envs_gdal(cmd: &mut Command) {
    cmd.env(
        "GDAL_DATA",
        format!("{OSGEO_ROOT}\\apps\\gdap\\share\\gdal"),
    );
    cmd.env(
        "GDAL_DRIVER_PATH",
        format!("{OSGEO_ROOT}\\apps\\gdap\\lib\\gdalplugins"),
    );
}
fn envs_gs(cmd: &mut Command) {
    cmd.env("GS_LIB", format!("{OSGEO_ROOT}\\apps\\gs\\lib"));
}

fn envs_openssl(cmd: &mut Command) {
    cmd.env("OPENSSL_ENGINES", format!("{OSGEO_ROOT}\\lib\\engines-3"));
    cmd.env(
        "SSL_CERT_FILE",
        format!("{OSGEO_ROOT}\\bin\\curl-ca-bundle.crt"),
    );
    cmd.env(
        "SSL_CERT_DIR",
        format!("{OSGEO_ROOT}\\apps\\openssl\\certs"),
    );
}
fn envs_pdal(cmd: &mut Command) {
    cmd.env(
        "PDAL_DRIVER_PATH",
        format!("{OSGEO_ROOT}\\apps\\pdal\\plugins"),
    );
}
fn envs_proj(cmd: &mut Command) {
    cmd.env("PROJ_DATA", format!("{OSGEO_ROOT}\\share\\proj"));
}
#[allow(non_snake_case)]
fn envs_python(cmd: &mut Command) {
    cmd.env("PYTHONHOME", format!("{OSGEO_ROOT}\\apps\\Python312"));
    cmd.env("PYTHONPATH", "");
    cmd.env("PYTHONUTF8", "1");
    let PATH = get_varbl(cmd, "PATH");
    cmd.env(
        "PATH",
        format!("{OSGEO_ROOT}\\apps\\Python312\\Scripts;{PATH}"),
    );
}

#[allow(non_snake_case)]
fn envs_qt5(cmd: &mut Command) {
    let PATH = get_varbl(cmd, "PATH");

    cmd.env("PATH", format!("{OSGEO_ROOT}\\apps\\qt5\\bin;{PATH}"));
    cmd.env(
        "QT_PLUGIN_PATH",
        format!("{OSGEO_ROOT}\\apps\\Qt5\\plugins"),
    );
    cmd.env("O4W_QT_PREFIX", format!("{OSGEO_FWD_ROOT}/apps/Qt5"));
    cmd.env("O4W_QT_BINARIES", format!("{OSGEO_FWD_ROOT}/apps/Qt5/bin"));
    cmd.env(
        "O4W_QT_PLUGINS",
        format!("{OSGEO_FWD_ROOT}/apps/Qt5/plugins"),
    );
    cmd.env("O4W_QT_LIBRARIES", format!("{OSGEO_FWD_ROOT}/apps/Qt5/lib"));
    cmd.env(
        "O4W_QT_TRANSLATIONS",
        format!("{OSGEO_FWD_ROOT}/apps/Qt5/translations"),
    );
    cmd.env(
        "O4W_QT_HEADERS",
        format!("{OSGEO_FWD_ROOT}/apps/Qt5/include"),
    );
    cmd.env("O4W_QT_DOC", format!("{OSGEO_FWD_ROOT}/apps/Qt5/doc"));
}
#[allow(non_snake_case)]
fn envs_qgis(cmd: &mut Command) {
    let PATH = get_varbl(cmd, "PATH");
    let PYTHONPATH = get_varbl(cmd, "PYTHONPATH");
    cmd.env("PATH", format!("{OSGEO_ROOT}\\apps\\qgis-ltr\\bin;{PATH}"));
    cmd.env(
        "QGIS_PREFIX_PATH",
        format!("{OSGEO_FWD_ROOT}/apps/qgis-ltr"),
    );
    cmd.env("GDAL_FILENAME_IS_UTF8", "YES");
    cmd.env("VSI_CACHE", "TRUE");
    cmd.env("VSI_CACHE_SIZE", "1000000");
    cmd.env(
        "QT_PLUGIN_PATH",
        format!("{OSGEO_ROOT}\\apps\\qgis-ltr\\qtplugins;{OSGEO_ROOT}\\apps\\qt5\\plugins"),
    );
    cmd.env(
        "PYTHONPATH",
        format!("{OSGEO_ROOT}\\apps\\qgis-ltr\\python;{PYTHONPATH}"),
    );
}
