# Rust - HashMap

*Simple implementation of a HashMap written in Rust. Based on the Rust API.*

The current implementation is static in the size of buckets (1000).
Each bucket handles collisions by handling a list of (key, value) pairs.
An improvement would be to resize the number of buckets when the number of entries reaches 
a certain threshold, compared to the total capacity.