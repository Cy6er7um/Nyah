# 🐱 Nyah ( Unfinished )

[![Run tests](https://github.com/EnabledFish/Nyah/actions/workflows/UnitTest.yml/badge.svg?branch=main)](https://github.com/EnabledFish/Nyah/actions/workflows/run-tests.yml)

An object-oriented programming language, meow~

## 🎖️ Status

Nyah is not currently available, I will slowly implement all its basic features.

Usually, every day I make some commits to this project.

## Example

```nyah
use Network.HttpServer;

pub func nyah() {
    var server = new HttpServer();
    server.bind(8080);
    server.route("/", func(_request, response) {
        response.write("Hello world!");
    });
    server.listen();
}
```

## 📚 Document

Here are some documents which can help you learn more about Nyah.

1. [Project - AboutProjectConfig](./Document/Project/AboutProjectConfig.md)

_Not completed yet._
