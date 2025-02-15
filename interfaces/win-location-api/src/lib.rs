// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#[cfg(target_os = "windows")]
pub use crate::windows::*;

#[cfg(target_os = "windows")]
mod windows {
    use log::{error, info, trace, warn};
    use windows::Devices::Geolocation::{
        GeolocationAccessStatus, Geolocator, PositionChangedEventArgs, StatusChangedEventArgs,
    };
    use windows::Foundation::TypedEventHandler;
    type EventRegistrationToken = i64;

    pub use crate::data::*;
    pub use crate::error::*;

    pub struct WindowsLocationAPI {
        locator: Geolocator,
    }

    impl WindowsLocationAPI {
        pub fn connect() -> Result<WindowsLocationAPI, Error> {
            let locator = Geolocator::new()?;
            trace!("Geolocator created.");

            if let Err(e) = locator.AllowFallbackToConsentlessPositions() {
                warn!("Error requesting fallback to consentless positions: {e:?}")
            }

            let access = Geolocator::RequestAccessAsync()?;
            let access_results = access.get()?;
            match access_results {
                GeolocationAccessStatus::Allowed => {
                    info!("User granted geolocation access.");
                }
                GeolocationAccessStatus::Denied => {
                    warn!("User denied geolocation access.");
                }
                _ => {
                    warn!("Unknown result from geolocation access request.");
                }
            }

            Ok(WindowsLocationAPI { locator })
        }

        pub fn get_location(&self) -> Result<WindowsCoordinate, Error> {
            let res = self.locator.GetGeopositionAsync()?;
            let res = res.get()?;
            let res = &(res.Coordinate()?);
            let out: WindowsCoordinate = res.into();
            trace!("WindowsLocation: {out:?}");
            Ok(out)
        }

        ///
        /// Returns the current status of the connection
        pub fn get_status(&self) -> Result<PositionStatus, Error> {
            Ok(self.locator.LocationStatus()?.0.into())
        }

        ///
        /// Registers a callback handler to receive updates when the location changes.
        /// When the [`LocationHandler`] object gets dropped or goes out of scope, the callback
        /// function is deregistered.
        pub fn on_location_changed<T: FnMut(WindowsCoordinate) + Send + 'static>(
            &self,
            mut cb: T,
        ) -> Result<LocationHandler, Error> {
            let handler = TypedEventHandler::new(
                move |_sender: &Option<Geolocator>, result: &Option<PositionChangedEventArgs>| {
                    let Some(args) = result else {
                        error!("No position changed args received.");
                        return Ok(());
                    };
                    if let Ok(pos) = args.Position() {
                        if let Ok(coord) = pos.Coordinate() {
                            let out: WindowsCoordinate = (&coord).into();
                            cb(out);
                        }
                    }

                    Ok(())
                },
            );
            let res = self.locator.PositionChanged(&handler)?;
            trace!("Location handler registered.");
            Ok(LocationHandler {
                locator: &self.locator,
                token: res,
            })
        }
        pub fn on_status_changed<T: FnMut(PositionStatus) + Send + 'static>(
            &self,
            mut cb: T,
        ) -> Result<StatusHandler, Error> {
            let handler = TypedEventHandler::new(
                move |_sender: &Option<Geolocator>, result: &Option<StatusChangedEventArgs>| {
                    let Some(args) = result else {
                        error!("No status changed args received.");
                        return Ok(());
                    };
                    if let Ok(status) = args.Status() {
                        let out: PositionStatus = status.0.into();
                        cb(out);
                    }
                    Ok(())
                },
            );
            let res = self.locator.StatusChanged(&handler)?;
            Ok(StatusHandler {
                locator: &self.locator,
                token: res,
            })
        }
    }

    pub struct LocationHandler<'a> {
        locator: &'a Geolocator,
        token: EventRegistrationToken,
    }

    impl Drop for LocationHandler<'_> {
        fn drop(&mut self) {
            let _res = self.locator.RemovePositionChanged(self.token);
            trace!("Dropped location handler.");
        }
    }

    pub struct StatusHandler<'a> {
        locator: &'a Geolocator,
        token: EventRegistrationToken,
    }

    impl Drop for StatusHandler<'_> {
        fn drop(&mut self) {
            let _res = self.locator.RemoveStatusChanged(self.token);
            trace!("Dropped location handler.");
        }
    }
}

#[cfg(target_os = "windows")]
mod data;
#[cfg(target_os = "windows")]
mod error;
