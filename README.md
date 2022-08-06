# 🐱 Nyah ( Unfinished )

[![Run tests](https://github.com/EnabledFish/Nyah/actions/workflows/UnitTest.yml/badge.svg?branch=main)](https://github.com/EnabledFish/Nyah/actions/workflows/UnitTest.yml)

An object-oriented programming language, meow~

## 🎖️ Status

Nyah is not currently available, I will slowly implement all its basic features.

## 🔮 Example

A tasting is available here.

```nyah
use Nyah.Network.HttpServer;

public func main(): ()? {
    var server = HttpServer.new();
    server.route("/", |_request, response| {
        response.write("Hello world!");
    });
    server.bind(8080);
    server.listen()
}
```

## 📚 Document

Click [here](Documents/README.md) for the documents.

## 🥳 Special Thanks

- [Xie-Jason](https://github.com/Xie-Jason)

[![☬](https://komarev.com/ghpvc/?username=EnabledFish&stype=flat&label=ViewedTimes)](https://github.com/EnabledFish)
