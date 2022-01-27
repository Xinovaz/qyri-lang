![alt text](https://github.com/Xinovaz/qyri-lang/blob/main/Sam_Candle_Q.png?raw=true)

# In This Branch:
The latest version of Qyri, and the first fully-developed iteration, including the frontend *and* backend. As of the time this is being written, the Qyri Linking Language (QLL) is fully developed and ready for use in the arclight crate. Run the test (which performs some basic operations such as storing a struct and enum) using the following command (Rust is needed at the moment):

``$ cargo run -- test.qll``

This will build the QLL interpreter.

# Current state of the Qyri project:
The QLL interpreter is unoptimised. Fortunately, Rust is very fast! So, do not expect noticeable lag in your programs.

The Qyri Programming Language currently lacks a working frontend. This is on its way. The docs (located in the `main` branch) do not provide accurate code, both in syntax and in function. However, do view the docs for a general understanding of what the language may look like in the future.