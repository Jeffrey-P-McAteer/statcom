
# StatCom

_High-level status communications for LAN devices_

Ever wish you could inform every phone and laptop in visual range to be silent/vibrate only?

Ever wish you could set a timer on a laptop and clear it with your phone in the next room?

How about making an announcement to everyone in the same building such that they could keep a copy of the announcement?


StatCom is a high-level status communications tool capable of linking all machines on a LAN together
and letting them act as a cohesive group. This happens via 2 multicast groups which are defined to be
used for "all hosts on the same local network", but as of yet no system makes use of them
(224.0.0.1 from [rfc1112](https://tools.ietf.org/rfc/rfc1112.txt), ff02::1 from [rfc4291](https://tools.ietf.org/rfc/rfc4291.txt))

StatCom binds to these addresses on port `0x5747` (decimal `22343`) and listens for event strings. Event strings are _very_
loosly defined: in general a pattern of

```
EVENT_NAME:data1,data2,data3
```

or

```
EVENT_NAME:key1=val1&key2=val2&key3=val3
```

should be used, but at the moment everything is going to be accepted
and a plugin system is underway for handling/producing event data and
publishing it to the LAN.

