# nelth
Network Health Check

The goal of Network Helath Check (nelth) is to find sporadic or problematic networking points by doing constant polling and metric generation with some element of tracing or debugging information.  So when that issue happens at 3:54am like it always does you'll have a bit more data to understand where the problem may be.

This is my first rust program.



# notes
## 8.21
I just got the rust basics going, learned about package management and importing dependencies with cargo and also cargo build and cargo run.  I found a decent http library, hyper, and I just figured out how to get https going with hyper to make a basic https request.

Next up I need to work on some timing information on the https requests, hopefully there is something i can hook up into hyper.

Then I need to do this over a loop and allow for a command line arg url.

Then I need to look at even lower level and look at some connection libraries or stdlib and do the same thing.  Ideally I want to get to the packet level for analysis, hyper may be too high level here.

I also need to start adding some logging and other error checking basics.