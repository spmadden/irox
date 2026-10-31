// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::Error;

pub const ROOT_INSTALL_DIR: &str = "C:\\OSGeo4W";
pub const SETUP_EXE: &str = "osgeo4w-setup.exe";
pub const MIRROR: &str = "https://download.osgeo.org/osgeo4w/v2/";
const DOWNLOAD_DIR: &str = "C:\\OSGeo4W\\Temp";
const EXEC_EXE: &str = "C:\\OSGeo4W\\Temp\\osgeo4w-setup.exe";

static PACKAGES: &[&str] = &[
    "base",
    "boost",
    "brotli",
    "cairo",
    "curl",
    "curl-ca-bundle",
    "exiv2",
    "exiv2-tools",
    "expat",
    "ffmpeg",
    "freetype",
    "freexl",
    "gdal",
    "gdal-ecw",
    "gdal-filegdb",
    "gdal-hana",
    "gdal-hdf5",
    "gdal-kea",
    "gdal-mrsid",
    "gdal-mss",
    "gdal-oracle",
    "gdal-sosi",
    "gdal-310-runtime",
    "geos",
    "gpsbabel",
    "grass",
    "grass8",
    "gs",
    "gsl",
    "imposm",
    "libgeotiff",
    "libpq",
    "libtiff-tools",
    "node",
    "opencl",
    "openssl",
    "openssl-doc",
    "osm2pgsql",
    "osmium",
    "poppler",
    "poppler-tools",
    "proj",
    "proj-data",
    "proj-runtime-data",
    "qgis-full",
    "qgis-grass-plugin",
    "qgis-oracle-provider",
    "qgis-qt6-full",
    "qgis-qt6-grass-plugin",
    "qgis-qt6-oracle-provider",
    "qgis-ltr-full",
    "qgis-ltr-grass-plugin",
    "qgis-ltr-oracle-provider",
    "qgis-ltr-qt6-full",
    "qgis-ltr-qt6-grass-plugin",
    "qgis-ltr-qt6-oracle-provider",
    "saga",
    "saga7",
    "setup",
    "sqlite3",
    "tiledb",
];

static ARGS: &[&str] = &[
    "--quiet-mode",
    "--advanced",
    "--autoaccept",
    "--upgrade-also",
    "--only-site"
];

pub struct SendingStream {
    pub sender: std::sync::mpsc::Sender<Box<[u8]>>,
    pub receiver: Option<std::sync::mpsc::Receiver<Box<[u8]>>>,
}
impl SendingStream {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            sender,
            receiver: Some(receiver),
        }
    }
}
impl MutBits for SendingStream {
    fn write_u8(&mut self, val: u8) -> Result<(), irox::bits::Error> {
        let _ = self.sender.send(vec![val].into_boxed_slice());
        Ok(())
    }

    fn write_all_bytes(&mut self, val: &[u8]) -> Result<(), irox::bits::Error> {
        let boxed = Vec::from(val).into_boxed_slice();
        let _ = self.sender.send(boxed);
        Ok(())
    }
}

macro_rules! log_stream_msg {
    ($level:expr, $streamer:ident, $($arg:tt)*) => {
        let msg = format!($($arg)*);
        log::log!($level, "{msg}");
        $streamer.write_all_bytes(msg.as_bytes())?;
    };
}

pub fn install_packages<T: MutBits + Send + 'static>(mut streamer: BitsWrapper<'static, T>) -> Result<ChildIO<T>, Error> {
    if !std::fs::exists(Path::new(EXEC_EXE))? {
        let req = ehttp::Request::get(format!("{MIRROR}/{SETUP_EXE}"));
        streamer.write_all_bytes(b"Starting download of OSGeo4W setup...")?;
        let resp = ehttp::fetch_blocking(&req);
        if let Ok(resp) = resp {
            if resp.status != 200 {
                return Error::download_error(format!(
                    "Failed to download OSGeo4W setup: {}",
                    resp.status
                ));
            }
            let body = resp.bytes;
            std::fs::create_dir_all(DOWNLOAD_DIR)?;
            let outpath = Path::new(DOWNLOAD_DIR).join(SETUP_EXE);
            std::fs::write(&outpath, body)?;
        }
    }
    let mut args = Vec::from(ARGS);
    let packages = PACKAGES.join(",");
    args.append(&mut vec![
        "--root", ROOT_INSTALL_DIR,
        "--site", MIRROR,
        "--local-package-dir", DOWNLOAD_DIR,
        "--packages", &packages,
    ]);

    exec_passthru_streamed::<_, &str, &str, &str, T>(EXEC_EXE, args.as_ref(), None, [], streamer)
}

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