# Rust Level Based Projects

### Level 0: "Hello, World!" and Basic Syntax
Project: Simple Calculator
Focus: Basic syntax, variables, data types, functions, and conditionals.
Learning Goals:
- Learn how to set up a Rust project using cargo.
- Get familiar with the syntax for defining functions, variables, and using control flow like if, else, and match.
- Understand how to use Rust's data types (integers, floats, strings).
- Description: Create a command-line calculator that can perform basic arithmetic operations (addition, subtraction, multiplication, division).
- Input: User inputs two numbers and an operator (+, -, *, /).
- Output: The result of the operation.

Level 1: Understanding Ownership and Borrowing
Project: Simple To-Do List
Focus: Ownership, borrowing, and references.

Learning Goals:

Learn about Rust’s ownership system: ownership, borrowing, and lifetime.

Understand how to avoid memory issues with ownership rules.

Description: Build a command-line to-do list manager that lets users add, view, and delete tasks.

Store tasks in a Vec or HashMap.

Ensure tasks are correctly managed with ownership and avoid borrowing issues.

Work with strings and ensure proper memory handling.

Level 2: Structs, Enums, and Pattern Matching
Project: Basic File Manager
Focus: Structs, enums, pattern matching.

Learning Goals:

Learn how to define structs and enums to model data.

Use pattern matching with match and if let.

Handle errors with Result and Option types.

Description: Create a basic file manager that can list files in a directory, delete a file, and display file properties (e.g., size, permissions).

Use enums to model different file operations.

Work with the std::fs library to manipulate files.

Level 3: Collections and Iterators
Project: Command-Line Address Book
Focus: Collections (Vec, HashMap), iterators.

Learning Goals:

Understand Rust’s collection types and when to use them (Vec, HashMap, etc.).

Learn how to use iterators effectively to process data.

Implement searching, adding, and removing entries from a collection.

Description: Build a command-line address book where users can add, search, and delete contacts.

Use a HashMap to store contact names and phone numbers.

Implement iterators to search and list contacts.

Level 4: Error Handling and Option/Result
Project: Simple CSV Parser
Focus: Error handling, Option, and Result.

Learning Goals:

Learn Rust’s approach to error handling: Result for recoverable errors and Option for optional values.

Handle both Ok and Err outcomes in functions.

Use unwrap, expect, and proper error propagation.

Description: Write a CSV parser that reads a CSV file and parses its contents into a vector of structs. Handle parsing errors and invalid data gracefully.

Ensure that file reading and data parsing errors are handled using Result.

Make the parser flexible to handle different formats.

Level 5: Ownership, Borrowing, and Mutability in Complex Structures
Project: Simple Web Scraper
Focus: Complex structures, ownership, borrowing, mutability.

Learning Goals:

Deepen your understanding of how ownership and borrowing work in more complex scenarios.

Work with structs and enums that borrow data or require mutable references.

Description: Build a simple web scraper that fetches data from a website (using the reqwest crate) and parses it using select or scraper crate.

Parse HTML to extract specific elements (e.g., headings, links).

Ensure proper handling of mutable and immutable references in complex data structures.

Level 6: Asynchronous Programming
Project: Async File Downloader
Focus: Asynchronous programming (async/await), futures.

Learning Goals:

Understand Rust’s async programming model using async/await.

Work with the tokio or async-std crate for async tasks.

Learn how to manage multiple async tasks concurrently.

Description: Create a file downloader that can download multiple files concurrently.

Use reqwest and tokio to download files asynchronously.

Implement progress reporting for each file being downloaded.

Level 7: Concurrency
Project: Multithreaded Web Server
Focus: Concurrency with threads, Mutex, and Arc.

Learning Goals:

Learn how to use Rust’s concurrency model (threads, Mutex, Arc, etc.).

Work with shared mutable state in a multithreaded environment.

Understand thread synchronization and race conditions.

Description: Build a basic web server that handles multiple incoming requests using threads.

Use std::thread for spawning threads and Arc<Mutex<T>> for shared data between threads.

Handle HTTP requests and send basic responses.

Level 8: Advanced Memory Management
Project: Custom Memory Allocator
Focus: Low-level memory management, custom allocators.

Learning Goals:

Learn how to implement custom memory allocators in Rust.

Understand memory management concepts like heap, stack, and buffer overflows.

Description: Write a basic memory pool allocator.

Implement a system where memory is allocated in blocks and reused.

Compare the performance of your custom allocator to the default system allocator.

Level 9: FFI (Foreign Function Interface)
Project: Rust Bindings for C Library
Focus: FFI, working with C libraries from Rust.

Learning Goals:

Learn how to interface with C code from Rust using the FFI.

Understand how to manage memory and pointers across language boundaries.

Description: Write Rust bindings for a C library (e.g., libpng for image manipulation).

Use bindgen to generate the bindings.

Write Rust code that calls the C functions and handles their memory management.

Level 10: Building an Operating System Kernel
Project: Basic Rust Operating System
Focus: Low-level systems programming, OS development.

Learning Goals:

Understand low-level programming concepts such as bootloaders, memory management, and interrupts.

Learn how to write and structure an operating system kernel in Rust.

Description: Write a basic operating system kernel in Rust.

Implement a simple kernel that can print "Hello, World!" to the screen.

Use x86_64 crate and set up a simple bootloader using GRUB.

Learn about hardware interfacing, interrupts, and memory management at the OS level.






Level 11: Real-World Scalable Project - A Distributed File Syncing System
Project: Distributed File Syncing System (like Dropbox)
Focus: Networking, file systems, distributed systems, concurrency, cloud-like storage.

Learning Goals:

Build a fully distributed, fault-tolerant system.

Implement concurrency, file system management, and network communication.

Use advanced concepts like message queues, worker pools, and possibly even cloud-native components like object storage and synchronization protocols.

Gain practical knowledge on designing a system with real-world users in mind.

Description: Build a distributed file syncing system, similar to what Dropbox or Google Drive offers, but without all the external dependencies. The project could be cloud-based or run on a local network for simplicity.

Core Features:

File Synchronization: Multiple devices can sync their files with a central server.

Conflict Resolution: Handle situations where multiple devices modify the same file at the same time.

Compression and Deduplication: Reduce bandwidth and storage requirements by implementing file compression and deduplication.

Multi-Device Support: Devices can sync files in real-time and handle device connections dynamically.

User Authentication & File Sharing: Implement a basic user system for file access control.

Offline Support: Handle scenarios where the user is offline and synchronize changes once they are online.

REST API or gRPC: Provide an API for managing file uploads/downloads, sharing, and syncing across devices.

Rust Concepts You’ll Use:

Concurrency: Rust’s concurrency tools like async/await and tokio for network communication.

Networking: Use tokio or async-std for networking, file transfers, and HTTP server/client creation.

Filesystem Management: Interact with file systems using Rust’s standard library and external crates like tokio-fs.

Distributed Architecture: Implement basic distributed systems concepts like file replication, consistency, and fault tolerance.

Multithreading: Rust’s thread, Arc<Mutex>, and RwLock for managing shared state in a multithreaded environment.

Tech Stack:

Rust for core development.

tokio for asynchronous networking and file handling.

serde for JSON serialization and deserialization (for API communication).

sled or rocksdb for a local key-value store (to simulate cloud-like storage).

gRPC or REST API for communication between devices and the server.

Challenge Areas:

Synchronization: Develop a conflict resolution mechanism for when multiple devices modify the same file at the same time. This is a difficult problem that Dropbox and others spend a lot of time optimizing.

Networking & Error Handling: Implement robust network communication that ensures no file is lost or corrupted during sync. Handle network interruptions gracefully.

Scalability: Make sure your system can scale to handle a large number of devices and users. Think about load balancing, data partitioning, and replication strategies.


