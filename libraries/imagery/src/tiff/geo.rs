// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::ImageError;

#[derive(Debug, Copy, Clone)]
pub struct KeyEntry {
    pub id: u16,
    pub location: u16,
    pub count: u16,
    pub value_offset: u16,
}
#[derive(Debug, Clone)]
pub struct GeoKeyDirectory {
    pub version: u16,
    pub key_revision: u16,
    pub minor_revision: u16,
    pub keys: Vec<KeyEntry>,
}

impl GeoKeyDirectory {
    pub fn version(&self) -> u16 {
        self.version
    }
    pub fn key_revision(&self) -> u16 {
        self.key_revision
    }
    pub fn minor_revision(&self) -> u16 {
        self.minor_revision
    }
    pub fn keys(&self) -> &Vec<KeyEntry> {
        &self.keys
    }
    pub fn parse_from(value: &[u16]) -> Result<GeoKeyDirectory, ImageError> {
        let mut iter = value.iter();
        let version = iter
            .next()
            .copied()
            .ok_or(ImageError::not_enough_values())?;
        let key_revision = iter
            .next()
            .copied()
            .ok_or(ImageError::not_enough_values())?;
        let minor_revision = iter
            .next()
            .copied()
            .ok_or(ImageError::not_enough_values())?;

        let key_count = iter
            .next()
            .copied()
            .ok_or(ImageError::not_enough_values())?;
        let mut keys = Vec::new();
        for _ in 0..key_count {
            let id = iter
                .next()
                .copied()
                .ok_or(ImageError::not_enough_values())?;
            let location = iter
                .next()
                .copied()
                .ok_or(ImageError::not_enough_values())?;
            let count = iter
                .next()
                .copied()
                .ok_or(ImageError::not_enough_values())?;
            let value_offset = iter
                .next()
                .copied()
                .ok_or(ImageError::not_enough_values())?;
            keys.push(KeyEntry {
                id,
                location,
                count,
                value_offset,
            })
        }

        Ok(GeoKeyDirectory {
            version,
            key_revision,
            minor_revision,
            keys,
        })
    }
}
