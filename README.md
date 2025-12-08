# Plabble Transport Protocol
This repository provides a Rust implementation of the Plabble Transport Protocol.

You can read the old spec [here](./PLABBLE.md)

# Plabble Protocol

## Packets
The Plabble Protocol works with messages called **packets**. There are two groups of packets: [request packets](#plabble-request-packet) and [response packets](#plabble-response-packet). Request packets are the packets that are sent from the client to the server and response packets are packets that are sent from the server to the client. We cannot make it easier for you!

Plabble packets can be used in two forms: TOML and binary. The TOML variant is only used for unencrypted payloads or to communicate with a library or service that translates the packets to encrypted, binary payloads. The TOML variant is not meant to be sent over the Plabble network, but can be used with [Plabble-over-HTTP(S)](#plabble-over-https-poh). In this documentation we use the TOML variant a lot because it is very human-readable and easy to explain.

Every Plabble packet contains of 3 parts, the [base](#plabble-packet-base), the **header** and the **body**. The header and body are different depending on if it is a request or response and the [packet type](#packet-type).

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
- 13 is reserved for future use
- **14** OPCODE
- **15** ERROR

## Plabble packet base
- Every packet extends the Plabble **packet base**.
- Implementation: [base/mod.rs](./src/packets/base/mod.rs)

The base of a packet will not be encrypted for it is required for the processing and decryption of the packet. It also does not contain any sensitive information. However, all encryption methods SHOULD make sure the base cannot be tampered by using a [MAC or authenticated data](#packet-integrity).

A Plabble packet base has the following base structure:

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

# [16B] required if pre_shared_key flag is set. Random bytes to salt the key generated from PSK
psk_salt = "base64url random generated salt"

# [16B] required if use_encryption flag is not set. 
mac = "base64url encoded MAC"

# [1B] required if specify_crypto_settings is true
[crypto_settings]
encrypt_with_cha_cha20 = true   # default true, use ChaCha20(Poly1305)
encrypt_with_aes = false        # use AES for encryption
larger_hashes = false           # use larger hashes if possible
use_blake3 = false              # use Blake3 instead of Blake2
sign_ed25519 = true             # default true, use Ed25519 for signing
key_exchange_x25519 = true      # default true, use X25519 for exchange
# 1 reserved flagg
use_post_quantum = false        # if set, include another byte with PQ eencryption settings

# [1B] required if crypto_settings.use_post_quantum is set
[crypto_settings.post_quantum_settings]
sign_pqc_dsa_44 = false          # use DSA44 for signing
sign_pqc_dsa_65 = false          # use DSA65 for siging
sign_pqc_falcon = false          # use Falcom-1024 for signing
sign_pqc_slh_dsa = false         # use SLH-DSA-SHA128s for signing
key_exchange_pqc_kem_512 = false # use ML-KEM-512 for key exchange
key_exchange_pqc_kem_768 = false # use ML-KEM-768 for key exchange
# 2 reserved flags
```

Most of these properties are optional and will only be sent in an initial request.

### Plabble request packet
- A Plabble Request packet contains of the [packet base](#plabble-packet-base), the request header and the request body.
- Header implementation: [packets/header/request_header.rs](./src/packets/header/request_header.rs)
- Body implementation: [packets/body/mod.rs](./src/packets/body/mod.rs)
- For each request type, a different request body will be used.

The Plabble request packet looks like this:
```toml
version = 1
# ... and other base properties/flags

[header]
packet_type = "Session" # the packet type in PascalCase
# ... type-specific flags. See type_and_flags.rs

[body]
# ... type-specific properties
```

### Plabble response packet
- A Plabble Response packet contains of the [packet base](#plabble-packet-base), the response header and the response body.
- Header implementation: [packets/header/response_header.rs](./src/packets/header/response_header.rs)
- Body implementation: [packets/body/mod.rs](./src/packets/body/mod.rs)
- For each response type, a different response body will be used.

The Plabble response packet looks like this:
```toml
version = 1
# ... and other base properties/flags

[header]
packet_type = "Session" # the packet type in PascalCase
request_counter = 1     # counter of the request to reply to (the server counts the client requests in a session). Optional. Only required if fire_and_forget is NOT set
# ... type-specific flags. See type_and_flags.rs

[body]
# ... type-specific properties
```

## Session
- **Goal**: _exchange keys_ with a server, create a [Session Key](#session-key).
- Implementation: [session.rs](./src/packets/body/session.rs)

> WARNING: You can only start a secure session if you know and trust the server's certificate!

### Session flow
1. The client generates one or more keypairs using one or more algorithms it wants to use for this session.
2. The client sends a [session request](#session-request) packet. The client COULD specify the cryptographic algorithms it wants to use using the `crypto_settings`. The public keys or encapsulation keys should be included.
3. The server also generates the keypairs for the algorithms the client requested, if applicable. Or it encapsulates a shared secret.
4. The server signs the entire **plain text** request (in binary form) the client sent using each signature algorithm the client specified in the `crypto_settings` AND the response (*psk_id*, optionally *salt* and *keys*) it will return to the client to ensure integrity.
5. The server generates a shared secret and derives a [session key](#session-key) from it. Optionally it stores the key if the client requested to store it as a PSK.
6. The server returns a [session response](#session-response).
7. The client checks the signatures the server returned using the public keys in the **server certificates** it already SHOULD know. This step is optional, but strongly recommended for integrity.
8. The client also generates a shared secret and derives the [session key](#session-key) from it.

### Session request
Request header flags:
- **persist_key**: If set to true, persist the generated shared secret key and return a server-generated [PSK ID](#psk-id).
- **enable_encryption**: If set to true, switch to Plabble [encrypted communication](#encrypted-client-server-communication) between the client and the server.
- **with_salt**: If set to true, include 16-byte random generated salt by client
- **request_salt**: If set to true, force server to (also) generate and use a salt

Request body:
- **psk_expiration**: 4-byte [Plabble timestamp](#plabble-timestamp) to request the server to delete the pre-shared key afterwards. REQUIRED if *persist_key* header flag is set.
- **salt**: 16-byte salt for the KDF algorithm that is used to create the [session key](#session-key). REQUIRED if *with_salt* is set.
- **keys**: One or more key exchange algorithm from the following options: `X25519` (32B), `Kem512` (800B) or `Kem768` (1184B). They are encoded in the same order as they are in the [crypto settings](#plabble-packet-base). See [crypto_keys.rs](./src/packets/base/crypto_keys.rs).

Example:
```toml
version = 1
specifiy_crypto_settings = true

[crypto_settings]
key_exchange_x25519 = true # this is the default
sign_ed25519 = true # this is the default
use_post_quantum = true

[crypto_settings.post_quantum_settings]
key_exchange_pqc_kem_512 = true
key_exchange_pqc_kem_768 = true

[header]
packet_type = "Session"
persist_key = true
with_salt = true
request_salt = true

[body]
psk_expiration = 2025-05-27T07:32:00-08:00Z
salt = "..."

[[body.keys]]
X25519 = "..."

[[body.keys]]
Kem512 = "..."

[[body.keys]]
Kem786 = "..."
```

> The server SHOULD respect the cryptography settings of the client. If the server does not support the cryptographic algorithms the client asked for, it MUST abort the connection with an error code.

### Session response
Response header flags:
- **with_psk**: If set, the client requested to store the session key as a PSK. This is a 12-byte ID.
- **with_salt**: If set to true, the response contains a 16-byte random generated salt by the server.

Response header body:
- **psk_id**: 12-byte ID the server assigned to the stored key derived from the shared secret of this session. REQUIRED if *with_psk* is set.
- **salt**: 16-byte salt generated by server. REQUIRED if *with_salt* is set.
- **keys**: List of public keys or encapsulated secrets the server generated according to the request. Similar to the request.
- **signatures**: List of signatures the server created from the client request and the *psk_id* and *keys* to ensure integrity. Encoded the same way as the keys in the request.

Example:
```toml
version = 1
# crypto settings etc. inherited from request

[header]
packet_type = "Session"
with_psk = true
with_salt = true

[body]
psk_id = "..." # base64url encoded 12-byte key ID
salt = "..."

[[body.keys]]
X25519 = "..."

[[body.keys]]
Kem512 = "..." # The KEM keys aren't public keys but encapsulated secrets

[[body.keys]]
Kem768 = "..."

[[body.signatures]]
Ed25519 = "..."
```

> The client SHOULD validate the signatures and validate if the server returned the algorithms it asked for! Else it should not trust the session and disconnect.

## Get
- **Goal**: _request_ data from one or more slots inside a [bucket](#buckets) on the server.


## Concepts

### Buckets
The Plabble Protocol is built around the concept of a **bucket**. A bucket is an isolated key-value database collection. A server can host many buckets, but a Plabble request is always targeting one bucket. Every bucket contains **slots** which are the entries inside the buckets. You can modify the binary content of a slot using its **key**. The **value** inside the slot is always binary (in bytes), no other data types are supported using the Plabble protocol. 

#### Bucket key types
There are two types of keys:
- **Numeric keys**: A _uint16_ value between 0 and 65535. So numeric buckets have a maximum amount of slots, exactly 65536. The big advantage of numeric slots is that they are very small to send (only 2 bytes) and that they follow a numeric order.
- **Binary keys**: A _utf-8_ encoded string. This gives the protocol more flexibility when working with buckets. The disadvantages are bigger requests and the dynamic length, so we need to prefix the keys with a [dynint](#plabble-dynamic-int) to encode the length in Plabble packets when using binary keys.

### Plabble-over-HTTPS (PoH)

### Session key
When creating a [session](#session-flow), the client and server will generate a **session key**.
The session key is used as key material for cryptographic functions during the session.

This is how to create a session key:
1. For each algorithm specified in the request, create a shared secret. Create a _hasher_ using the `blake2b` or `blake3` algorithm. Use blake3 if this is set in the crypto settings in the request. For each shared secret, _update_ the hash function. _Finalize_ the hasher into a 64-byte hash that will serve as the _input key material_.
2. Use the `blake2b_512` with _salt and personal_ or `blake3` in KDF mode algorithm.
3. If the client provided a salt in the [session request](#session-request), provide it to the _salt_ field of the blake KDF or MAC function. If the client did not provide a salt but asked the server for a salt, use the server salt. If also no server salt is available, use the ASCII equivalent of the string value `PLABBLE-PROTOCOL` (which should be exactly 16 bytes)
4. If the client asked the server to create a salt in the [session request](#session-request), provide it to the _persona_ field of the blake KDF or MAC function. If not, use the ASCII equivalent of the string value `PROTOCOL.PLABBLE` (which should be exactly 16 bytes)
5. Derive a 64-byte session key using the salts and keep it in memory for the connection. If the user asked in the session request to store it, generate a random 16-byte _PSK ID_ and return it to the client.

### PSK ID

### Packet integrity

### Encrypted client-server communication

### Plabble Timestamp

### Plabble dynamic int