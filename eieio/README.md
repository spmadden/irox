IROX-EIEIO
============

*Ergonomic Interfaces with Extensible Interchange Operations - EIEIO*

> *and on this farm he had some pigs...*

`IROX-EIEIO` is a software toolkit that helps to mitigate the damage done by the Gang-of-Four's `Adapter` pattern. Once
a junior developer learns of this pattern, suddenly it's used everywhere - like throwing spaghetti against a wall - it
sticks.

`EIEIO` is simply an implementation of a Live-Proxy/Facade pattern. Traits are used as the primary indicators of
behaviors, and builders for a particular codec provide the specific implementation of those traits, whose actual
underlying data-storage format varies based on which implementation is chosen. The actual implementation of the struct
is fully hidden behind the Trait object itself

## Concepts:

### Message
A `Message` is a discrete and atomic (typically event-based) unit of communication between two entities in a `System`. 
Messages are usually encoded to and from bytes (or files, or strings) and passed between entities through a 
communications `Channel`.  

In EIEIO, `Messages` and their builders are described using Traits to codify the core concepts they represent.  The actual
serialization and deserialization of a message is performed by a particular `Codec` implementation.

The `Message` Traits and the `Builder` Traits live in the [`EIEIO API`](./api) module.

For example, the concept of a GNSS/GPS "Fix" (a discrete position) can be represented or encoded in any number of ways,
including NMEA-0183, SIRf Binary, as a GPX Waypoint or Track, or even sourced from the Windows Location API.  These
methods of encoding and decoding data are known as `Codec`s in EIEIO.   In all of these encodings, 95% of all the 
concepts are the same, just named, encoded, formatted, or ordered differently.  By abstracting away the details of
specifically /which/ `Codec` is being used, the application logic can focus on the application, and NOT on the specifics
of encoding or decoding the data.


### Codec
A codec is an implementation of an encoding or decoding operation to a specific data format.  NMEA-0183, KML, GPX, and 
many others all have the concept of "Position", "Waypoint", and "Track", yet each encodes the data differently.

Because Codecs all share the same API, converting a message that originated in one codec into the format of another 
codec is trivially easy.

An implementation of a codec is permitted to have mutable internal state (so long as the implementation manages that
state itself).  As such, using multiple messages to aggregate data together through "Collectors" is possible. 

### Basic Conceptual Structure:
```
Codec {
  /// uniquely identifies this codec implementation with a name, description, and version (for multi-version codecs)
  get_codec_id => CodecIdentifier {
    name => String
    description => Option<String>
    version => Option<String> 
  }
  
  /// A enum list of the message types this codec supports
  get_supported_builders => [MessageType...]
  
  /// The assorted sources from which this codec supports parsing into messages
  get_supported_readers => SupportedReaders {
    from_file => Option<ReadFromFile>
    from_bytes => Option<ReadFromBytes>
    from_directory => Option<ReadFromDirectory>
    from_reader => Option<ReadFromRead>
    from_string => Option<ReadFromString>
    from_xyz...
  }
  
  /// And finally the builder types themselves
  get_gnss_fix_builder => Option<GNSSFixBuilder{
  
    /// setters for the indivual fields themselves
    set_timestamp...
    set_positions...
    set_xyz...
    
    /// Build is implemented by the specific codec, and this one returns a dyn GNSSFix Trait Object
    build => dyn GNSSFix {
      /// The following getters are unique to each individual message's fields
      get_timestamp => UTCDateTime,
      get_positions => Positions,
      get_xyz...
    
      /// the 'super' "Base Message" field is common across ALL messages.
      get_super => BaseMessage {
        
        /// enum from 'supported builders' above.
        get_message_type => MessageType::GnssFix 
        
        /// A reference to the codec that originally created this message
        get_codec => Codec
        
        /// enum with an "Owned Copy" of this message to avoid messy generics
        get_message => Message::GnssFix(dyn GNSSFix)
        
        /// And the various different ways this message can be encoded, usually the same supported
        /// as 'SupportedReaders' above.
        get_supported_writers => SupportedWriters {
          to_file => Option<WriteToFile>
          to_directory => Option<WriteToDirectory>
          to_bytes => Option<WriteToBytes>
          to_writer => Option<WriteToWrite>
          to_string => Option<WriteToString>
          to_xyz...
        }
      }

    }
    
  }>
  
  /// and builders for other supported message types
  xyz_message_builder => Option<XYZMessageBuilder>
  ... 
}
```

### Channel
> 🚧 *Current Status*: Future Work🚧

A Channel is a mechanism to move messages from one place to another.  TCP, QUIC, WebSockets, and message brokers like 
MQTT, ZeroMQ, or RabbitMQ are examples of Channels.

## Goals:

* Support easy conversion of one data type to another
* Allow application logic to operate on multiple different data exchange formats without having to know the details of
  the underlying format

## Roadmap

Support Codecs:

* NMEA0183
  * Current Status: 
    * `GNSSFix` Supported using `GGA` and `ZDA` messages
* NMEA2000
  * Current Status: 🚧None
* GPSd JSON
  * Current Status: 🚧None
* GPX
  * Current Status: 🚧None
* SIRf
  * Current Status: 🚧None
* Windows Location API
  * Current Status: 🚧None
* KML
  * Current Status: 🚧None
