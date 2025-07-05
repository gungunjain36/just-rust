# Contact List Manager

**Concepts Covered:**  
- Structs  
- Methods  
- Ownership  
- Borrowing  

## Project Overview

Design a small in-memory address book where each contact is represented as a struct (e.g., name, phone, optional email). Use a `Vec` to store contacts and implement methods to add, find, and remove contacts. Use `Option` for fields that may be missing (e.g., optional email).

This project teaches:
- Ownership (storing `String` data in structs)
- Borrowing (passing references to methods)

It highlights how Rust’s ownership model enforces a single owner for each value.