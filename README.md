# variable-resolver
The aim of this package is to easily replace variables inside a given template.

The package contains a library and a cli application (**repvar**).

## Format of the template
The package is capable of finding variable names inside double curly braces.

Example template:
```
Hello, {{name}}
```

Escaping double curly braces happens by using triple curly braces. Examples:
* The following template: `Hello, {{{name}}}` will resolve to `Hello, {{name}}`
* The following template: `Hello, {{{{name}}}}` will resolve to `Hello, {{{name}}}`
* The following template: `Hello, {{{{{name}}}}}` will resolve to `Hello, {{Jane}}`
* The following template: `Hello, {{{{{{name}}}}}}` will resolve to `Hello, {{{{name}}}}`

## Using *repvar* cli utility
The following example demonstrates how every occurence of a variale, named *name*, can be replaced with *Jane* in the given template. Example:
```bash
echo "Hello, {{name}}" | repvar -v name=Jane
```

Templates inside files can be handled by piping the file into repvar. Example:
```bash
cat greeting.txt | repvar -v name=Jane
```

Be aware that writing to the same file as the source is **not safe**. Example:
```bash
cat greeting.txt | repvar -v name=Jane > greeting.txt
```

## Todos
* docstring
* case-insensitive variable resolver mode
* load variables from environment variables
* load variables from .env files
* load template from file
* replace variable with empty string
