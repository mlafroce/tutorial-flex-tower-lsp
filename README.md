# Tower-LSP + Flex tutorial (WIP)

Hi! I started a compilers course recently so I decided to do this simple LSP tutorial. We are going to make a server for our **Barba-lang** custom language.

In this tutorial we are doing a simple language server with **Tower LSP**. To parse our lexemes we use **Flex**

*But you could use LALRPOP or Nom, they produce native Rust code*

Yes, I could, but playing with extern code is fun and most compilers tutorials are made with *flex*/*jflex* and *yacc*/*bison*, so, maybe this could help somebody.

*async code is not ok*

I know nothing about async :(

*Your comments are in spanish...*

Sorry, sometimes I'm lazy to translate

Feel free to PR! :D


## Steps

* *step-00*: a simple hello-world lexer in C with flex. Make sure you've got your lexing tools!

* *step-01*: the same lexer but now cargo-integrated.

* *step-02*: our rust code calls the C lexer, that calls some rust code to push our lexemes in a *token map*.

* *step-03*: we run our *tower-lsp* service to serve a test language server.

* *step-04*: we integrate *tower-lsp* with our lexer service.
