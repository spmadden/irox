IROX-LOG
==========
*Basic console and file logging*

Examples
----------

### Console Logging: 

```rust
use irox_log::{init_console_level};
use log::{Level, info};

pub fn main() {
    // With 'Info' specified, 'Error', 'Warn' and 'Info' will be printed, but 'Debug' and 'Trace' will not.
    irox_log::init_console_level(Level::Info);

    info!("it works!");
}
```