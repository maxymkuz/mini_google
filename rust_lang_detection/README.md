### A simple Rust language detector

Intended to be used both from the crawlers and from the backend.

Probably won't be able to launch this on the client side, the startup
of the detector takes significant CPU time. Maybe should consider running
it in a separate container with its own threads or something as a layer
between the backend and the database, since after the startup it responds
to requests pretty quickly and shouldn't cause a huge delay.
