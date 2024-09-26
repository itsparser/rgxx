## RGXX - Natural Language Regex Generator

A minimal Python library powered by Rust and PyO3 for generating regular expressions using natural language chainable functions.

## Overview

This project provides a Python library that simplifies the creation of regular expressions by using a natural language syntax. It leverages Rust's performance and safety, with PyO3 bridging Rust and Python, to deliver an ultra-minimal runtime with zero dependencies.

## Features

*   **Zero-Dependency and Ultra-Minimal Runtime**: Designed to be lightweight with no external dependencies, ensuring fast execution and minimal overhead.
*   **Pure RegExp Compilation**: Generates standard regular expressions compatible with Python's `re` module and other regex engines.
*   **Automatically Typed Capture Groups**: Easily define named capture groups that are automatically recognized, simplifying pattern matching and data extraction.
*   **Natural Language Syntax**: Utilize chainable functions that read like natural language, enhancing code readability and maintainability.
*   **IDE Support**: Generated regular expressions display on hover in supported IDEs, aiding in development and debugging.

## Installation

Install the library using pip:

```shell
pip install rgxx
```

## Usage

Here's how you can use the library to create a regular expression for matching dates in the `YYYY-MM-DD` format:

python

Copy code

```python
from rgxx import digit, exactly, any_of, RegExp 


# Define the components of the date pattern 
year = digit().times(4).grouped_as('year') 
month = any_of(
        exactly('0') & digit(), 
        exactly('10'), exactly('11'), 
        exactly('12')
    ).grouped_as('month') 
day = any_of(
        exactly('0') & digit(), 
        exactly('1') & digit(), 
        exactly('2') & digit(), 
        exactly('30'), exactly('31')
    ).grouped_as('day')
    
# Combine the components into a single RegExp object
date_pattern = RegExp(year, exactly('-'), month, exactly('-'), day)
print(date_pattern.compile())

```

**Output:**

```shell
`(?P<year>(\d){4})\-(?P<month>((0)\d|10|11|12))\-(?P<day>((0)\d|(1)\d|(2)\d|30|31))`
```

**Example Usage with Python's** `**re**` **Module:**

```python
import re

# Compile the generated regular expression
date_regex = re.compile(date_pattern.compile())

# Match a date string
match = date_regex.match('2023-10-05')
if match:
    print(match.group('year'))   # Output: 2023
    print(match.group('month'))  # Output: 10
    print(match.group('day'))    # Output: 05

```

## Documentation

*   `**digit()**`: Matches any single digit (`\d`).
*   `**exactly(s)**`: Matches the exact string `s`, escaping special regex characters.
*   `**any_of(*patterns)**`: Matches any one of the provided patterns.
*   **Chaining Methods**:
    *   `**.times(n)**`: Repeats the pattern exactly `n` times.
    *   `**.grouped_as(name)**`: Names the capture group as `name`.
    *   `**.and(other)**` **or** `**&**`: Concatenates the current pattern with another.

## Contributing

Contributions are welcome! Please follow these steps:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Commit your changes with clear messages.
4.  Open a pull request describing your changes.

## License

This project is licensed under the MIT License.

---

Feel free to customize this bio further to suit your project's specific details or to add more sections such as acknowledgments, FAQs, or a roadmap.
