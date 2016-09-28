Stocks - A small-scale logistics managing server
================================================

**This is currently in early stages of development and not really usable in production.**

Stocks is a database application that keeps track of your
grocery stocks (but is not limited to groceries!).

The core of stocks is the backend application that exposes
its stored data via a RESTful Api. Additionally there are
a set of client application that bind against the REST Api.

Platform support
----------------

The following platforms are targeted:

 - Server
  * Linux
 - Client
  * Web browser
  * Android

Dependencies
------------

The server requires a running PostgreSQL database. No other
database servers are currently supported.

