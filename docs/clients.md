# Client implementation

- [Client implementation](#client-implementation)
  - [Rust Client Implementation](#rust-client-implementation)
  - [C Client Implementation](#c-client-implementation)
  - [C++ Client Implementation](#c-client-implementation)

## Rust Client Implementation

## C Client Implementation

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
octopipes_cleanup(client);
```

Two clients implementation can be found in and are provided with liboctopipes [Here](https://github.com/ChristianVisintin/Octopipes/tree/master/libs/liboctopipes/clients)

## C++ Client Implementation

Dependencies:

- pthread
- liboctopipespp

Initialize the client

```cpp
octopipes::Client* client = new octopipes::Client(clientId, capPipe, octopipes::ProtocolVersion::VERSION_1);
```

Set callbacks (optional)

```cpp
client->setReceive_errorCB(on_receive_error);
client->setReceivedCB(on_received);
client->setSubscribedCB(on_subscribed);
client->setUnsubscribedCB(on_unsubscribed);
```

Subscribe to server

```cpp
if ((ret = client->subscribe(groups, cap_error)) != octopipes::Error::SUCCESS) {
  //Handle error
}
```

Start loop (after that you will start receiving messages through the on_received callback):

```cpp
if ((ret = client->startLoop() != octopipes::Error::SUCCESS) {
  //Handle error
}
```

Send messages

```cpp
if ((ret = client->send(remote, data, data_size)) != octopipes::Error::SUCCESS) {
  //Handle error
}
```

Unsubscribe

```cpp
if ((ret = client->unsubscribe()) != octopipes::Error::SUCCESS) {
  //Handle error
}
```

Free resources

```cpp
delete client;
```

Two clients implementation can be found in and are provided with liboctopipes [Here](https://github.com/ChristianVisintin/Octopipes/tree/master/libs/liboctopipespp/tests/client/)
