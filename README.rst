crusty bits
~~~~~~~~~~~

This is my personal playground for Rusty code.  This is all MIT licensed.
None of this is meant to be production-ready code, but I would like to at
least try to write good and useful code, so feel free to PR improvements.

The primary goal of all crates here is to be correct, easy to use, and
fast to build, in that order.  Being fast is useless if you're wrong.
``cargo test`` should take less than a second per crate, the punch-card
days called, they want their work-flow back.

A few design-choices are common to all the crates here, which you may
want to know before digging further here:

* Absolutely no ``async``, I strongly believe that the tiny minority
  who may need ``async`` should pay the price, and leave the rest of
  us with simpler, prettier code.
* Only use ``unsafe`` for FFI, or if something is impossible to implement
  without it (e.g. data-structures?), and then wrap it in a sound, safe
  API.  No using unsafe for tiny speed-ups or clever tricks.
* With very few exceptions do not depend on crates.io, pulling in
  even simple-seeming crates has a bad habit of pulling in crazy-deep
  dependency hains; resuling in ballooning build-times, and adding a huge
  number of lines of code to vet, often for frivolous features like
  colorful terminal-output or spelling-suggestions for options.

Crates
------

``reqeasy``
    HTTP client
``libpcre2``
    Bindings to libpcre2
``simple_regex``
    Regular Expressions -- pretty much fubar, need to go read a book
