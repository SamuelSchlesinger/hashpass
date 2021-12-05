# Hashpass

A very simple password manager which derives a password for every service
based on a hash of your master password appended to the name of the service.

The main benefit of this is that it does not rely upon encryption to keep
your passwords safe, but instead it is truly impossible for anyone to find
your passwords unless they can invert a cryptographic hash function or have
your master password.

The main con to this is that if a hacker has your master password, they will
be able to recreate your passwords on their machine. In order to get around
this, you can store a secret nonce which you should append to the service name,
but then the security of that nonce is your new concern, so its sort of
turtles all the way down.
