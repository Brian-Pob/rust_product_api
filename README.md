# Product API

Hello, world.

This is my first ever Rust project.

This project is a version of a Product management API coded in Rust. The original version was done in C# for a college class I took. Some concepts that it utilizes include but are not limited to: OOP, Design Patterns, File Management, Data Serialization, and HTTP Requests.  I will slowly be building this by converting parts of the C# API into Rust as I go through the rust lang book. Eventually, I want to connect this with a simple React web app.

Side note: I also want to practice TDD with this project.

---

## TODO

### 8/13/2022

So far, I have found that it might not be possible to implement the API in Rust the same way it was implemented in C#. I will need to think about other ways to create the API.

### 8/15/2022

I decided to take things a lot slower. My first goal will be to implement inheritance for the product model.

So it looks like Rust does not support inheritance in the same way a traditional OOP-friendly language like Java does.
One recommendation is to encapsulate the parent struct in the child struct. This creates a has-a relationship instead of an is-a relationship, which is what you would usually see in a class-based language.

For this project, I should probably learn traits. Traits in Rust are similar to interfaces in Java.

So I have been able to implement polymorphism, or at least something that resembles it.
My next goal is to implement serialization and deserialization and saving the data to a file.
After that I could either try to implement MongoDB as my database or create the CLI to use the API.

### 8/17/2022

Today I am going to use serde to implement serialization and deserialization. I added the serde crate
using `cargo add serde --features derive` as well as serde_json with `cargo add serde_json`.

First thing I need to do is let my product class derive the Serialize and Deserialize traits.

### 8/19/2022

I have successfully implemented serialization and deserialization. I found out that when you try to deserialize a mismatched JSON object, it will throw an error and Rust will panic.

Now, I can try to implement MongoDB like I originally planned. However, I think I will try to implement testing first. Test Driver Developement
has always been something I've wanted to try with any language and Rust seems to have a very easy to use TDD system.

After that, I will try to implement MongoDB. Ideally, I would run the MongoDB server in a Docker container and connect to it from the Rust code.
Although that seems to be a bit complicated. I may just run the server locally.

An alternative is to use something similar to Filebase from my C# project.
