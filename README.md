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
    * [X] Query parameters
    * [X] Body
    * [ ] Headers
  * [ ] Responses
    * [X] Body
    * [ ] Headers
* [X] OpenAPI components
  * [ ] [Security schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#securitySchemeObject)
    * [X] HTTP Basic Auth
  * [ ] [Schemas](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schemaObject)
    * [X] Documentation
    * [x] Property documentation
    * [X] Primitive JSON types
    * [X] Lists, Sets, and Maps
    * [X] Properties referencing other components
    * [ ] Type-safe enums

### Python

Generated Python code uses `aiohttp` to make requests and `pydantic` for
request/response serialization and validation. Check
[`requirements.in`](./requirements.in) for the specific versions supported.

`ApiClient` member functions may appear to have peculiar return types, this is
because Python has no support for sum types. `isinstance` is incapable of
operating on types with generic parameters; for example, `isinstance(x,
List[int])` is not allowed. As a result, generated functions have a return type
of `Tuple[str, Union[...]]`. Thus, you can check which type from the `Union` you
have by checking against the tuple's first element like so:

```python
client: ApiClient = # construct the client

(ty, resp) = await client.get_something() # -> Tuple[str, Union[str, List[int]]]

if ty == "str":
    s = cast(str, resp)
    print("a string:", s)
elif ty == "List[int]":
    ints = cast(List[int], resp)
    print("a list of ints:", ints)
else:
    raise AssertionError("unreachable")
```
