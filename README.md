# `openapi-codegen`

Generate well-typed code from an OpenAPI spec

---

## Usage

1. Set up the development environment. Instructions for how to do that are in
   [`CONTRIBUTING.md`](./CONTRIBUTING.md).

2. You'll need an OpenAPI spec file, in either JSON or YAML format, in order to
   generate code. For an example, grab the [Pet Store][petstore] OpenAPI spec by
   running `make petstore`. (Note: this will overwrite any existing
   `openapi.json` file.)

3. In order for the generated Python code to be useable, run `make deps` to
   download the required dependencies.

4. Finally, you can generate code by running `make from-json` or `make
   from-yaml`. The generated code will be written to `openapi.py`.

5. As a bonus, you can also generate documentation for your generated code by
   running `make doc` (or `make doc-open` to automatically open the docs in
   a browser).

There are more commands in the [`Makefile`](./Makefile) (with documentation) so
be sure to check that out too.

[petstore]: https://petstore3.swagger.io/

## Features

This checklist is nonexhaustive, but useful as a quick reference:

* [X] [OpenAPI Info](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#infoObject)
  * Python: this is included as module-level documentation
* [X] Client object
* [ ] Client object methods ([OpenAPI Paths](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#pathsObject))
  * [X] Documentation
  * [ ] Requests
    * [X] Path parameters
    * [ ] Query parameters
    * [ ] Headers
    * [ ] Body
  * [ ] Responses
    * [ ] Body
    * [ ] Headers
  * [ ] Python: `aiohttp`
* [X] OpenAPI components
  * [ ] [Security schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#securitySchemeObject)
  * [X] [Schemas](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schemaObject)
    * [X] Documentation
    * [x] Property documentation
    * [X] Primitive JSON types
    * [X] Lists, Sets, and Maps
    * [X] Properties referencing other components
    * [X] Python: `pydantic`
