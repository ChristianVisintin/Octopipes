# Octopipes Clients

[![Crates.io](https://img.shields.io/badge/crates.io-v1.0.0-orange.svg)](https://crates.io/crates/octopipes-clients)

Current Version: 1.0.0 (12/01/2020) ~ Built against rustypipes 0.1.1
Developed by *Christian Visintin*

Octopipes-clients provides two application to quickly send and receive message from an Octopipes server.

```sh
cargo install octopipes-clients
```

- [Octopipes Clients](#octopipes-clients)
  - [Usage](#usage)
    - [Octopipes-send](#octopipes-send)
    - [Octopipes-recv](#octopipes-recv)
  - [Changelog](#changelog)
  - [License](#license)

## Usage

Octopipes Clients comes with two different clients, one to send messages and one to listen for messages.

### Octopipes-send

Octopipes-send is the client to send messages to a certain remote groups through an Octopipes Server.

```txt
Usage: octopipes-send [options]

Options:
    -c, --cap-path <CAP_PATH>
                        Specify CAP path
    -r, --remote <REMOTE>
                        Specify the remote
    -p, --payload <PAYLOAD>
                        Specify the payload to send
    -C, --clid <CLIENT_ID>
                        Specify the client id
    -h, --help          print this help menu
```

- Cap Path: path of the Common Access Pipe used by the Octopipes Server
- remote: the recipient group of your message
- payload: the payload of the message
- clid: The ID of the client, if not specified a random one will be generated

### Octopipes-recv

Octopipes-recv is the client to listen for incoming messages on the groups you want to listen to.

```txt
Usage: octopipes-recv [options] GROUPS

Options:
    -c, --cap-path <CAP_PATH>
                        Specify CAP path
    -a, --count <AMOUNT>
                        Specify the amount of message to receive before
                        terminating (if 0, won't terminate)
    -C, --clid <CLIENT_ID>
                        Specify the client id
    -v, --verbose       Verbose mode prints messages as {ORIGIN} {PAYLOAD}
    -h, --help          print this help menu
```

- Cap Path: path of the Common Access Pipe used by the Octopipes Server
- Count: amount of message to receive before terminating
- clid: The ID of the client, if not specified a random one will be generated
- GROUPS: groups separated by space to listen to

---

## Changelog

---

## License

```txt
MIT License

Copyright (c) 2020 Christian Visintin

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
