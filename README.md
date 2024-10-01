# Custom Key:Value Store in Memory Safe Rust
Inspired by Redis

This project serves as a milestone in my journey to learn Rust. It is also in response to Redis changing it's licensing.
The goal is to create a very simple key/value store in memory that can cache json objects and facilitate queuing systems.

Below are my initial goals for the project:
1. Create a concurrent TCP server to handle client requests.
 - Rust's standard `net` library for TCP Connection, stream.
 - Rayon's ThreadPool for thread management.
2. Creation of my own Protocol from which I will eventually build the client binary.
3. Implement Arc and Mutex types for keeping the data store in sync between threads.
