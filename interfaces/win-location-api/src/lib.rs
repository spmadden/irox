// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#[cfg(target_os = "windows")]
pub use crate::windows::*;

#[cfg(target_os = "windows")]
mod windows {
    use log::{error, trace, warn};
    use windows::Devices::Geolocation::{Geolocator, PositionChangedEventArgs};
    use windows::Foundation::{EventRegistrationToken, TypedEventHandler};

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
            Ok(LocationHandler {
                locator: &self.locator,
                token: res,
            })
        }
    }

    pub struct LocationHandler<'a> {
        locator: &'a Geolocator,
        token: EventRegistrationToken,
    }

    impl<'a> Drop for LocationHandler<'a> {
        fn drop(&mut self) {
            let _res = self.locator.RemovePositionChanged(self.token);
        }
    }
}

#[cfg(target_os = "windows")]
mod data;
#[cfg(target_os = "windows")]
mod error;
