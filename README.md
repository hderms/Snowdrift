# Snowdrift
Heavily borrows from the ideas in the (unfortunately defunct) Twitter [Snowflake](https://github.com/twitter-archive/snowflake) project.


## Details
id is composed of:

* time - 41 bits (millisecond precision from UNIX epoch, not very useful currently)
* configured machine id - 10 bits - gives us up to 1024 machines
* sequence number - 12 bits - rolls over every 4096 per machine (with protection to avoid rollover in the same ms)


## Why?
I always thought it was a really awesome idea for a project and wanted to see how fast I could get it to run in Rust.
On my computer:
```
./hey -c 500 -z 10s http://localhost:3000/
``` 
yields 144558.9811 req/s. This would imply the max scalability for a distributed set of Snowdrift nodes would be 148M IDs generated per second. 
