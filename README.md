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
- The packet types, their flags and header fields can be found in [type_and_flags.rs](./src/packets/header/type_and_flags.rs).

- **0** [CERTIFICATE](#certificate)
- **1** [SESSION](#session)
- **2** [GET](#get)
- **3** STREAM
- **4** [POST](#post)
- **5** PATCH
- **6** PUT
- **7** DELETE
- **8** SUBSCRIBE
- **9** UNSUBSCRIBE
- **10** REGISTER
- **11** IDENTIFY
- **12** PROXY
- **13** CUSTOM
- **14** OPCODE
- **15** ERROR

## Plabble packet base
- Every packet extends the Plabble **packet base**.
- Implementation: [base/mod.rs](./src/packets/base/mod.rs)

The base of a packet will not be encrypted for it is required for the processing and decryption of the packet. It also does not contain any sensitive information. However, all encryption methods SHOULD make sure the base cannot be tampered by using a [MAC or authenticated data](#authentication).

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

# [12B] required if pre_shared_key flag is set.
psk_id = "base64url (no padding) pre-shared key ID"

# [16B] required if pre_shared_key flag is set. Random bytes to salt the key generated from PSK
psk_salt = "base64url (no padding) random generated salt"

# [1B] required if specify_crypto_settings is true
[crypto_settings]
encrypt_with_chacha = true   # default true, use ChaCha20(Poly1305)
encrypt_with_aes = false        # use AES 256 (CTR/GCM) for encryption
# 1 reserved flag for future use
use_blake3 = false              # use Blake3 instead of Blake2
sign_ed25519 = true             # default true, use Ed25519 for signing
key_exchange_x25519 = true      # default true, use X25519 for exchange
# 1 reserved flag for future use
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
# ... type-specific header fields/flags. See type_and_flags.rs

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
# ... type-specific header fields/flags. See type_and_flags.rs

[body]
# ... type-specific properties
```

## Certificate
- **Goal**: _Get the server certificate or another certificate by ID, prove server identity_ ...
- Implementation: [certificate.rs](./src/packets/body/certificate.rs)

### Certificate flow
1. The client sends the [ID of the certificate](#certificate-id) it wants to request (or no ID if it wants the certificate of the server), a challenge (or no challenge if the client is not interested in proving the servers' identity)
2. The server retrieves the certificates the client requested, and signs the challenge + certificate(s) bytes
3. The client verifies if the signature is correct
4. The client stores the certificates in its certificate store so it doesn't need to request it another time

### Certificate request
Request header flags:
- **full_chain**: if set, the server does not only return 1 certificate but all certificates in the certificate chain
- **full_certs**: if set, the server will return the full certificates of the chain. If not set, the chain certificates (all certificates above the certificate that is requested) are not fully sent, but only in a compact form (without body)
- **challenge**: if set, the client sends a 16-byte random challenge to the server to sign
- **query_mode**: if set, the client specifies the certificate to retrieve by its certificate ID

Request body:
- **id**: the [ID of the certificate](#certificate-id) the client wants to retrieve, 16 bytes, REQUIRED when *query_mode* is set
- **challenge**: the challenge for the server to sign, 16 bytes, REQUIRED when *challenge* flag is set.

Example:
```toml
version = 1
use_encryption = true

[header]
packet_type = "Certificate"
full_chain = true # this is the default
full_certs = true # this is the default
challenge = true  # default is false
query_mode = true # default is false

[body]
id = "AAAAAAAAAAAAAAAAAAAAAA"
challenge = "AQEBAQEBAQEBAQEBAQEBAQ"
```

### Certificate response
No response flags.

Response body:
- **signatures**: signatures for each algorithm determined by the crypto settings in the [base](#plabble-packet-base), in order. Each signature consist of challenge + list of certificates (as bytes)
- **certificates**: list of [Plabble certificates](#certificates) based on the request.

Example:
```toml
version = 1
use_encryption = true

[header]
packet_type = "Certificate"
request_counter = 1

    [[body.signatures]]
    Ed25519 = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQ"

    [[body.certificates]]
    id = "AgICAgICAgICAgICAgICAg"
    uri = "..."
    valid_from = "2025-05-15T12:00:00+00:00"
    valid_until = "2026-01-01T00:00:00+00:00"
    issuer_uri = "..."
    data = "CA=plabble"

        [[body.certificates.keys]]
        Ed25519 = "AwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM"

        [[body.certificates.signatures]]
        Ed25519 = "BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBA"
```

> The client should only trust a certificate if the chain is verified and the signatures are correct!

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
- **enable_encryption**: If set to true, switch to Plabble [encrypted communication](#encrypted-client-server-communication) between the client and the server to enjoy full packet encryption
- **with_salt**: If set to true, include 16-byte random generated salt by client
- **request_salt**: If set to true, force server to (also) generate and use a salt

Request body:
- **psk_expiration**: 4-byte [Plabble timestamp](#plabble-timestamp) to request the server to delete the pre-shared key afterwards. REQUIRED if *persist_key* header flag is set.
- **salt**: 16-byte salt for the KDF algorithm that is used to create the [session key](#session-key). REQUIRED if *with_salt* is set.
- **keys**: One or more key exchange algorithm from the following options: `X25519` (32B), `Kem512` (800B) or `Kem768` (1184B). They are encoded in the same order as they are in the [crypto settings](#plabble-packet-base). See [key_exchange.rs](./src/crypto/key_exchange.rs).

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
psk_expiration = 2025-05-27T07:32:00Z
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
- **signatures**: List of signatures the server created from the client request and the *psk_id* and *keys* to ensure integrity. Encoded the same way as the keys in the request. See [signatures.rs](./src/crypto/signatures.rs)

Example:
```toml
version = 1
# crypto settings etc. inherited from request

[header]
packet_type = "Session"
with_psk = true
with_salt = true

[body]
psk_id = "..." # base64url (no padding) encoded 12-byte key ID
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
- **Goal**: _request_ data from one or more slots inside a `bucket` on the server, optionally subscribe to updates.
- Implementation: [bucket.rs](src/packets/body/bucket.rs)

### Get flow
1. The client builds a `Get` request targeting a single bucket (identified by a 16-byte ID) and chooses a range to read.
2. The server looks up the bucket and returns the requested slots (or an empty result if none match).
3. If the `subscribe` flag is set, the server may continue to send updates for the requested keys/slots until the subscription is cancelled.

### Get request
Request header:
- **binary_keys**: keys in the request/response are UTF-8 strings instead of numeric slot indexes.
- **subscribe**: request a subscription for changes on the requested keys/range.
- **range_mode_until**: treat a single provided range value as an "until" (end-only) bound.
- **id**: 16-byte [bucket identifier](#bucket-id) (base64/URL-safe when using the TOML representation).

Request body:
- **range**: either `Numeric(start?, end?)` or `Binary(start_key?, end_key?)` (both bounds are optional). Numeric ranges use `u16` slots; binary ranges use UTF-8 keys.

Example (numeric range):
```toml
version = 1
use_encryption = true

[header]
packet_type = "Get"
id = "AAAAAAAAAAAAAAAAAAAAAA"  # 16-byte id (base64url (no padding))

[body]
range.Numeric = [5, 25]
```

Example (binary range):
```toml
version = 1
use_encryption = true

[header]
packet_type = "Get"
id = "AAAAAAAAAAAAAAAAAAAAAA"
binary_keys = true

[body]
range.Binary = ["key_start", "key_end"]
```

### Get response
Response header flags:
- **binary_keys**: indicates keys in the response are UTF-8 strings (matches the request's `binary_keys`).

Response body:
- `Get` responses carry a `BucketBody` (see [implementation](./src/packets/body/bucket.rs)). For numeric buckets this is a key-value set with numbers and binary values, for binary it is a key-value set with both binary keys and values.

Notes:
- Values in the TOML examples are typically encoded as base64 when represented in text (see `BucketBody` in the crate).
- When `subscribe` is set the server semantics for update delivery are implementation-dependent (push over the current connection or via separate subscription messages) but should follow the protocol's subscription guarantees.

## Post
- **Goal**: _Create a new bucket on the server_ ...
- Implementation: [post.rs](./src/packets/body/post.rs)

### Post flow
0. Requirement: a [Session](#session) MUST be established
1. The client generates a [bucket id](#bucket-id) and determines the [bucket permissions](#bucket-permissions).
2. The client sends the request to the server, the server creates the bucket (if it doesn't exist)
3. The client and server derive a [bucket key](#bucket-key) from the current session
4. The server sends an empty response or an [error](#errors) to indicate if it succeeded

### Post request
Request header flags:
- **binary_keys**: If you want to subscribe to the bucket, specify if the keys are binary or numeric
- **subscribe**: Subscribe to the bucket when it is created
- **range_mode_until**: Use "until" range mode when subscribing to the bucket
- **do_not_persist**: Create a **Memory bucket** instead, not persisting the bucket to the server storage but keeping it in RAM (not all servers will allow this)

Request body:
- **id**: the [Bucket ID](#bucket-id) you would like to use for this bucket. If it already exists you'll get an [error](#errors).
- **settings**: The [bucket settings](#bucket-permissions) and permissions you want to use
- **range**: Range if you want to subscribe to the bucket, REQUIRED if _subscribe_ is set.

Example:
```toml
version = 1
use_encryption = true

[header]
packet_type = "Post"
subscribe = true

[body]
id = "@test"
range.Numeric = [1, 10] # subscribe to key 1-10

settings.access_control_list = [] # If you want to prefill the ACL, add base64-urlencoded user IDs here

[body.settings.permissions]
public_read = true # this is one of the defaults
# ...
```

### Post response
Empty response without flags.

## Errors
- Implementation: [error.rs](./src/packets/body/error.rs)

0. **UnsupportedVersion**: Requested Plabble protocol version not supported by server. Body: `min_version` (min supported version by server), `max_version` (max supported version by server). _Occurence_: every request Plabble packet.
1. **UnsupportedAlgorithm**: Requested algotithm (in cryptography settings) is not supported by the server. Body: `name` The name of the algorithm(s) that is not supported, UTF-8 [dynint](#plabble-dynamic-int) length encoded. _Occurence_: any packet, but especially [Session](#session), [Certificate](#certificate) and other packets that use cryptography settings. 
10. **BucketNotFound**: Requested bucket was not found
11. **BucketAlreadyExists**: Bucket with that ID already exists. _Occurence_: [Post](#post)
110. **CertificateNotFound**: Requested certificate (by id) was not found. _Occurence_: [Certificate](#certificate-request)
111. **CertificateInvalid**: Requested certificate was not valid. _Occurence_: [Certificate](#certificate)

## Concepts

### Buckets
The Plabble Protocol is built around the concept of a **bucket**. A bucket is an isolated key-value database collection. A server can host many buckets, but a Plabble request is always targeting one bucket. Every bucket contains **slots** which are the entries inside the buckets. You can modify the binary content of a slot using its **key**. The **value** inside the slot is always binary (in bytes), no other data types are supported using the Plabble protocol. Every bucket has a [bucket id](#bucket-id) that is unique per server.

#### Bucket key types
There are two types of keys:
- **Numeric keys**: A _uint16_ value between 0 and 65535. So numeric buckets have a maximum amount of slots, exactly 65536. The big advantage of numeric slots is that they are very small to send (only 2 bytes) and that they follow a numeric order.
- **Binary keys**: A _utf-8_ encoded string. This gives the protocol more flexibility when working with buckets. The disadvantages are bigger requests and the dynamic length, so we need to prefix the keys with a [dynint](#plabble-dynamic-int) to encode the length in Plabble packets when using binary keys.

#### Bucket ID
- Implementation: [bucket_id.rs](./src/core/bucket_id.rs)

The bucket ID is the 16-byte server-wide unique ID for one bucket. There are 3 ways to notate a bucket ID:
1. Base64-URL-encoded without padding. For example: `RKiZXdULZlegN6eDkwRTWw`.
2. UTF-8 with the magic prefix `#`: This can be any valid UTF-8 string, like `#mybucket` or `#😁`. The content following the # will be hashed into a 16-byte ID using `blake2b-128`. This allows the Plabble protocol to use things like usernames, aliases, addresses etc.
3. UTF-8 with the magic prefix `@`: This is similar to the #, but uses `blake3-128` instead. Not all servers and client are required to support Blake3.

When creating a bucket, you can generate the bucket ID yourself. If it is taken, the server will return an error.

#### Bucket Key
The bucket key is a 64-byte secret that is derived from the [session key](#session-key) when [creating](#post-request) a bucket. It is stored on both the client and the server and is used for [authentication](#authentication).


#### Bucket permissions
- Implementation: [post.rs](./src/packets/body/post.rs)

Every bucket has permissions which are set when creating the bucket (altough they can be changed later).
Bucket Permissions come in 3 flavours:
- **public**: everyone on the internet who knows your bucket ID can do this
- **protected**: only people who are authenticated using [Identity](#TODO) and are on the *access_control_list* can do this
- **private**: only people who know the [bucket key](#bucket-key) can do this

The following list of settings/permissions with their default values is supported:
- **public_read**: (default _true_), allow _everyone_ to [read](#get) slots from this bucket
- **public_append**: (default _false_), allow _everyone_ to [append](#TODO) a slot to the bucket
- **public_write**: (default _false_), allow _everyone_ to [update](#TODO) a slot
- **public_delete**: (default _false_), allow _everyone_ to [delete](#TODO) a slot
- **public_script_execution**: (default _false_), allow _everyone_ to execute [opcode](#TODO) scripts interacting with this bucket (read/write/append/delete)
- **protected_read**: (default _true_), allow _users on the ACL_ to [read](#get) slots from this bucket
- **protected_append**: (default _false_), allow _users on the ACL_ to [append](#TODO) a slot to the bucket
- **protected_write**: (default _false_), allow _users on the ACL_ to [update](#TODO) a slot
- **protected_delete**: (default _false_), allow _users on the ACL_ to [remove](#TODO) a slot
- **protected_script_execution**: (default _false_), allow _users on the ACL_ to execute [opcode](#TODO) scripts interacting with this bucket (read/write/append/delete)
- **protected_bucket_delete**: (default _false_), allow _users on the ACL_ to [delete](#TODO) this bucket
- **private_read**: (default _true_), allow _users owning the [bucket key](#bucket-key)_ to [read](#get) slots from this bucket
- **private_append**: (default _true_), allow _users owning the [bucket key](#bucket-key)_ to [append](#TODO) a slot to the bucket
- **private_write**: (default _true_), allow _users owning the [bucket key](#bucket-key)_ to [update](#TODO) a slot
- **private_delete**: (default _true_), allow _users owning the [bucket key](#bucket-key)_ to [remove](#TODO) a slot
- **private_script_execution**: (default _false_), allow _users owning the [bucket key](#bucket-key)_ to execute [opcode](#TODO) scripts interacting with this bucket (read/write/append/delete)
- **private_bucket_delete**: (default _true_), allow _users owning the [bucket key](#bucket-key)_ to [delete](#TODO) this bucket
- **deny_existence**: (default: _false_) If public read is off and a user queries this bucket, let the server tell them this bucket does not exist

### Plabble-over-HTTPS (PoH)

### Session key
When creating a [session](#session-flow), the client and server will generate a **session key**.
The session key is used as key material for cryptographic functions during the session.

This is how to create a session key:
1. For each algorithm specified in the request, create a shared secret. Create a _hasher_ using the `blake2b-512` or `blake3` algorithm. Use blake3 if this is set in the crypto settings in the request. For each shared secret, _update_ the hash function. _Finalize_ the hasher into a 64-byte hash that will serve as the _input key material_.
3. If the client **provided a salt** in the [session request](#session-request), provide it as the _salt_. If the client did NOT provide a salt but asked the server for a salt, use the **server salt**. If also no server salt is available, use the ASCII equivalent of the string value `PLABBLE-PROTOCOL` (which is exactly 16 bytes)
4. If the client asked the server to **create a salt** in the [session request](#session-request), provide it as the _context_. If not, use the ASCII equivalent of the string value `PROTOCOL.PLABBLE` (which is be exactly 16 bytes)
5. Pass the `input key material`, `salt` and `context` to the [key generation function](#key-generation)
5. Derive a 64-byte session key and keep it in memory for the connection. If the user asked in the session request to store it, generate a random 12-byte [_PSK ID_](#psk-id) and return it to the client.

### PSK ID
We don't want an attacker to relate a PSK ID to a bucket key, so the PSK ID is a randomly generated 12-byte identifier.

### Authentication
Plabble has two ways of ensuring the integrity of packets.
When the `use_encryption` flag in the base packet is off, it will use a Message Authentication Code (MAC).
If the encryption flag is on, Plabble uses Authenticated Encryption with Associated Data (AEAD).

For each request

### Key generation
- Implementation: [context.rs](./src/crypto/mod.rs)

Plabble keys are generated using the `blake2b-512` or `blake3` functions.
The key derivation mechanism accepts 3 input parameters: `ikm` (64-byte input key material/existing key), `salt` (16-byte salt)
and `context` (16-byte unique(!) context)
For `blake3`, the context is passed to the *derive_key* mode/KDF mode of blake3 as a `base64-url (no padding)`-encoded UTF-8 string,and the hash is updated with the `ikm` first and then the `salt` before the hasher is finalized.
For `blake2b-512`, the _MAC mode_ is used accepting directly a `key`, `salt` and `persona`. The context is passed to the persona.

### Encrypted client-server communication

#### Encryption/decryption stream

#### Packet Encryption

#### Packet Decryption
1. If full packet encryption is enabled (within a [Session](#session)), all incoming bytes will be decrypted with a[crypto stream](#encryptiondecryption-stream). 
2. The base packet is read, decrypted and parsed. If the [crypto_settings](#plabble-packet-base) are set, the crypto settings of the decryption context are overwritten
3. If encryption is enabled on the base packet, the current crypto stream is overwritten with a new crypto stream based on the base packet settings. If no encryption is used, a 16-byte offset will be kept at the end of the stream for the MAC.
4. The packet header is read, decrypted and parsed.
5. The body bytes are read and decrypted. But this is not yet the plain text, because the body is encrypted twice.
6. The [associated data/authentication data](#authenticated-data) is generated from the context to make AEAD-decryption possible on the body.
7. The body is decrypted and parsed using the authenticated data.
8. Optionally, if no encryption was used, the MAC is read from the stream and checked.

#### Authenticated data
The authenticated data is a `blake2b-256` or `blake3` (64-byte) hash of the raw plaintext base packet bytes and header bytes and is used to authenticate the entire packet and decrypt the body. Optionally, also the [bucket key](#bucket-key) is appended if it is needed. When reading a request, the server first tries to decrypt the packet (or verify the MAC) _without_ the bucket key, and then _with_ the bucket key. At least one of them must succeed or the packet will be rejected. Responses never contain the bucket key in the authenticated data.

### Plabble Timestamp
- Implementation: [datetime.rs](./src/core/datetime.rs)

A Plabble Timestamp is a _uint32-big endian encoded_ **seconds** since the Plabble epoch, which is `2025-01-01T00:00:00Z` (the minimum datetime). The maximum datetime of the Plabble Timestamp is `2161-02-07T06:28:15.000Z` in RFC 3339 format.
The precision of a Plabble Timestamp is thus limited to 1 second which is sufficient for most things.
The advantage of the Plabble Timestamp is that it is thus very small, only taking 4 bytes.

### Plabble dynamic int
A dynamic int is serialized in a way that every last bit of a byte indicates if another byte is needed for encoding a number. This way, serializing integers can be very efficient. 

### Certificates
- Implementation: [certificate.rs](./src/crypto/certificate.rs)

Plabble uses its own certificate format, the _Plabble Certificate_. This is because we want the certificates to be as small as possible. Trust is based on our own root certificate authority. Everyone can become a certificate authority by starting an application. This can be done by creating an issue in this repository.
As a Plabble Client programmer, you need to include the root certificate in the client application.

#### Certificate ID
Every certificate has a unique 16-byte certificate ID. The ID is created by hashing the fields `valid_from`, `valid_to`, `issuer_uri`, `data` together using `blake2b-128` (in incremental mode). The data field thus MUST be unique.