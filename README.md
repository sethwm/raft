# Overview
This project is serving a few main goals.  
1. play around with [Raft](https://raft.github.io/).
2. play around with languages.  
3. play around with raspberry pi.
4. play around with wire protocols (stretch goal)

The idea here is to build basic Raft implementations in various languages, perhaps some day having the different implementations actually cooperate in a single heterogenous Raft cluster.

This toy application is responsible for choosing colors.  The color selected will be output to stdout, but also echoed to a udp server which can set the LED on the raspberry pi [sense hat](https://www.raspberrypi.org/products/sense-hat/).  That part is optional (stdout suffices, albeit is boring).  Eventually the UDP server may be replaced with language specific drivers for the sense hat (see goal 3 above).

Unless otherwise specified, everything in this repository is covered by the MIT License.

# Wire Protocol
## LED Server
The LED server will listen to UDP messages in the form of:

```
1,2,3,4,5,0
```

which will set pixel `(1,2)` to `[r,b,g]` value `[3,4,5]` with "blink" set to 0.  Blink will be set to 1 for the leader to make it obvious that there is a single leader.

## Inter-cluster messages
The Raft participants can send messages to each other with the following format:

TBD

# Discovery
Initially the participants will find each other via a hard-coded port range: `[9900-9963]`.  Participants will know their ip and port at startup and guess everyone else's.  They will assume the same ip address for their peers as for themselves.

# Advanced

## setup

* [Raspberry Pi Model 3B](https://www.raspberrypi.org/products/raspberry-pi-3-model-b/)
* [sense hat](https://www.raspberrypi.org/products/sense-hat/)

## iptables
Play around with firewalls and watch the pixels go stale until majority is crossed, then the whole thing falls apart.

```
iptables -I INPUT -p udp  --dport $PORT_TO_BLOCK -j DROP
```