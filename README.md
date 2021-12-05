# Hashpass

## NOT AN ORIGINAL IDEA

It turns out that the idea and the name have been previously claimed by
others. Specifically, the [hashpass](https://github.com/stepchowfun/hashpass)
Chrome extension seems to be the tool you should use if you're looking for
a tool like this. I'm going to keep my implementation published and potentially
improve it, but definitely not try to compete with this tool which does
exactly what I want and doesn't need to be maintained by me.

## Original README for Posterity

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
