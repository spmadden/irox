// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::Error;

pub const ROOT_INSTALL_DIR: &str = "C:\\OSGeo4W";
pub const DOWNLOAD_DIR: &str = "C:\\OSGeo4W\\Temp";
pub const SETUP_EXE: &str = "osgeo4w-setup.exe";
pub const MIRROR: &str = "https://download.osgeo.org/osgeo4w/v2/";


pub fn run_qgis_processing(
    project_path: &str,
    script: &str,
    extraparams: &[&str],
) -> Result<(), Error> {
    let projpath = format!("--PROJECT_PATH={project_path}");
    let mut params = Vec::new();
    params.push("run");
    params.push(script);
    params.push(projpath.as_str());
    params.push("--");
    params.extend(extraparams);
    crate::os::win::proc::exec_passthru_osgeo("qgis_process.exe", &params)?;
    Ok(())
}