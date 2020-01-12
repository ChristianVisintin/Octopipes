# Octopipes

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Build](https://api.travis-ci.org/ChristianVisintin/Octopipes.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Octopipes) [![codecov](https://codecov.io/gh/ChristianVisintin/Octopipes/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Octopipes)

Just an IPC server based on pipes.

Developed by *Christian Visintin*

Currently under development (Scheduled release: February 2020)

---

- [Octopipes](#octopipes)
  - [Introduction](#introduction)
    - [How it works](#how-it-works)
  - [Project Tree](#project-tree)
  - [Octopipes Server](#octopipes-server)
  - [Octopipes Clients](#octopipes-clients)
  - [Octopipes libraries](#octopipes-libraries)
    - [Client implementation](#client-implementation)
    - [Server implementation](#server-implementation)
  - [Octopipes Protocol](#octopipes-protocol)
  - [Know Issues](#know-issues)
  - [Upcoming features](#upcoming-features)
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

### How it works

Before talking of the working flow of Octopipes let's introduce what Octopipes is.
Octopipes is, indeed, an IPC server which takes care of:

- Listening for incoming client subscription requests.
- Assigning to each client two named pipes, one for transmission and one for reception.
- Listening on client TX pipe and dispatch the messages to their recipients through their RX pipe.

Octopipes working flow is pretty much simple and consists of a 4 steps lifecycle.

1. Subscription
2. Assignment
3. IPC
4. Unsubscription

Each client can subscribe to groups or other clients for receiving messages associated to that node. The client, once subscribed, receives a pipes where to listen for incoming messages and one where to transmit to other clients.

The entire protocol ruleset/workflow/reference can be found [here](docs/protocol.md)

---

## Project Tree

- root
  - docs: Octopipes documentation pages
  - libs: [official Octopipes libraries](#octopipes-libraries))
  - octopipes-clients: Test clients to quickly send to and listen for messages from your octopipes server
  - octopipes-server: contains the Octopipes Server

---

## Octopipes Server

You can find the entire Octopipes Server documentation [Here](octopipes-server/README.md)

## Octopipes Clients

Octopipes comes with two clients, one to send and one to receive messages.
The clients are provided both with liboctopipes and with the cargo crate octopipes-clients.

For more details:

- [octopipes-clients](octopipes-clients/README.md)
- [C-clients](libs/liboctopipes/README.md)

---

## Octopipes libraries

Here follows a list of all the available libraries to implement Octopipes clients and servers

| Language | Library        | Requirements                         | Supported Platforms                           | Server/Client | Repository                                       |
|----------|----------------|--------------------------------------|-----------------------------------------------|---------------|--------------------------------------------------|
| C        | liboctopipes   | C compiler (GNU gcc, Clang), CMake   | GNU Linux, MacOS, Windows (MinGW **not yet**) | Both          | <https://github.com/ChristianVisintin/Octopipes> |
| C++      | liboctopipespp | C++ compiler (GNU g++, Clang), CMake | GNU Linux, MacOS, Windows (MinGW **not yet**) | Both          | <https://github.com/ChristianVisintin/Octopipes> |
| Rust     | rustypipes     | rustc, cargo                         | GNU Linux, MacOS, Windows                     | Both          | <https://github.com/ChristianVisintin/Octopipes> |

### Client implementation

Here you can find how to implement an Octopipes Client by language:

- **C**
  - [liboctopipes](libs/liboctopipes/README.md#client-implementation)
- **C++**
  - [liboctopipespp](libs/liboctopipespp/README.md#client-implementation)
- **Rust**
  - [rustypipes](libs/rustypipes/README.md#client-implementation)

### Server implementation

Here you can find how to implement an Octopipes Server by language:

- **C**
  - [liboctopipes](libs/liboctopipes/README.md#server-implementation)
- **C++**
  - [liboctopipespp](libs/liboctopipespp/README.md#server-implementation)
- **Rust**
  - [rustypipes](libs/rustypipes/README.md#server-implementation)

---

## Octopipes Protocol

[Here](docs/protocol.md) You can find the entire Protocol reference.

---

## Know Issues

---

## Upcoming features

- Windows support
- OctoNetworks (IPC between multiple Octopipes hosts)
- New CAP messages:
  - Get clients

---

## Contributing

TBD

---

## Changelog

---

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
