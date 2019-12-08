# Octopipes

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![HitCount](http://hits.dwyl.io/ChristianVisintin/Octopipes.svg)](http://hits.dwyl.io/ChristianVisintin/Octopipes) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Octopipes.svg)](https://github.com/ChristianVisintin/Octopipes) [![Build](https://api.travis-ci.org/ChristianVisintin/Octopipes.svg?branch=master)](https://github.com/ChristianVisintin/Octopipes)

Just an IPC server based on pipes.

Developed by *Christian Visintin*

Currently under development

---

- [Octopipes](#octopipes)
  - [Introduction](#introduction)
  - [How it works](#how-it-works)
  - [Client implementation](#client-implementation)
  - [Octopipes Protocol](#octopipes-protocol)
    - [Common Access Pipe Protocol](#common-access-pipe-protocol)
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

## How it works

TBD

## Client implementation

TBD

## Octopipes Protocol

Defined, but not public yet.

### Common Access Pipe Protocol

Defined, but not public yet.

## Know Issues

## Contributing

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
