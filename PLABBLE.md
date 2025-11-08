# Plabble Protocol Specification
A Plabble client can connect to a Plabble server that is port forwarded and reachable from the ethernet. The server uses a database to store messages in buckets.
Also, a server can be used as a broker to establish a peer-to-peer connection between two clients that both are connected to the server

When using local network/bluetooth, direct connections are used (client -> client).

Goals:
- General-purpose, anonymous and secure protocol for storing files, messages, chats, video streams etc.

# Security improvements
- [Password-Authenticated Key Agreement](https://github.com/RustCrypto/PAKEs): to prevent "offline guessing"

- TODO Use [X.509](https://github.com/RustCrypto/formats/tree/master/x509-cert) certficates instead?
- TODO more unqiue buckets by hasing (named bucket, server ID/domain?)

# Codec
- Custom binary format
- Interop/FFI format: TOML strings
- Plabble-Over-HTTPS (PoH) supports TOML, `application/toml`, or JSON.
- `Bucket ID` should be 16 bytes, GUID is 16 bytes too so we can display bucket IDS like GUIDs. Named buckets (string) are hashed with `Blake2b_128` to get the 16-byte ID. Bucket IDS tend to be unique across servers (GUID-nature), but this is not a enforced.

## Packet
```
Header (3-32B)
	---- unencrypted ----
	Length (dynint) (1-4B)
	Version (4b) [Plabble version 0-127]
	Flags   (4b) [4 flags]
		- FireAndForget: Fire-and-Forget [if encryption enabled, use PskId. If off, use counter]
		- PreSharedKey: Use pre-shared key/PSK [required when using FireAndForget]
		- UseEncryption: Encryption Enabled [if disabled, use a MAC]
		- SpecifyCryptoSettings: Specify algorithms used (adds 1 byte of encryption settings)
	CryptoSettings (1B)* [required if SpecifyCryptoSettings set]
	PostQuantumSettings (1B)* [required if CryptoSettings.UsePostQuantum set]
	PskId (12B)* [required if PreSharedKey is set]
	PskSalt  (12B)* [required if PreSharedKey is set]
	
	---- header encryption ----
	Type         (4b)  [CERTIFICATE, SESSION, GET, POST, PATCH, PUT, DELETE, SUBSCRIBE, UNSUBSCRIBE, STREAM, REGISTER, IDENTIFY, PROXY, ERROR] 
	Secure Flags (4b)  [4 flags]
	ResponseTo   (2B)* [counter of request to respond to, if in session]

Body
	---- body encryption ----
	Data (variable)

Tail (0-32B)
	---- unencrypted ----
	MAC (16-32B)* [Blake2b_128 or 256 MAC. Depends on LargeHashes flag]
```

How an encrypted packet can be read and parsed:
- Read length from stream (dynint), byte by byte
- Read full packet in buffer
- Read version & flags byte
- Based on PreSharedKey flag, read or not read 24B voor PSKID and Salt
- Generate header & body decryption keys
- Read & decrypt 1-3 bytes from the header (based on if in existing session and if it is a response or a request)
- Decrypt the entire body and parse it according to the type

```toml
version = 1
use_encryption = true
fire_and_forget = false
pre_shared_key = true

psk_id = "<preshared key id base64>"
psk_salt = "<salt base64>"
mac = "<Message Authentication Code base64>"

[header]
packet_type = "Session" # or any other CERTIFICATE, PUT, PATCH etc.
flags = [false, true, false, false] # 4 type-specific packet flags
response_to = 0
```

Client state
```
- Session
	- Session key
	- Client counter
	- Server counter
- Pre-shared keys (Map<id, key, counter, expiration>)
- Received messages (Map<counter, Packet>)
```

Server state
```
- Sessions[]
	- Session key
	- Client counter
	- Server counter
	- Identifications (Map<id, key>)
- Pre-shared keys (Map<id, key, counter, expiration>)
- Subscriptions (advanced data structure)
```

`*` = optional

> If fire-and-forget or using a PSK, a Salt is **required**.

- Header encryption key: `Blake2b_256(ikm: PSK? + Session Key, info: counter (of other party, if present) + 0x00, salt: Salt?)`
- Body encryption key: `Blake2b_256(ikm: PSK? + Session Key, info: counter (of other party, if present) + 0xFF, salt: Salt?)`

> After using a PSK, it should be ROTATED. The PSK counter should be incremented at both sides and the request salt will be used to rotate the PSK like this: `Blake2b_256(ikm: PSK, info: Session key? + counter, salt: Salt)`

## Packet type
The following packet types are supported:
- **0** [CERTIFICATE](#certificate)
- **1** [SESSION](#session)
- **2** [GET](#get)
- **3** [POST](#post)
- **4** [PATCH](#patch)
- **5** [PUT](#put)
- **6** [DELETE](#delete)
- **7** [SUBSCRIBE](#subscribe)
- **8** [UNSUBSCRIBE](#unsubscribe)	 
- **9** [STREAM](#stream)
- **10** [REGISTER](#register)
- **11** [IDENTITY](#identify)
- **12** [PROXY](#proxy)
- 13, 14: reserved for future use
- **15** [ERROR](#error) (only for responses)

## Crypto Settings
```
Crypto settings <flags> (8b-16b)
	--- Encryption algorithms, can be combined ---
	1 EncryptWithChaCha - use ChaCha20Poly1305 as encryption algorithm (default)
	2 EncryptWithAesCTR - use AES CTR as encryption algorithm

	4 LargeHashes - use **32-byte** hashes instead of default **16-byte** hashes
	8 Blake3      - use Blake3 instead of Blake2 for MAC, KDF

	16 SignEd25519 - Sign with Ed25519, default
	32 KeyExX25519 - Key exchange with X25519, default
	64 --- not used ---
	128 UsePostQuantum - use Post Quantum Cryptography algorithms, add another 8 flags (next byte)

	1 SignPQCDsa44 - use smallest PQC signing algorithm
	2 SignPQCDsa65- use larger, more secure PQC signing algorithm (still fast)
	4 SignPQCFalcon - use large, secure PQC siging algorithm (not fast)
	8 SignPQCSlhDsa - use larger, heavier PQC signing algorithm (very slow)

	16 KeyExPQCSmall - use smallest PQC key exchange algorithm
	32 KeyExPQCMedium - use larger, more secure KX algorithm
	64 KeyExPQCLarge - use largest, most secure KX algorithm
	128 --- not used ---
```

## FFI (TODO)
```
send(len: i32, data: Toml);
recv() -> (i32, Toml);
```

## SESSION
- Goal: exchange keys with a server, create a Session Key
> If a PSK id is set in the request, but the PersistKey flag is set in the SESSION request, the old PSK will be **deleted**.

### SESSION Request

```
Header Flags
	- PersistKey: Save key as PSK [returns PSK ID] / include expiration
	- EnableEncryption: Switch to encrypted session	

Body (32-36B+)
	PskExpiration (4B)* [Plabble Timestamp, if Persist Key is set]
	Public/encapsulation keys (fixed) [based on crypto settings in header. Defaults to X25519, 32B]
```

```toml
[header]
packet_type = "Session"
flags = [true, true, false, false]
# flags[0] = persist_key
# flags[1] = enable_encryption

[body]
psk_expiration = 2025-05-27T07:32:00-08:00Z
keys = ["<public key base64>"]
```

> To stop a Session, the user can send a new SESSION request or disconnect from the server.

### SESSION Response

```
Header Flags
	- WithPSK: Includes PSK

Body (>= 96B)
	PskID      			(12B)* [if includes PSK is set]
	Public keys/encapsulated secrets (fixed) [based on algorithm in header. Defaults to X25519, 32B]
	Signatures  		(fixed) [signature of SHA256(plain client request), server keys, secrets. based on algorithm in header. Defaults to ED25519, 64B]
```

```toml
[header]
type = "SESSION"
responseTo = 0
flags.WithPSK = true

[body]
pskId = "pre-shared key id"
keys = ["<x25519 server key base64>"]
signatures = ["<ed25519 server signature base64>"]
```

> The server and client generate a Session Key based on the key exchange: `HKDF_Sha256(ikm: shared secret(s), info: PSK?, salt: Salt?)`. This session key is used in the encryption of the packet headers and body's.

## CERTIFICATE
- Goal: retrieve server certificate, or a certificate stored on this server by ID.

### CERTIFICATE request
```
Header Flags
	- FullCertificateChain: Return full certificate chain. If not set, only return self and parent
	- Challenge: If set, send a challenge in the body that the server should sign
	- Query: If set, query certificate by ID (string)

Body (0B | 32B)
	--- If Query flag is set ---
	IdLength (1B)*
	Id       (variable)*

	Challenge	(32B) [optional, required if Challenge flag is set]
```

```toml
[header]
type = "CERTIFICATE"
flags.Challenge = true
flags.FullCertificateChain = true
flags.Query = true

[body]
id = "server-root"
challenge = "<random base64 16-byte string>"
```

### CERTIFICATE response
```
Body (64+ B):
	Signature  (fixed) [signature of challenge, Session Key (if present) and optionally certificates. Depends on algoritm in header. Defaults to ED25519, 64B]
	Certificates[] (variable)
```

```toml
[header]
type = "CERTIFICATE"
responseTo = 0

[body]
signature = "<ed25519 server signature base64>"

  [[body.certificates]]
  fullCertificate = true
  isRootCertificate = false
  algorithm = "Auto"
  keys = ["<ed25519 public key base64>"]
  validFrom = 2025-05-27T07:32:00Z
  validUntil = 2025-05-27T07:32:00Z
  signatures = ["<ed25519 CA signature base64>"]
  data = "CA=plabble;CN=Root certificate"
  uri = "https://certs.plabble.org/certs/test.cert"
  parent_uri = "plabble://chat.plabble.org/certificate?id=2ibfa3buijq"

  [[body.certificates]]
  fullCertificate = false
  isRootCertificate = true
  hash = "Blake2b_256 hash of certificate"
  uri = "https://certs.plabble.org/certs/root.cert"
```

### Certificate
```
Certificate (>34B | >107B)
	- FullCertificate: flag (1b)
	- IsRootCertificate: flag (1b)
	Algorithm  (4b) [algorithms used in certificate according to Algorithm enum]

	--- if full certificate flag is off
	Hash       (16B) [Blake2b_128 hash of certificate content]

	--- if full certificate flag is on ---
	Signing keys (fixed) [based on algorithms used]
	ValidFrom  (4B)  [Plabble timestamp]
	ValidUntil (4B)  [Plabble timestamp]
	Signature  (fixed) [signature of cert provider, or self-signed if CA. based on algorithms used]
	DataLength (dynint) [certificate data length]
	Data (variable)* [certificate data, string format `CA=...;CN=...`]
	
	--- if not IsRootCertificate ---
	ParentUriLength  (1B)*
	ParentUri        (variable)*

	UriLength  	     (1B)
	Uri              (variable)
```

## PROXY
- Goal: redirect encrypted package to other server using layered encryption
If fire-and-forget, it needs to include the next server address
If a proxy session, include a proxy ID to make linking possible

### PROXY request

```
Header flags
	- InitSession: Initialize session
	- KeepConnection: Keep connection open [can only be used with non-FaF requests]
	- RandomHops: Select random intermediate hops of count n (set by Hops in body)

Body (>=12B | >1B)
	SessionId     (12B)* [optional if FaF or initialize request]
	Keys		  (fixed) [based on algorithms used]

	--- if initialize session, or FaF ---
	AddressLength (1B)*
	URI     (variable)*
	Hops		  (1B)* [only if RandomHops flag is set]

	--- if NOT init session, or IS FaF ---
	Packet  (variable)  [request proxied complete packet]
```

```toml
[header]
type = "PROXY"
flags.InitSession = true
flags.KeepConnection = true
flags.RandomHops = true

[body]
sessionId = "<base64 session id>"
keys = ["<base 64 keys based on algorithm used>"]
uri = "relay.plabble.org"
hops = 3
packet = "<base64 raw, encrypted packet bytes, not TOML>"
```

### PROXY response

```
Header flags
	- IncludeHops: If any random hops were selected, include hops in body

Body (>12B)
	SessionId      (12B)* [optional if FaF]
	HopCount        (1B)* [only if IncludeHops is set]
	HopInfo[] (variable)* [only if IncludeHops is set]
	
	Packet (variable) [response proxied packet]
```

```toml
[header]
type = "PROXY"
flags.IncludeHops = true
responseTo = 0

[body]
sessionId = "<base64 created session id>"
packet = "<base64 raw packet bytes, not TOML>"
hopCount = 1

	[[body.hops]]
	uri = "intermediate-proxy.anydomain.com"
	keys = ["<base64 encoded hop key or encapsulated hop secrets>"]
```

### HopInfo object
```
URILength (1B)
URI     (variable)
Keys    (fixed) [ keys from this hop to decrypt the packets ]
```

## GET
- Goal: retrieve data from bucket, if allowed

### GET request
```
Header flags
	- `BinaryBucket` vBinary keyed bucket (string)
	- `Subscribe` Also subscribe to the bucket [only if not FaF]
	- `RangeUntilMode` Range mode [0 = From, 1 = Until. If from mode, until can be omitted. If until mode, from can be omitted. In every mode omitting both GETs the entire bucket]

Body (16-21B)
	BucketId (16B)

	--- if binary keyed bucket ---
	KeyLength   (1B)* [length of the first key]
	From  (variable)* [string, start key]
	Until (variable)* [string, end key]

	--- if normal bucket ---
	From  (2B)* [u16, start index]
	Until (2B)* [u16, end index]
```

```toml
[header]
type = "GET"
flags.BinaryBucket = false
flags.Subscribe = false
flags.RangeUntilMode = false

[body]
bucketId = "<base64 bucket id>"
from = 0
until = 25_565
```
### GET response (or a future response to a SUBSCRIBE)

```
Header flags
	- Binary keyed bucket (string)

Body (variable, array)
	--- if binary keyed bucket ---
	[
		KeyLength   (1B) [length of the key]
		Key   (variable)
        Length (dynint)
        Content (variable)
	]

	--- if normal bucket ---
	[
		Key (2B) [u16 key]
		Length (dynint)
		Content (variable)
	]
```

```toml
[header]
type = "GET"
flags.BinaryBucket = true
responseTo = 0

[[body]]
key = "plabble-custom-bucket-key"
content = "base64 bucket content"

[[body]]
key = 25_565 # normally a response would not mix binary and non-binary, but this is just an example
content = "base64 bucket content"
```

## POST
- Goal: create a new bucket

### POST request
```
Header flags
	- `BinaryBucket` Binary keyed bucket (string)
	- `DoNotPersist` Do not persist (RAM bucket)
	- `Subscribe` Also subscribe to bucket
	- `RangeUntilMode` Subscription Range mode (see GET,SUBSCRIBE)

Body (variable)
	DesiredBucketId (16B)
	MaxBucketSize (dynint)
	MaxBucketSlotSize (dynint)
	BucketLifetime (4B) [plabble timestamp]
	PermissionsLength (dynint)
	BucketPermissions (variable)

	--- if also subscribe to bucket ---
	
	--- if binary keyed bucket ---
	KeyLength   (1B)* [length of the first key]
	From  (variable)* [string, start key]
	Until (variable)* [string, end key]

	--- if normal bucket ---
	From  (2B)* [u16, start index]
	Until (2B)* [u16, end index]
```

```toml
[header]
type = "POST"
flags.BinaryBucket = true
flags.DoNotPersist = false
flags.Subscribe = false
flags.RangeUntilMode = false

[body]
desiredBucketId = "<bucket id base64>"
maxBucketSize = 12_345
maxSlotSize = 123
bucketLifetime = None # leave away or give date
from = 0
until = 25_565

  [body.permissions]
  # AR|AA|AU|AD|SR|SA|SU|SD|PR|PA|PU|PD|SBD|PBD|DenyExistence
  flags.AnonymousRead = true # AR
  flags.AnonymousAdd = false # AA
  flags.ProtectedAppend = true # PA
  # etc.

    [body.permissions.acl]
    append = [ "user ID", "other user ID" ] # PA list
    update = [ "user ID", "other user ID" ]
    delete = [ "user ID", "other user ID" ]
```

### POST response
```
No response body
```

## Bucket permissions
Access control for a bucket
If ACL is disabled, "private" means `bucket key owner`

Variants:
- **Anonymous**: without authentication
- **Secret**: with secret bucket key authentication
- **Protected**: using an Access Control List with authenticated users

```
Bucket flags (2B)
	1     `AR` Allow anonymous read
 	2     `AA` Allow anonymous append
	4     `AU` Allow anonymous update
	8     `AD` Allow anonymous delete
	16    `SR` Allow secret    read
	32    `SA` Allow secret    append
	64    `SU` Allow secret    update
	128   `SD` Allow secret    delete
	256   `PR` Allow protected read
	512   `PA` Allow protected append
	1024  `PU` Allow protected update
	2048  `PD` Allow protected delete
	4096  `SBD` Allow private   bucket deletion
	8196  `PBD` Allow protected bucket deletion
	16384 `DenyExistence` Deny bucket existence for anonymous
	32768 not used
	
Access control lists (>2B)
	PRListLength (2B)* [only if flag protected read is set]
	PRList       (N * 20B)* [list of allowed users on ACL, Blake2b_160 hash of public key = userID]
	
	PAListLength (2B)* [only if flag protected append is set]
	PAList       (N * 20B)*
	
	PUListLength (2B)* [only if flag protected update is set]
	PUList       (N * 20B)*
	
	PDListLength (2B)* [only if flag protected delete is set]
	PDList       (N * 20B)*
```

## PATCH
- Goal: update bucket permissions (access control list)
> You can update multiple lists with the same ID at the same time.
The server will avoid duplicates and in case of removal will ignore if not exists.

### PATCH request
```
Body (>=18B)
	BucketId (16B)
	Modes      (1B, flags) [AddPR, RemovePR, AddPA, RemovePA, AddPU, RemovePU, AddPD, RemovePD]
	ListLength (1B) [if set to 0 and mode is "remove", CLEAR the list]
	List       (ListLength * 20B)
```

```toml
[header]
type = "PATCH"

[body]
bucketId = "bucket id base64"

# AddPR|RemovePR|AddPA|RemovePA|AddPU|RemovePU|AddPD|RemovePD
modes.AddPR = true
modes.RemovePR =true
# etc.

list = [ "<user ID>", "<other user ID>" ]
```

### PATCH response
```
No response data
```

## PUT
- Goal: put or append data to one or more indices

### PUT request
```
Header flags
	- `BinaryBucket` Binary keyed bucket (string)
	- `Append` Append mode [without keys]
	- `WithKeys` Append-with-keys mode [send keys, but fail if taken]
	- `Subscribe` Also subscribe to given keys

Body (variable)
	BucketId (16B)
	
	--- if binary keyed bucket ---
	[
		KeyLength   (1B)* [length of the key, if with keys]
		Key   (variable)* [if with keys]
        Length (dynint)
        Content (variable)
	]

	--- if normal bucket ---
	[
		Key (2B) [u16 key]* [if with keys]
		Length (dynint)
		Content (variable)
	]
```

```toml
[header]
type = "PUT"
flags.BinaryBucket = false
flags.Append = false
flags.WithKeys = true
flags.Subscribe = true

[body]
bucketId = "<bucket id base64>"

[[body.slots]]
key = "slot key"
content = "update slot content"

[[body.slots]]
key = 5
content = "update slot content"

```
### PUT response
```
No response data
```

## DELETE
- Goal: delete entires from bucket or entire bucket

### DELETE request
```
Header flags
	- `BinaryBucket` Binary keyed bucket (string)
    - `RangeUntilMode` Range mode [0 = From, 1 = Until. If from mode, until can be omitted. If until mode, from can be omitted. In every mode omitting both DELETEs the entire bucket]

Body
	BucketId (16B)

	--- if binary keyed bucket ---
	KeyLength   (1B)* [length of the first key]
	From  (variable)* [string, start key]
	Until (variable)* [string, end key]

	--- if normal bucket ---
	From  (2B)* [u16, start index]
	Until (2B)* [u16, end index]
```

```toml
[header]
type = "DELETE"
flags.BinaryBucket = false
flags.RangeUntilMode = false

[body]
bucketId = "<bucket id base64>"
from = 0
until = 20
```

### DELETE response
```
No response data
```

## SUBSCRIBE
- Goal: subscribe to entire bucket or to key range

### SUBSCRIBE request
```
Header flags
	- Binary keyed bucket (string)
    - Range mode [0 = From, 1 = Until. If from mode, until can be omitted. If until mode, from can be omitted. In every mode omitting both SUBSCRIBEs to the entire bucket]

Body
	BucketId (16B)

	--- if binary keyed bucket ---
	KeyLength   (1B)* [length of the first key]
	From  (variable)* [string, start key]
	Until (variable)* [string, end key]

	--- if normal bucket ---
	From  (2B)* [u16, start index]
	Until (2B)* [u16, end index]
```

```toml
[header]
type = "SUBSCRIBE"
flags.BinaryBucket = false
flags.RangeUntilMode = false

[body]
bucketId = "<bucket id base64>"
from = 0
until = 20
```

### SUBSCRIBE response
```
No response data
```

## UNSUBSCRIBE
- Goal: delete subscription range(s) for a bucket

### UNSUBSCRIBE request
```
Header flags
	- Binary keyed bucket (string)
    - Range mode [0 = From, 1 = Until. If from mode, until can be omitted. If until mode, from can be omitted. In every mode omitting both UNSUBSCRIBEs from the entire bucket]

Body
	BucketId (16B)

	--- if binary keyed bucket ---
	KeyLength   (1B)* [length of the first key]
	From  (variable)* [string, start key]
	Until (variable)* [string, end key]

	--- if normal bucket ---
	From  (2B)* [u16, start index]
	Until (2B)* [u16, end index]
```

```toml
[header]
type = "UNSUBSCRIBE"
flags.BinaryBucket = false
flags.RangeUntilMode = false

[body]
bucketId = "<bucket id base64>"
from = 0
until = 20

```
### UNSUBSCRIBE response
```
No response data
```

## STREAM
- Goal: PUT (append) or GET/subscribe to binary data from a SINGLE bucket slot.

> A STREAM is usefull for media streaming. When in PUT mode, it will only APPEND data
to a single slot instead of overwriting it. 
For people subscribed to a streaming slot using the STREAM (GET-mode) request, they will only receive the _new_/fresh appended bytes.
When joining a stream, you can use the STREAM in GET mode, but with ranges specified.
This will subscribe you to a single bucket slot, if you want that

### STREAM request
```
Header flags
	- Binary keyed bucket (string)
	- StreamAppend: PUT/append mode [if not set, use GET/request mode]
    - Range mode [0 = From, 1 = Until. If from mode, until can be omitted. If until mode, from can be omitted. In every mode omitting both GET the entire bucket]
	- Subscribe: Subscribe to partial updates of the slot [in read mode, subscribe to partial updates to this bucket slot]

Body
	BucketId (16B)

	-- If BinaryKeyed bucket --
	SlotSize (1B)*
	Slot	 (variable)*

	-- If not binary keyed bucket --
	Slot     (2B)*

	--- If PUT mode ---
	Length (dynint)
	Blob   (variable)

	--- If READ mode ---
	From (dynint)*
	To   (dynint)*
```

```toml
[header]
type = "STREAM"
flags.BinaryBucket = false
flags.StreamAppend = true
flags.RangeUntilMode = true
flags.Subscribe = false

[body]
bucketId = "<bucket id base64>"
slot = 50

# in append mode
blob = "<base64 bytes>"

# or, in read mode
from = 28593
to = 52394587

```
### STREAM response
```
Header flags
	- StreamAppend: PUT/append mode [if not set, use GET/request mode]

Body
	--- If PUT mode ---
	NewSlotLength (dynint)

	--- If READ mode
	Length (dynint)
	Blob   (variable)
```

```toml
[header]
type = "STREAM"
flags.StreamAppend = true
responseTo = 0

[body]
# in append mode
newSlotLength = 98275938

# or, in read mode
blob = "base64 byters"
```

## IDENTIFY
- Goal: prove identity to server by sending a certificate (chain) for your Plabble account, offered by your home server. Offer public key for ACL's of a bucket
> The server SHOULD NOT persist identities, they SHOULD be stored in RAM. Identity is used for sessions, not for FaF. Server checks if timestamp is not too far away.
The server should decide how long an identity will be accepted. The identity should be bound to the session

### IDENTIFY request
```
Body
	Signature      (fixed) [signature of server address, timestamp, certificates. Based on Algorithm header]
	Timestamp      (4B)  [Plabble timestamp, UTC]
	Certificates[] (variable) [first certificate is always cert owner]
```

```toml
[header]
type = "IDENTIFY"

[body]
signature = "<base64 server address + timestamp signature>"
timestamp = 2025-05-27T07:32:00Z

	[[body.certificates]]
	fullCertificate = false
	hash = "Blake2b_256 hash of certificate"
	uri = "https://certs.plabble.org/certs/root.cert"

	# etc.
```

### IDENTIFY response
```
Empty response body
```

## REGISTER
- Goal: Create a new identity on the server by providing a public key and some information about you
> Not all servers might accept this request. Some might want another onboarding process, for instance by sending you a certificate in another way, or to apply some screening.
This request SHOULD NOT be accepted without encryption!

### REGISTER request
```
Body
	Signing keys (fixed) [new, random generated siging key. Size based on algorithm header]
	Claims     (variable) [claims to be put in the certificate, in the format `CA=...;CN=...`]
```

```toml
[header]
type = "REGISTER"

[body]
keys = ["<base 64 signing key>"]
claims = "USERNAME=henk;AGE=12" # or whatever claims the server or protocol requires
```

### REGISTER response
```
Body
	Certificate (variable) [The new, generated certificate, signed by the server]
```

```toml
[header]
type = "REGISTER"
responseTo = 0

[body.certificate]
fullCertificate = true
isRootCertificate = false
keys = ["<public key base64>"]
validFrom = 2025-05-27T07:32:00Z
validUntil = 2025-05-27T07:32:00Z
signatures = ["<CA signature base64>"]
data = "CA=plabble;CN=Root certificate"
uri = "https://certs.plabble.org/certs/test.cert"
parent_uri = "plabble://chat.plabble.org/certificate?id=2ibfa3buijq"
```

## ERROR
- Goal: for any request, indicate that an error occured
> Error requests are NOT accepted.

ERROR response
```
Body (>=2B)
	Code (2B) [u16 status code]
	Message (variable) [string]
```

```toml
[header]
type = "ERROR"
responseTo = 0

[body]
code = 404
message = "Bucket not found"
```

### ERROR codes
```
- 1xx: Informational - Request received, continuing process   
- 2xx: Success - The action was successfully received, understood, and accepted - 3xx: Redirection - Further action must be taken in order to complete the request
- 4xx: Client Error - The request contains bad syntax or cannot be fulfilled
- 5xx: Server Error - The server failed to fulfill an apparently valid request 
```

- `400 Bad Request` - Wrong packet format
- `401 Authentication Failed` - Not allowed without authentication (AD), or authentication failed (bucketKey)
- `402 Payment Required` - You need a paid account to use this server (i.e. disk quota)
- `403 Forbidden` - You're not on the ACL, or not allowed to perform this action
- `404 Not Found` - Bucket not found, index not found
- `405 Method Not Allowed` - Bucket doesn't support this operation
- `406 Not Acceptable` - When using a key that is too large or something like that (PoH-only)
- `408 Request Timeout` - Not able to access database etc.
- `409 Conflict` - When appending with index to existing key, or creating a bucket with an existing ID
- `412 Precondition failed` - Certificate not available or unsupported algorithm
- `413 Content Too Large` - Request too large, or bucket/your account is out of space
- `415 Unsupported Media Type` - When putting non-UTF8 data to a string (or putting JSON to PoH)
- `416 Range Not Satisfiable` - When doing a STREAM request that is out of the bounds of the binary size of the bucket slot
- `418 I'm a teapot` - I'm not a normal HTTP server (PoH-only), or unsupported server version
- `428 Precondition failed` - Couldnt upgrade to encrypted connection
- `429 Too Many Requests` - Rate limiting

- `500 Internal Server Error` - Something went wrong
