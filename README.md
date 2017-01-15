# pwm

A simple password manager for Linux.

## Notes on Security

### Zeroing Sensitive Memory

pwm makes some attempt to zero memory used to store sensitive data like encryption keys or decrypted data, however this cannot be guaranteed in all cases. If we are to assume that an attack has access to read all memory, swap, registers, and etc. on your system, [zeroing buffers is insufficient](http://www.daemonology.net/blog/2014-09-06-zeroing-buffers-is-insufficient.html). Because ensuring that sensitive information is not leaked whatsoever is virtually impossible (see the linked article for details), pwm makes some attempt at zeroing memory purely as a defense-in-depth, but at the same time it accepts the potential risk involved in storing sensitive information in memory.
