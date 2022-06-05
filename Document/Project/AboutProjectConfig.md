# About Project Config

Unless you want to use Nyah's compiler to compile code directly into binaries, every Nyah project will always have its
configuration file named `Nyah.json`.

Typically, a project's configuration file looks like this:

```json
{
  "name": "ExampleProject",
  "dependency": [
    "ExampleDependency"
  ],
  "config": {
    "exampleConfig": "This is a string."
  }
}
```

## name

In the configuration file, `name` represents the project name of a project. By configuring `name`, you can use your
project as a dependency of other projects.

Like this:

```json
{
  "name": "ExampleProject"
}
```

## dependency

`dependency` represents the dependencies of the project, and each element corresponds to each dependency. The type of
the element can be [`String`](#String), [`Array`](#Array), or [`Map`](#Map).

The function of each type is as follows:

### String

```json
{
  "name": "ExampleProject",
  "dependency": [
    "ExampleDependency"
  ]
}
```

In the above example, `ExampleDependency` is used as the name of a dependent project. During the compilation process,
the Nyah project manager will call the compiler from `NyahProjectCenter` before invoking the compiler Download all
project dependencies. This project name will be used as the index name, that is, the open source project
named `ExampleDependency` downloaded in `NyahProjectCenter` and used as a dependency. If you need to use more advanced
project dependency definitions, such as custom project version numbers, etc., use [Map](#map) to express project
dependencies.

### Array

```json
{
  "name": "ExampleProject",
  "dependency": [
    [
      "ExampleDependency1",
      "ExampleDependency2",
      "ExampleDependency3"
    ],
    [
      "ExampleDependency4",
      "ExampleDependency5",
      "ExampleDependency6"
    ]
  ]
}
```

In the above example, we use an Array instance as an element. This is just for categorizing project dependencies,
nothing more.

### Map

```json
{
  "name": "ExampleProject",
  "dependency": [
    {
      "name": "ExampleDependency",
      "version": "1.0.*",
      "config": {
        "threadPool": {
          "init": 1,
          "max": 4
        }
      }
    }
  ]
}
```

In the above example, we used a `Map` instance as an element. By using `Map`, we can implement more advanced operations,
such as `version` in the example Limit the version to `1.0.*`, Nyah will download the version number starting
with `1.0.` when compiling. Usually Nyah will choose the largest number in the set of numbers represented by `*`, such
as an item
`ExampleDependency` has 3 versions, namely `1.0.10`, `1.0.5`, `0.9.15`, Nyah will choose the package version `1.0.10`.

`config` is used to override the configuration of the dependent project, which can be learned
in [the next chapter](#config).

## config

[NotCompleted](../NotCompleted.md)
