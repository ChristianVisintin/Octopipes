# Octopipes

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![HitCount](http://hits.dwyl.io/ChristianVisintin/Octopipes.svg)](http://hits.dwyl.io/ChristianVisintin/Octopipes) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Build](https://api.travis-ci.org/ChristianVisintin/Octopipes.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Octopipes) [![codecov](https://codecov.io/gh/ChristianVisintin/Octopipes/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Octopipes)

Just an IPC server based on pipes.

Developed by *Christian Visintin*

Currently under development

---

- [Octopipes](#octopipes)
  - [Introduction](#introduction)
  - [Supported platforms and Requirements](#supported-platforms-and-requirements)
    - [Server Requirements](#server-requirements)
    - [Client libraries](#client-libraries)
  - [How it works](#how-it-works)
    - [The Subscription Step](#the-subscription-step)
    - [The Assignment Step](#the-assignment-step)
    - [The IPC Step](#the-ipc-step)
    - [The Unsubscribption Step](#the-unsubscribption-step)
    - [Entire lifecycle](#entire-lifecycle)
  - [Client implementation](#client-implementation)
    - [C Client Implementation](#c-client-implementation)
  - [Octopipes Protocol](#octopipes-protocol)
    - [Payload Syntax](#payload-syntax)
    - [Common Access Pipe Protocol](#common-access-pipe-protocol)
      - [Subscribe](#subscribe)
      - [Assignment](#assignment)
      - [Unsubscribe](#unsubscribe)
      - [CAP Errors](#cap-errors)
  - [Know Issues](#know-issues)
  - [Contributing](#contributing)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

Octopipes is a server application which improves and organize the good old named pipes to implement and handle Inter Process Communication (IPC).
Octopipes uses named pipes (or FIFO) to exchange data between hosts. Each process subscribes to the Octopipes server with its own ID and optionally with a group ID and then the server assigns to it a pipe to use to communicate with other processes. The messages must be sent using the Octopipe protocol as defined.

These are the main features Octopipes provides:

- Fast and reliable IPC
- Message exchanging between individual processes and groups
- Free data format

## Supported platforms and Requirements

Supported Operating Systems:

- GNU Linux
- MacOS

### Server Requirements

- Rust compiler
- Cargo

### Client libraries

| Language | Library        | Requirements                         | Supported Platforms                           | Repository                                       |
|----------|----------------|--------------------------------------|-----------------------------------------------|--------------------------------------------------|
| C        | liboctopipes   | C compiler (GNU gcc, Clang), CMake   | GNU Linux, MacOS, Windows (MinGW **not yet**) | <https://github.com/ChristianVisintin/Octopipes> |
| C++      | liboctopipespp | C++ compiler (GNU g++, Clang), CMake | GNU Linux, MacOS, Windows (MinGW **not yet**) | <https://github.com/ChristianVisintin/Octopipes> |
| Rust     | rustypipes     | rustc, cargo                         | GNU Linux, MacOS, Windows                     | <https://github.com/ChristianVisintin/Octopipes> |

## How it works

Before talking of the working flow of Octopipes let's introduce what Octopipes is.
Octopipes is, indeed, an IPC server which takes care of:

- Listening for incoming client subscription requests.
- Assigning to each client two named pipes, one for transmission and one for reception.
- Listening on client TX pipe and dispatch the messages to their recipients through their RX pipe.

Octopipes working flow is pretty much simple and consists of a 4 steps lifecycle.

1. [Subscription](#the-subscription-step)
2. [Assignment](#the-assignment-step)
3. [IPC](#the-ipc-step)
4. [Unsubscription](#the-unsubscribption-step)

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

## Client implementation

### C Client Implementation

Dependencies:

- pthread
- liboctopipes

Initialize the client

```c
OctopipesClient* client;
if ((rc = octopipes_init(&client, client_id, cap_path, protocol_version)) != OCTOPIPES_ERROR_SUCCESS) {
  //Handle error
}
```

Set callbacks (optional)

```c
octopipes_set_received_cb(client, on_received);
octopipes_set_receive_error_cb(client, on_error);
octopipes_set_sent_cb(client, on_sent);
octopipes_set_subscribed_cb(client, on_subscribed);
octopipes_set_unsubscribed_cb(client, on_unsubscribed);

//All callbacks takes in an OctopipesClient; received and sent takes also an OctopipesMessage*, while receive_error the returned error from receive:

OctopipesError octopipes_set_received_cb(OctopipesClient* client, void (*on_received)(const OctopipesClient* client, const OctopipesMessage*));
OctopipesError octopipes_set_sent_cb(OctopipesClient* client, void (*on_sent)(const OctopipesClient* client, const OctopipesMessage*));
OctopipesError octopipes_set_receive_error_cb(OctopipesClient* client, void (*on_receive_error)(const OctopipesClient* client, const OctopipesError));
OctopipesError octopipes_set_subscribed_cb(OctopipesClient* client, void (*on_subscribed)(const OctopipesClient* client));
OctopipesError octopipes_set_unsubscribed_cb(OctopipesClient* client, void (*on_unsubscribed)(const OctopipesClient* client));
```

Subscribe to server

```c
OctopipesCapError cap_error;
if ((rc = octopipes_subscribe(client, groups, groups_amount, &cap_error)) != OCTOPIPES_ERROR_SUCCESS) {
  //Handle error
  //Check also OctopipesCapError here
}
```

Start loop (after that you will start receiving messages through the on_received callback):

```c
if ((rc = octopipes_loop_start(client)) != OCTOPIPES_ERROR_SUCCESS) {
  //Handle error
}
```

Send messages

```c
if ((rc = octopipes_send(client, remote, (void*) data, data_size)) != OCTOPIPES_ERROR_SUCCESS) {
  //Handle error
}
```

Unsubscribe

```c
if ((rc = octopipes_unsubscribe(client)) != OCTOPIPES_ERROR_SUCCESS) {
  //Handle error
}
```

Free resources

```c
octopipes_cleanup(message);
```

Two clients implementation can be found in and are provided with liboctopipes [Here](https://github.com/ChristianVisintin/Octopipes/tree/master/libs/liboctopipes/clients)

## Octopipes Protocol

### Payload Syntax

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

### Common Access Pipe Protocol

In the previous chapter we’ve seen how a standard packet is encoded in Octopipes, but we still don’t know how to communicate using the Common Access Pipe (CAP), which as we’ve seen before the pipe where hosts subscribe and unsubscribe.
The CAP requires a special sub-protocol to communicate properly with octopipes server. The packet is encapsulated into DAT (data field) of the octopipes standard packet.
Since the CAP is unique, if multiple clients tries at the same time to read the CAP (which is anyway very uncommon), the client which received a wrong packet must ignore the incoming packet and return and error.
Each Message has a different payload, let's see the them in details:

#### Subscribe

The subscribe packet is sent by the host to the server and it’s the request of subscribing to octopipes server. The server will respond with an ASSIGNMENT packet. It can also be used as a resubscription to new groups. It’s important to understand processes are groups too. Each process is subscribed implicitly to its group. The process name used is the one set in the packet at LND.

| Name | Syntax | Length (bytes) | Description                                                                                                                                                          |
|------|--------|----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| OBJ  | 0x01   | 1              | **Object**: Indicates this is a SUBSCRIBE message                                                                                                                    |
| GRP  | uint8  | 1              | **Groups**: amount of groups the node is registered to. After this record a GRP amount of GSZn and GNMn rows follow. If GRP is 0, no other GSZn/GNMn record follows. |
| GSZn | uint8  | 1              | **Group Size n**: Describes the size of group n name (1 => GRP)                                                                                                      |
| GNMn | char   | GSZn           | Group name: Describes the group name the host is registered to. *The group name mustn’t include the null terminator, which must be added by the parser*              |

#### Assignment

The assignment packet is transmitted by the server to the client after a subscription request.

| Name | Syntax | Length (bytes) | Description                                                                               |
|------|--------|----------------|-------------------------------------------------------------------------------------------|
| OBJ  | 0xFF   | 1              | **Object**: Indicates this is an ASSIGNMENT message                                       |
| ERR  | uint8  | 1              | **Error**: 0 if the request was accepted, error code otherwise. (See CAP Error reference) |
| LTX  | uint8  | 1              | **Length TX Pipe**: Length of PTX field                                                   |
| PTX  | char   | LTX            | **PTX**: TX Pipe file path                                                                |
| LRX  | uint8  | 1              | **Length RX Pipe**: Length of PRX field                                                   |
| PRX  | char   | LRX            | **PRX**: RX Pipe file path                                                                |

#### Unsubscribe

Tells the server the client has unsubscribed from Octopipes

| Name | Syntax | Length (bytes) | Description                                          |
|------|--------|----------------|------------------------------------------------------|
| OBJ  | 0x02   | 1              | **Object**: Indicates this is an UNSUBSCRIBE message |

#### CAP Errors

| Value | Description                                                                               |
|-------|-------------------------------------------------------------------------------------------|
| 0     | No error                                                                                  |
| 1     | **NAME_ALREADY_TAKEN**: Unable to accept subscription since the ID has already been taken |
| 2     | **FS**: Unable to create FIFO                                                             |

## Know Issues

## Contributing

TBD

## Changelog

## License

```txt
MIT License

Copyright (c) 2019-2020 Christian Visintin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
