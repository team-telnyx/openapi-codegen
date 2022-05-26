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

`ApiClient` member functions may appear to have peculiar return types. This is
because they are [sum types][wikipedia], and you can read more about how they
work in Python [here][sum_types_python]. Here's a simple example of how to use
the return types meaningfully:

```python
client: ApiClient = # construct the client

resp = await client.get_example()

# The type of `resp` is:
#
# Union[
#   Tuple[Literal["str"], str],
#   Tuple[Literal["List[int]"], List[int]]
# ]

match resp:
    case ("str", x):
        # The type of `x` is narrowed to `str`
        print("a string:", x)
    case ("List[int]", x):
        # The type of `x` is narrowed to `List[int]`
        print("a list of ints:", x)
```

Currently, undocumented HTTP response codes are raised as
`aiohttp.ClientResponseError`, deserialization failures are raised as their
usual `pydantic` exceptions, and other such failures are raised as exceptions.

[wikipedia]: https://en.wikipedia.org/wiki/Tagged_union
[sum_types_python]: http://charles.page.computer.surgery/blog/python-has-sum-types.html
