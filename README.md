# Introduction

This repository provides fluvio client that uses WebSocket connection to communicate with Fluvio cluster.  It is intended to be used in browser environment.   This works with websocket proxy server in the fluvio-ws-proxy crate which is intended to be used for debugging and testing purpose.

For non-browser environment, is suggested to use the native Rust client library. Nevertheless, in this repository, using the `fluvio-ws` crate, you can use the WebSocket client in any environment that supports Rust.

# Demo of WebComponent counter

In the `widget/counter` is a simple Rust WebComponent that streams count event from fluvio topic directly. This is a simple example to show how to use fluvio client in browser environment.

## Run WS proxy

First you need to run the websocket proxy server.  You can do this by running:

```
make -C crates/fluvio-ws-proxy run
```

This will start the proxy server on port 8000.

## Create fluvio topic

Then you need to create fluvio topic.  You can do this by running:

```
fluvio topic create my-counter
```

## Run WebComponent counter

Then run counter using embedded web server by:

```
make -C widgets/counter serve
```

Then open browser and navigate to `http://localhost:7000`.  You should see counter widget.  When you click on the button, it will send event to fluvio topic and you should see the count event in the browser.

To see counter values:
```
fluvio consume my-counter
```
