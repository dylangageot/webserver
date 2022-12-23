# Web server

This repository holds my first Rust project as a learning exercise, which is a web server with a simple [HTTP/1.1 protocol](https://www.rfc-editor.org/rfc/rfc2616) implementation, including a simple directory listing page generator.
As a regular modern C++ developer, this document also includes some thoughts and things I learned about the language and its specificities during my journey in developing the application.

## Introduction

[The Rust Programming Language](https://doc.rust-lang.org/book/) book Chapter 20 focuses on a web server implementation using sockets and threads from the standard library.
Though I find it interesting and on-point, as the entire book, I thought it would go deeper and undergo a basic modeling and implementation of an HTTP server.
It was the perfect time for me to go off-road and pursue my desire to model and implement my own to practice what I've learned throughout the book.

## Modeling an HTTP message

### First, a request

In the first iteration, I started modeling a request as a `struct` that contains a request start-line with an HTTP method, a URL, an HTTP version, then optional headers, and finally an optional body:
- HTTP methods and versions are symbols that can be represented with an `enum` in its simplest form,
- the start-line is a `struct` with the method, the URL, and the version,
- headers are stored in a `std::HashMap<String, String>`,
- body is a `Option<Vec<u8>>`.

### Then, a response

As I wanted to implement HTTP responses in a future iteration, I refactored the whole response model into a commonized HTTP message model to represent responses as well.
This model uses an `enum` start-line to represent either a request start-line or a response status-line, containing the status code and the current HTTP version.

### Dealing with common concepts and implementations

Here an HTTP message is a concrete type, which represents either a request or a response. 
As implementation for parsing and serialization are the same for headers and bodies, it is intuitive to have a common concept for an HTTP message.
In an OOP language, an HTTP message would have been modeled with a base class defining headers and body fields, and parsing and serialization methods. 
Request and Response classes would derive from Message to inherit the common structure and implementation. 
Determining which type of message has been instantiated would require an RTTI identification at run-time using a dynamic cast or a call to a virtual method.

With the current Rust implementation, the `enum` type `StartLine` is either a request start-line or a response status-line, which is statically encoded and the values it can have are known at compile-time, which enforce the developer to handle all the cases of a message being either a request or a response.

> Note: the `std::variant` object in C++17 is the equivalent of Rust `enum`.

### Enforcing strong types

To get further with type safety, I wanted to provide stronger types to the existing concepts in the project:
- bodies are represented with a `Body` type, a type that uses the newtype pattern to wrap a `Vec<u8>`,
- headers are represented with a `Headers` type, a type that uses the newtype pattern to wrap a `std::BTreeMap<String,String>`.

> Note: Noticed the change in map implementation from `HashMap` to `BTreeMap` for `Headers`? The latter one provides an order to classify keys, which helps to test equality in serialization unitary test.

Each of these new types provides a set of methods specialized to their concept.
For example, `Header::get_content_length()` returns the content length stored in the message headers if available.

## Parsing and serialization

### Reading from and writing to a TCP stream

At this point, knowing the nature of a stream to read from or write to an HTTP message would be overfitting to the problem.
Overfitting might be prejudicial for unit testing as it would require opening a TCP stream for each test.
What matters here is to use the correct abstraction to read from and write to.
The `std::net::TcpStream` type implements both the `Read` and `Write` traits.
Hence, parsing and serializing methods for our types can use those two traits to perform reading and writing operations.

More specifically, the message parsing method `Message::read(bufread: &mut impl BufRead)` is slightly more sophisticated since it takes a mutable reference to a type that implements the `BufRead` trait as an argument.
The `BufRead` trait extends the `Read` trait, which is its super trait under the hood, to provide better management of memory with a buffer while reading from it.
In the example of `Message::read`, it enables the parser to interpret the buffer as a line iterator, hence parsing start-line to headers as string lines through the `BufRead::lines()` method.
This method is more convenient than searching for CR and LF characters in a flat buffer.

### Parsing and serializing enum types

The `FromStr` trait can be implemented on any type that could be parsed from a string.
On the other way around, the `ToString` trait can be implemented on any type that could be serialized. 
In this project, the HTTP method, the HTTP version, and the HTTP status are candidate enum types for parsing and serializing to the TCP stream.
Implementing `FromStr` on each of these enum types will enable the library consumer to call `str::parse()` on a string to get the target type.

However, implementing `FromStr` and `ToString` traits is cumbersome for flat enum types that would have benefitted from annotations in their alternative definitions instead.
This is exactly what provides the [strum](https://crates.io/crates/strum) crate with macros that implement `FromStr` and `ToString` from annotate `enum` alternatives.
In my opinion, this crate is a great way to reduce the bug opportunities raised by implementing manually those traits.

## Handling error

As I wrote the HTTP message parser, I unwrapped so many `Result` types for the sake of fast programming that it was in no way possible to ignore them while testing.
Thanks to explicit `Result::unwrap()` or `Result::expect()`, it was easy to come back later in development to handle leftover error cases.
The first pitfall from my side was laziness: many error types of different natures to handle, and many reasons for them to appear that I would not bother to work with while I was in flow mode.

To get back on track, I read this [article](https://nick.groenen.me/posts/rust-error-handling/) online from Nick Groenen.
This article cover techniques to handle error for both library and binary projects.
As a result of this reading, the `Error` type has been introduced to the `http` module.
In addition, a new `Result` definition that aliases the core one with this specific `Error` type is defined.
This new `Result` is returned from every method that could fail in the `http` module.
Each error of different nature than `Error` is wrapped and appended a reason of failure with a specific context with manual `From<T>` trait implementation, hence enabling the use of the `?` operator.

This project does not use [thiserror](https://crates.io/crates/thiserror) crate as recommended in the article mentioned beforehand.
I wanted to train myself by manually writing my error type and its various implementation, from `From<T>` traits to `Display`.

## Generating a directory listing page

The final step of this training project was to generate a directory listing/index page on request from a web browser.
As a result, the `http::index` module provides a public method `generate()` to generate an HTTP response with a directory listing page in the body from a given URL.
If the URL is a file, the body contains the content of the file with the response header signaling an `application/octet-stream` MIME type.
If the URL refers to a nonexisting inode on the file system, a 404 Not Found page is sent.

The directory listing is generated using a Mustache template format thanks to the [ramhorns](https://crates.io/crates/ramhorns) crate.
This crate is incredibly helpful to organize a text generator based on a `struct` and a simple plain text definition of the template.

## Run the server

Start the server by running the following command, listening on port 7878:

```
cargo run
```