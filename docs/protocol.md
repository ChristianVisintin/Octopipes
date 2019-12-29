# Octopipes Protocol

Protocol Reference V1

- [Octopipes Protocol](#octopipes-protocol)
  - [Protocol Reference](#protocol-reference)
  - [Protocol Ruleset](#protocol-ruleset)
    - [General ruleset](#general-ruleset)
    - [Server ruleset](#server-ruleset)
    - [The Subscription Step](#the-subscription-step)
    - [The Assignment Step](#the-assignment-step)
    - [The IPC Step](#the-ipc-step)
    - [The Unsubscribption Step](#the-unsubscribption-step)
    - [Entire lifecycle](#entire-lifecycle)
  - [Payload Syntax](#payload-syntax)
  - [Common Access Pipe Protocol](#common-access-pipe-protocol)
    - [Subscribption](#subscribption)
    - [Assignment](#assignment)
    - [Unsubscribtion](#unsubscribtion)
    - [CAP Errors](#cap-errors)
  - [List of protocol versions](#list-of-protocol-versions)

## Protocol Reference

The protocol implements a workflow of 4 steps, each one is described in detail in the following chapters:

- [Subscription](#the-subscription-step)
- [Assignment](#the-assignment-step)
- [IPC](#the-ipc-step)
- [Unsubscription](#the-unsubscribption-step)

In addition to these steps, the protocol has also a set of rules for both client and servers.

## Protocol Ruleset

### General ruleset

1. All clients must be able to communicate through two different pipes, on to transmit and one where receive data. All the clients must listen for incoming messages on the RX pipe and if they want to transit data, they must use the TX pipe.
2. All clients must identify themselves with a name, which is a set of ASCII printable characters, alphanumericals are preferred. An empty name (0 length id) is allowed and must be used only by the server.
3. ACK can be required by a client, the request of an ACK can be required through the RCK flag in the message. Follows obligations for the two endpoints:
   1. Each endpoint which receives a message with an RCK must send immediately after parsing the message an ACK to the origin of the message, unless if the endpoint is the server.
   2. The server doesn't have to respond to RCK.
   3. The client mustn't request the server for ACK
   4. The client which requires ACK must handle acknowledgements internally and by itself. No developer who implements an Octopipes library has to implement logic to handle ACKs (except sending ACK when RCK is required), but can implement it as an optional feature.
4. Clients mustn't keep the CAP busy when not waiting for a server response.
5. Clients must be subscribed to communicate with each others.
6. Clients must unsubscribe when IPC is no more required.

### Server ruleset

1. The server at start, must create and open the CAP.
2. The server must be always listening for incoming messages on the CAP. When a message is received on the CAP the server must parse the client request and, if it is a valid request, it must try to satisfy it. Once the request has been worked, the server must reply to the client (unless for unsubscriptions).
3. The server must track subscriptions for each client which has subscribed.
4. For each subscription which has been accepted, the server must prepare a TX pipe and a RX pipe, once created the server must start a worker which will listen for incoming messages on the TX pipe (the TX pipe is where the client writes).
5. For each message received from a client, the server must check each client's subscritpion list and send the message to each client subscribed to the remote described in the message received from the client.

### The Subscription Step

The subscription step consists in requesting to Octopipes a TX pipe and RX pipe to communicate with other processes. The process which requests a subscription will be also able to subscribe to a group or to a process.
What are groups and what are processes? Well, actually they're the same thing for some point of views, but in general they're very similiars.

- **Process**: A process is a client, it identifies itself with an ASCII id. Each process is **implicitly** subscribed to itself and can be subscribed to other groups or processes.
- **Group**: A group is a shared name used by different processes to share messages with more than one process (or even none). A process should not expect any response when using groups. Groups should be used to notify other process of something, more than to communicate (communication: answer and response).

### The Assignment Step

The assignment step, which is carried by the server, consists in:

- checking if a client with that ID already exists. If it does a CAP error is returned.
- assigning the TX and RX pipes to the client (which means in creating, start listening to the TX pipe, encoding it in the assignment message)
- registering the client subscription to the susbcription lookup table.

### The IPC Step

The IPC step covers the entire time the client communicates with the other nodes.
During this time the client doesn't use the CAP (unless it wants to change its subscriptions, but in that case it would just restart from the subscription step).

### The Unsubscribption Step

The unsubscription step is the last step in the lifecycle and consists in the client requesting to unsubscribe. These are the sub steps of the unsubscription process:

1. The client sends the unsubscription request
2. The client stops the RX polling thread
3. The client terminates
4. The Server removes the client's pipes
5. The Server removes the subscription records associated to the client

### Entire lifecycle

So let's explain this with an example

1. A process starts with pid xxxx, it initializes its Octopipes client with ID 'foo'.
2. foo sends a subscription request through the CAP; foo wants to subscribe to 'BROADCAST' group too.
3. Octopipes receives the subscription request from 'foo', it checks if a process 'foo' is already subscribed. Octopipes subscribes foo to 'foo' and to 'BROADCAST', eventually it creates a TX pipe and an RX pipe for 'foo' and sends back the assignment packet to it.
4. Another process starts with pid yyyy, it initializes its Octopipes client with ID 'bar'.
5. bar sends a subscription request through the CAP.
6. Octopipes receives the subscription request from 'bar', it checks if a process 'bar' is already subscribed. Octopipes subscribes foo to 'bar', eventually it creates a TX pipe and an RX pipe for 'bar' and sends back the assignment packet to it.
7. We've got two processes 'foo' and 'bar'. foo is subscribed to 'BROADCAST' group, while both of them are **implitcitly** subscribed to themselves.
8. foo sents a message to bar through its TX pipe. It achieves this setting remote to 'bar' (and origin to 'foo', but this should be automatically handled by the library, not by the user).
9. Octopipes checks who is subscribed to 'bar'. Bar obviously, but maybe someone else (not here though).
10. Octopipes sends the message to bar on its RX pipe.
11. bar receives the message on its RX pipe.
12. A new process 'jupiter' subscribes to group 'BROADCAST'.
13. jupiter sends a message to group 'BROADCAST' "Hello everybody" through its TX pipe.
14. Octopipes checks who is subscribed to 'BROADCAST'. It finds out that both jupiter and foo are subscribed to it; since jupiter is also the origin, it is excluded from the recipients.
15. Octopipes sends the message to foo RX pipe.
16. foo received "Hello Everybody" from jupiter on its rx pipe.
17. foo wants to terminate, so it sends an unsubscribe message through the CAP.
18. Octopipes deletes foo from its subscription records and its TX and RX pipes.
19. foo terminates

## Payload Syntax

This payload describes how the different hosts communicate, once the header is encoded, data can contain anything and its content is not Octopipes’ business.
The payload for Octopipes protocol is a series of byte, witten in MSB notation, made of a header and of a payload. The header has a variable byte size (depends on nodes names) and must have the following syntax:

| Name | Syntax | Length (bytes) | Description                                                                                                                                        |
|------|--------|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| SOH  | 0x01   | 1              | **Start of header**: starts an OPP packet                                                                                                          |
| VER  | uint8  | 1              | **Version**: Describes the protocol version associated to this packet.                                                                             |
| LNS  | uint8  | 1              | **Local Node Size**: Describes the size in bytes of Local Node (LND). 0 is reserved for the server                                                 |
| LND  | uint8  | LNS            | **Local Node**: The name of the node which sent the packet.                                                                                        |
| RNS  | uint8  | 1              | **Remote Node Size**: Describes the size in bytes of the Remote Node (RNS). 0 is reserved for the server                                           |
| RND  | uint8  | RNS            | **Remote Node**: The name of the node or of the group the message is sent to                                                                       |
| TTL  | uint8  | 1              | **Time to live**: Time in seconds after that the fifo must be flushed if no endpoint has read the message                                          |
| DSZ  | uint64 | 8              | **Data Size**: Size of data in bytes                                                                                                               |
| OPT  | uint8  | 1              | **Options**: bit mask for options; starting from the msb (RCK: requires AC, ACK: is an ACK message, ICK: ignore checksum, RFU, RFU, RFU, RFU, RFU) |
| CHK  | uint8  | 1              | **Checksum**: Indicates integrity of data. Its value is calculated as XOR between each value of header (from SOH to ETX included)                  |
| STX  | 0x02   | 1              | **Start Of Text**: indicates the start of data                                                                                                     |
| DAT  | uint8  | DSZ            | **Data**: payload of the message. Its size must match DSZ value                                                                                    |
| ETX  | 0x03   | 1              | **End Of Text**: Indicates the packet has ended

## Common Access Pipe Protocol

In the previous chapter we’ve seen how a standard packet is encoded in Octopipes, but we still don’t know how to communicate using the Common Access Pipe (CAP), which as we’ve seen before the pipe where hosts subscribe and unsubscribe.
The CAP requires a special sub-protocol to communicate properly with octopipes server. The packet is encapsulated into DAT (data field) of the octopipes standard packet.
Since the CAP is unique, if multiple clients tries at the same time to read the CAP (which is anyway very uncommon), the client which received a wrong packet must ignore the incoming packet and return and error.
Each Message has a different payload, let's see the them in details:

### Subscribption

The subscription packet is sent by the host to the server and it’s the request of subscribing to octopipes server. The server will respond with an ASSIGNMENT packet. It can also be used as a resubscription to new groups. It’s important to understand processes are groups too. Each process is subscribed implicitly to its group. The process name used is the one set in the packet at LND.

| Name | Syntax | Length (bytes) | Description                                                                                                                                                          |
|------|--------|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| OBJ  | 0x01   | 1              | **Object**: Indicates this is a SUBSCRIPTION message                                                                                                                    |
| GRP  | uint8  | 1              | **Groups**: amount of groups the node is registered to. After this record a GRP amount of GSZn and GNMn rows follow. If GRP is 0, no other GSZn/GNMn record follows. |
| GSZn | uint8  | 1              | **Group Size n**: Describes the size of group n name (1 => GRP)                                                                                                      |
| GNMn | char   | GSZn           | Group name: Describes the group name the host is registered to. *The group name mustn’t include the null terminator, which must be added by the parser*              |

### Assignment

The assignment packet is transmitted by the server to the client after a subscription request.

| Name | Syntax | Length (bytes) | Description                                                                               |
|------|--------|----------------|-------------------------------------------------------------------------------------------|
| OBJ  | 0xFF   | 1              | **Object**: Indicates this is an ASSIGNMENT message                                       |
| ERR  | uint8  | 1              | **Error**: 0 if the request was accepted, error code otherwise. (See CAP Error reference) |
| LTX  | uint8  | 1              | **Length TX Pipe**: Length of PTX field                                                   |
| PTX  | char   | LTX            | **PTX**: TX Pipe file path                                                                |
| LRX  | uint8  | 1              | **Length RX Pipe**: Length of PRX field                                                   |
| PRX  | char   | LRX            | **PRX**: RX Pipe file path                                                                |

### Unsubscribtion

Tells the server the client has unsubscribed from Octopipes

| Name | Syntax | Length (bytes) | Description                                          |
|------|--------|----------------|------------------------------------------------------|
| OBJ  | 0x02   | 1              | **Object**: Indicates this is an UNSUBSCRIPTION message |

### CAP Errors

| Value | Description                                                                               |
|-------|-------------------------------------------------------------------------------------------|
| 0     | No error                                                                                  |
| 1     | **NAME_ALREADY_TAKEN**: Unable to accept subscription since the ID has already been taken |
| 2     | **FS**: Unable to create FIFO                                                             |

## List of protocol versions

When new version will be added, here a table of different protocol with support state will be added
