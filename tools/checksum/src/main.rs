// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Checksumming tool
//!

#![forbid(unsafe_code)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::print_stdout)]

use irox_tools::hash::HashAlgorithm;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

#[cfg(not(target_arch = "wasm32"))]
const BUF_SIZE: usize = 1024 * 1024;
#[derive(Debug)]
pub struct LiveItem<T> {
    _permit: OwnedSemaphorePermit,
    item: T,
}
impl<T> Deref for LiveItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
pub struct SmarterSender<T> {
    tx: Sender<Arc<LiveItem<T>>>,
    rx: Option<Receiver<Arc<LiveItem<T>>>>,
    sema: Arc<Semaphore>,
}
impl<T: Clone + Debug> SmarterSender<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel(capacity);
        Self {
            tx,
            rx: Some(rx),
            sema: Arc::new(Semaphore::new(capacity)),
        }
    }
    pub fn receiver(&mut self) -> Receiver<Arc<LiveItem<T>>> {
        self.rx.take().unwrap_or_else(|| self.tx.subscribe())
    }
    pub async fn send(&self, val: T) {
        let pmt = self.sema.clone().acquire_owned().await.unwrap();
        let li = Arc::new(LiveItem {
            _permit: pmt,
            item: val,
        });
        self.tx.send(li).unwrap();
    }
}
#[cfg(not(target_arch = "wasm32"))]
async fn start_file(
    path: &PathBuf,
    tx: SmarterSender<Arc<[u8]>>,
) -> Result<tokio::task::JoinHandle<()>, std::io::Error> {
    use tokio::io::AsyncReadExt;
    let mut file = tokio::fs::File::open(&path).await?;
    file.set_max_buf_size(BUF_SIZE);
    let fut = tokio::spawn(async move {
        let mut file = file;
        loop {
            let mut buf = Vec::<u8>::with_capacity(BUF_SIZE);
            let read = file.read_buf(&mut buf).await.unwrap();
            if read == 0 {
                break;
            }
            buf.truncate(read);
            let buf: Arc<[u8]> = buf.into();
            tx.send(buf).await;
        }
    });
    Ok(fut)
}
#[cfg(not(target_arch = "wasm32"))]
fn spawn_single_job(
    alg: HashAlgorithm,
    ss: &mut SmarterSender<Arc<[u8]>>,
    path: &std::path::Path,
    printalg: bool,
) -> tokio::task::JoinHandle<()> {
    let mut rx = ss.receiver();

    let path = path.display().to_string();
    let task = tokio::task::spawn(async move {
        let mut hasher = alg.hasher();
        while let Ok(rxb) = rx.recv().await {
            hasher.write(&rxb);
        }
        let hash = hasher.finish();
        let hash = irox_tools::hex::to_hex_str_lower(&hash);
        if printalg {
            println!("{alg}: {hash} *{path}");
        } else {
            println!("{hash} *{path}");
        }
    });

    task
}

#[cfg(target_arch = "wasm32")]
fn main() {}
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    use clap::Parser;

    let args = Args::parse();
    let path = args.path;
    let mut algs = if args.all {
        Vec::from(HashAlgorithm::items())
    } else {
        args.algorithm
    };
    if algs.is_empty() {
        algs.push(HashAlgorithm::BLAKE2b512);
    }
    let mut tasks = Vec::new();
    let printalg = algs.len() != 1;
    for p in &path {
        let mut ss = SmarterSender::new(100);
        for n in &algs {
            tasks.push(spawn_single_job(*n, &mut ss, p, printalg));
        }
        tasks.push(start_file(p, ss).await?);
    }
    for t in tasks {
        t.await?;
    }
    Ok(())
}
#[derive(Debug, Clone, clap::Parser)]
#[command(version, about)] // Read from `Cargo.toml`
pub struct Args {
    #[arg(short = 'b')]
    /// <md5> <sha1> <sha224> <sha256> <sha384> <sha512> <murmur3_128|murmur3|m3> <b2|b2b|blake2b|blake2b512> <b2s|blake2s|blake2s256> <murmur3_32> <blake2s128>
    algorithm: Vec<HashAlgorithm>,

    #[arg(short = 'a')]
    /// run all available algorithms
    all: bool,

    #[arg(value_name = "DIR|File", value_hint = clap::ValueHint::AnyPath)]
    path: Vec<PathBuf>,
}
