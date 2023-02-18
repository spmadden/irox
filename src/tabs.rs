use std::{collections::HashMap, sync::{Mutex, Arc}};

use lazy_static::lazy_static;

lazy_static! {
    static ref CACHED_TABS: Mutex<HashMap<usize, Arc<String>>> = {
        Mutex::new(HashMap::new())
    };
}

pub(crate) fn tabs(len: usize) -> Arc<String> {
    let mut res = CACHED_TABS.lock().unwrap();
    let elem = res.entry(len)
        .or_insert_with_key(|k| Arc::new("\t".repeat(*k)));
    elem.to_owned()
}