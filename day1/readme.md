# Day 1

This is my first stab back at Rust since version 0.6 and I think you can tell. Having
done more imperative programming of recent than functional, my solution is highly imperative.

After completeing this I shopped around online for other solutions and found one that I
really liked, using a very functional approach. What I found most surprising (my learning moment
for day 1) is that the compiler generated much less assembly for the version that took advantage
of higher-level concepts like iterators, zip, map, and collect than it did for the version I
present here. 

While I don't have benchmarks on the two, I have been intrigued to dig into the compiler details
of Rust a bit more.