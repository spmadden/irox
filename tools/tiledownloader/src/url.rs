use irox_tools::iterators::{looping_forever::LoopingForever, Itertools};

use crate::tile::{TileAddress, TileAddressURL};

pub struct URLParams {
    pub min_x_idx: u64,
    pub max_x_idx: u64,
    pub min_y_idx: u64,
    pub max_y_idx: u64,

    pub zoom_level: u8,
    pub server_parts: Option<Vec<String>>,
    pub url_template: String,
}

impl IntoIterator for URLParams {
    type Item = TileAddressURL;

    type IntoIter = URLIterator;

    fn into_iter(self) -> Self::IntoIter {
        URLIterator {
            x_idx: self.min_x_idx,
            y_idx: self.min_y_idx,
            server_parts: self
                .server_parts
                .clone()
                .map(|e| e.into_iter().looping_forever()),
            params: self,
        }
    }
}

pub struct URLIterator {
    params: URLParams,
    x_idx: u64,
    y_idx: u64,
    server_parts: Option<LoopingForever<String>>,
}

impl Iterator for URLIterator {
    type Item = TileAddressURL;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_idx >= self.params.max_x_idx && self.y_idx >= self.params.max_y_idx {
            return None;
        }
        if self.x_idx >= self.params.max_x_idx {
            self.x_idx = self.params.min_x_idx;
            self.y_idx += 1;
        }

        let inversed_y = (1 << self.params.zoom_level) - self.y_idx - 1;

        let x = format!("{}", self.x_idx);
        let y = format!("{}", self.y_idx);
        let inv_y = format!("{inversed_y}");
        let z = format!("{}", self.params.zoom_level);

        let address = TileAddress {
            x_index: self.x_idx,
            y_index: inversed_y,
            zoom_level: self.params.zoom_level,
        };

        self.x_idx += 1;

        let mut url = self
            .params
            .url_template
            .replace("{x}", x.as_str())
            .replace("{y}", y.as_str())
            .replace("{z}", z.as_str())
            .replace("{-y}", inv_y.as_str());

        if let Some(svr) = &mut self.server_parts {
            if let Some(sparam) = svr.next() {
                url = url.replace("{s}", sparam.as_str());
            }
        }

        Some(TileAddressURL { address, url })
    }
}
