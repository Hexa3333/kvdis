# KVdis
KVdis is a lightweight redis-like database that works over **TCP**.

## Commands
\> **SET** \<key\> \<value\><br>
\> **GET** \<key\><br>
\> **DEL** \<key\><br>
\> **EXISTS** \<key\><br>
\> **EXPIRE** \<key\> \<TTL in [humantime format](#time_format_section)\><br>
\> **INCR** \<key\><br>
\> **DECR** \<key\><br>
\> **CLEAR**<br>
\> **SAVE**<br>
\> **LOAD**

<a id="time_format_section"></a>
### Time format
KVdis uses the [humantime](https://github.com/chronotope/humantime) Duration format as input, and stores information in [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) timestamp format.
