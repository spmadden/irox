// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::msgs::{ChronycRequest, PAYLOAD_LEN};
use core::ops::{Deref, DerefMut};
use irox_bits::{BitsBuffer, BitsErrorKind, BitsWrapper, Error};
use irox_structs::Struct;
use irox_tools::buf::ZeroedBuffer;
use irox_tools::cfg_unix;
use irox_tools::hex::HexDump;
use std::fs::Permissions;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct Chronyc<'a, T> {
    internal: BitsBuffer<'a, T>,
}

impl<'a, T> Chronyc<'a, T> {
    pub fn new(inner: BitsBuffer<'a, T>) -> Self {
        Self { internal: inner }
    }
}
impl<T: Read + Write> Chronyc<'_, T> {
    pub fn request_response(&mut self, request: &mut ChronycRequest) -> Result<(), Error> {
        let mut buf = Vec::<u8>::with_capacity(PAYLOAD_LEN);
        request.write_to(&mut buf)?;
        self.internal.write_all_bytes(&buf)?;
        buf.hexdump();
        self.internal.flush()?;

        let mut buf = Vec::<u8>::new_zeroed(PAYLOAD_LEN);
        self.internal.read_some_into(&mut buf.as_mut_slice())?;
        buf.hexdump();
        Ok(())
    }
}

cfg_unix! {
    use irox_bits::{Bits, MutBits};
    use irox_tools::{fs::TempDirPath, random::PRNG};
    use std::os::unix::net::UnixDatagram;

    pub fn connect_unix_socket<T: AsRef<Path>>(path: T) -> Result<Chronyc<'static, TempSock>, Error> {
        let path = path.as_ref();
        let pid = std::process::id();
        let random = irox_tools::random::PcgXshRR::default().next_u32();
        let Some(parent) = path.parent() else {
            return Error::err(
                BitsErrorKind::FormatError,
                "Socket path must be to a socket and have a parent path",
            );
        };
        let mysocketdir = parent.join(format!("irxchronyc.{pid}"));
        let tmppath = TempDirPath::from(&mysocketdir);
        let mysocketrnd = &mysocketdir.join(format!("{random:0X}"));
        let mysocket = &mysocketrnd.join("sock");
        std::fs::create_dir_all(mysocketrnd)?;
        let sock = UnixDatagram::bind(mysocket)?;
        sock.connect(path)?;

        std::fs::set_permissions(&mysocketdir, Permissions::from_mode(0o711))?;
        std::fs::set_permissions(mysocketrnd, Permissions::from_mode(0o711))?;
        std::fs::set_permissions(mysocket, Permissions::from_mode(0o666))?;
        let sock = TempSock {
            path: Some(tmppath),
            sock,
        };
        let mut buf = BitsBuffer::new(BitsWrapper::Owned(sock));
        buf.flush_every_n(PAYLOAD_LEN);

        Ok(Chronyc::new(buf))
    }

    pub struct TempSock {
        path: Option<TempDirPath>,
        sock: UnixDatagram,
    }
    impl TempSock {
        pub fn path(&self) -> Option<&TempDirPath> {
            self.path.as_ref()
        }
    }
    impl Drop for TempSock {
        fn drop(&mut self) {
            if let Some(p) = self.path.take() {
                drop(p);
            }
        }

    }
    impl Read for TempSock {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.sock.recv(buf)
        }
    }
    impl Write for TempSock {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.sock.send(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }

    }
    impl Deref for TempSock {
        type Target = UnixDatagram;

        fn deref(&self) -> &Self::Target {
            &self.sock
        }
    }
    impl DerefMut for TempSock {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.sock
        }
    }

}
