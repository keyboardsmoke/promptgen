# PromptGen

A robust Rust-based templating engine for generating sophisticated LLM prompts, powered by MiniJinja2.

## Overview

PromptGen is a specialized tool designed for creating, managing, and executing complex prompts for Large Language Models (LLMs). By leveraging the power of the Jinja2 templating system through MiniJinja2, it enables the creation of dynamic, reusable prompt templates that can incorporate context, variables, and external data.

## Features

- **MiniJinja2 Integration**: Full support for Jinja2 syntax including variables, loops, conditionals, and template inheritance
- **Dynamic Data Inclusion**: Seamlessly incorporate data from various sources:
  - Environment variables (`getenv`)
  - File contents (text with `readfile` and JSON with `readjson`)
  - HTTP requests (`webget`, `webpost`)
- **Data Processing**: Rich set of filters for text manipulation:
  - Base64 encoding/decoding
  - Cryptographic hashing (MD5, SHA-256, SHA-512)
  - Regular expression operations
- **Modular Design**: Create reusable prompt components through template inclusion and inheritance
- **CLI Interface**: Easy to use command-line interface for script execution
- **Library Support**: Can be used as a standalone tool or integrated as a library in other Rust applications
- **Testing Support**: Structured testing capabilities for template validation

## Usage

### Command Line

```
promptgen --script <script_path> [--script_dir <directory>] [--arguments <json_args>]
```

### Example Template

```jinja
{# A prompt template for coding assistance #}
You are a coding assistant helping with {{ language }} programming.

Here is the task description:
{{ task_description }}

{% if context_files %}
Here is some relevant context:
{% for file in context_files %}
File: {{ file.name }}
```
{{ readfile(file.path) }}
```
{% endfor %}
{% endif %}

Please provide a solution with clear explanations.
```

## Functions

PromptGen provides several built-in functions for use in templates:

### Environment
- `getenv(key)`: Get environment variable values

### JSON
- `parsejson`: Parse JSON from string data
- `readjson(path)`: Read and parse JSON file contents

### I/O
- `readfile(path)`: Read text file contents

### HTTP
- `webget(url, [useragent])`: Make HTTP GET requests
- `webpost(url, body, [useragent])`: Make HTTP POST requests

### Utility
- `now(format)`: Generate a time string
- `uuid`: Generate a UUID

## Filters

PromptGen includes powerful filters for text transformation and processing:

### Text Manipulation
- `repeat`: Repeat a string n times (e.g., `{{ "abc" | repeat(3) }}` → `abcabcabc`)

### Encoding/Decoding
- `b64encode`: Encode a string to Base64
- `b64decode`: Decode a Base64 string

### Cryptographic Hashing
- `hash`: Generate cryptographic hashes with specified algorithm
  - `{{ "data" | hash("md5") }}`
  - `{{ "data" | hash("sha256") }}`
  - `{{ "data" | hash("sha512") }}`

### Regular Expressions
- `regex_match`: Test if a string matches a pattern (returns boolean)
    - `{{ "test" | regex_match("t.st") }}`
- `regex_replace`: Replace text matching a pattern
    - `{{ "This is a longer test" | regex_replace("l.nger", "SHORT") }}`
- `regex_split`: Split a string by pattern (returns array of strings)
    - `{{ "1 2 3 4 5 6" | regex_split("\\s+") }}`
- `regex_search`: Extract capture groups from a string (returns array of captures)
    - `{{ "http://test-url:80/index.php?hello=1234567890" | regex_search("(http|https)://([a-zA-Z0-9.-]+)(:\\d+)?") }}`

## Testing

PromptGen supports structured testing of templates through `jinja_test.py`

- Run `python jinja_test.py`
- This will test all basic functionality and sanity in the jinja interpreter
- Otherwise, you can run cargo testing as usual

## Advanced Features

- Template inheritance and inclusion
- Rich error handling and debugging
- High performance through Rust's efficiency

## Integration

PromptGen can be integrated with various LLM APIs and platforms, making it a versatile tool for AI prompt engineering and development workflows. The Jinja2 templating system makes it particularly powerful for:

- Creating structured prompt formats
- Implementing few-shot learning examples
- Building complex prompt chains
- Generating systematic variations of prompts for experimentation

## Building from Source

```
cargo build --release
```

## License

[MIT License](LICENSE)

