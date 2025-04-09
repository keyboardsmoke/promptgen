# PromptGen

A robust Rust-based templating engine for generating sophisticated LLM prompts, powered by MiniJinja2.

## Overview

PromptGen is a specialized tool designed for creating, managing, and executing complex prompts for Large Language Models (LLMs). By leveraging the power of the Jinja2 templating system through MiniJinja2, it enables the creation of dynamic, reusable prompt templates that can incorporate context, variables, and external data.

## Features

- **MiniJinja2 Integration**: Full support for Jinja2 syntax including variables, loops, conditionals, and template inheritance
- **Dynamic Data Inclusion**: Seamlessly incorporate data from various sources:
  - Environment variables (`getenv`)
  - File contents (text with `readfile` and JSON with `readjson`)
  - HTTP requests (`http_get`, `http_post`)
- **Modular Design**: Create reusable prompt components through template inclusion and inheritance
- **CLI Interface**: Easy to use command-line interface for script execution
- **Library Support**: Can be used as a standalone tool or integrated as a library in other Rust applications

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

- `getenv(key)`: Get environment variable values
- `readfile(path)`: Read text file contents
- `readjson(path)`: Read and parse JSON file contents
- `httpget(url, [useragent])`: Make HTTP GET requests
- `httppost(url, body, [useragent])`: Make HTTP POST requests

## Advanced Features

- Template inheritance and inclusion
- Custom filters (e.g., `repeat` filter)
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

