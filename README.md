# Plabble Transport Protocol
This repository provides a Rust implementation of the Plabble Transport Protocol.

You can read the old spec [here](./PLABBLE.md)

# Plabble Protocol

## Packets
The Plabble Protocol works with messages called **packets**. There are two groups of packets: [request packets](#plabble-request-packet) and [response packets](#plabble-response-packet). Request packets are the packets that are sent from the client to the server and response packets are packets that are sent from the server to the client. We cannot make it easier for you!

Plabble packets can be used in two forms: TOML and binary. The TOML variant is only used for unencrypted payloads or to communicate with a library or service that translates the packets to encrypted, binary payloads. The TOML variant is not meant to be sent over the Plabble network, but can be used with [Plabble-over-HTTP(S)](#plabble-over-https-poh). In this documentation we use the TOML variant a lot because it is very human-readable and easy to explain.

## Packet type
- The Plabble Transport Protocol is build upon several **packet types**.
- The packet types and their flags can be found in [type_and_flags.rs](./src/packets/header/type_and_flags.rs).

- **0** CERTIFICATE
- **1** [SESSION](#session)
- **2** GET
- **3** STREAM
- **4** POST
- **5** PATCH
- **6** PUT
- **7** DELETE
- **8** SUBSCRIBE
- **9** UNSUBSCRIBE
- **10** REGISTER
- **11** IDENTIFY
- **12** PROXY
- 13,14 are reserved for future use
- **15** ERROR

## Plabble base packet
- Every packet extends the Plabble **base packet**.
- Implementation: [base/mod.rs](./src/packets/base/mod.rs)

A Plabble packet has the following base structure:

```toml
# [u4] the version number of Plabble. 0 = debug. Serialized as first 4 bits of Plabble packet
version = 1 

# [bit] 1st bit flag. If set to true, the packet is sent outside of a session and no or only a single response is expected
fire_and_forget = false

# [bit] 2nd bit flag. If set to true, the packet uses a pre-shared key for the encryption of this packet. The PSK ID MUST be included if this flag is set
pre_shared_key = false

# [bit] 3rd bit flag. If set to true, this packet uses the Plabble encryption. If set to false, a MAC MUST be included
use_encryption = false

# [bit] 4th bit flag. If set to true, next byte will contain 7 flags for cryptography settings. If not set, the defaults will be used.
specifiy_crypto_settings = true

# [16B] required if pre_shared_key flag is set.
psk_id = "base64url pre-shared key ID"

# [16B] required if pre_shared_key flag is set. Random bytes to salt key generated from PSK
psk_salt = "base64url random generated salt"

# [16B] required if use_encryption flag is not set. 
mac = "base64url encoded MAC"

[crypto_settings]
TODO continue
```

### Plabble request packet

### Plabble response packet

## Session
- **Goal**: _exchange keys_ with a server, create a [Session Key](#session-key).
- Implementation: [session.rs](./src/packets/body/session.rs)

### Session request

Request flags:
- **persist_key**: If set to true, persist the generated shared secret key and return a server-generated [PSK ID](#psk-id).
- **enable_encryption**: If set to true, switch to Plabble [encrypted communication](#encrypted-client-server-communication) between the client and the server.




## Concepts

### Plabble-over-HTTPS (PoH)

### Session key

### PSK ID

### Encrypted client-server communication