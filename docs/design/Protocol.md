# Engage UX Network Protocol

Engage UX supports multiple connection types, including TCP/IP, Unix Sockets, XPC, AIDL, and more. No matter which connection is being used, the protocol for the data packets is identical.

## Common Header

Both the request and response start with a common 56 byte header. It is composed of:

- A CRC-32 of the remainder of the message
- The security token of the caller
- A UUID assigned by the caller to the request, and used to correlate the replies to specific requests
- A byte indicating the length of the _data_ portion of the message

```mermaid
packet-beta
	0-31: "CRC-32"
	32-39: "Security token"
	40-55: "Request UUID"
	56: "Data length"
```

## Request Format

After the common header, each request contains the following:

- A vendor specific ID indicating what extension operation code set to use. The core uses the ID zero (0x00).
- A code indicating the desired operation
- Any data required by the operation

```mermaid
packet-beta
	0-56: "Common header"
	57-60: "Vendor ID (32 bit)"
	61-64: "Operation Code (32 bit)"
	65-320: "Data..."
```

## Reply Format

Responses contain the following after the header:

- A zero (0x00) indexed sequence number, allowing multipart responses
- The data for the indicated portion of the response

```mermaid
packet-beta
	0-54: "Common header"
	55: "Sequence Number"
	56-311: "Data..."
```
