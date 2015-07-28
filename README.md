# pwm

A simple password manager for Linux.

## Notes on Security

### Key Derivation

pwm uses the Scrypt key derivation algorithm to turn a password into an encryption key. The Scrypt parameters used are those recommended for sensitive storage from [Colin Percival's original slides about Scrypt](http://www.tarsnap.com/scrypt/scrypt-slides.pdf).

### Zeroing Sensitive Memory

pwm makes no attempt to zero memory used to store sensitive data like encryption keys or decrypted data. If we are to assume that an attack has access to read all memory, swap, registers, and etc. on your system, [zeroing buffers is insufficient](http://www.daemonology.net/blog/2014-09-06-zeroing-buffers-is-insufficient.html). Because ensuring that sensitive information is not leaked whatsoever is virtually impossible (see the linked article for details), pwm simply accepts the potential risk involved in storing sensitive information in memory.
