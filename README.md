# Product API

Hello, world.

This is my first ever Rust project.

This project is a version of a Product management API coded in Rust. The original version was done in C# for a college class I took. Some concepts that it utilizes include but are not limited to: OOP, Design Patterns, File Management, Data Serialization, and HTTP Requests.  I will slowly be building this by converting parts of the C# API into Rust as I go through the rust lang book. Eventually, I want to connect this with a simple React web app.

Side note: I also want to practice TDD with this project.

---

## TODO

### Part 1

So far, I have found that it might not be possible to implement the API in Rust the same way it was implemented in C#. I will need to think about other ways to create the API.

8/15/2022
I decided to take things a lot slower. My first goal will be to implement inheritance for the product model.

So it looks like Rust does not support inheritance in the same way a traditional OOP-friendly language like Java does.
One recommendation is to encapsulate the parent struct in the child struct. This creates a has-a relationship instead of an is-a relationship, which is what you would usually see in a class-based language.

For this project, I should probably learn traits. Traits in Rust are similar to interfaces in Java.

### Part 2

1. To follow...
