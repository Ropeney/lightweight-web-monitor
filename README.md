# :collision: This is not finished and not operational :collision:

---

# Lightweight Web Monitor

This is the start of creating a lightweight web server monitoring system in rust.

#### Design Goals

A polling type system that feeds a queue dependant on DB SQL results. The SQL will be responsible
for deciding if a check should be performed. There will be seperate workers that are multi threaded
and checking the notification queue for any work they must perform.

### License
---
This is realeased under the [MIT License](http://www.opensource.org/licenses/MIT) you can do what you wish with it :trollface:
