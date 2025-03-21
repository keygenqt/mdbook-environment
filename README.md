
## MdBook environment preprocessor

A preprocessor for MdBook for working with environment variables.

### Feature

- Set environment variables.
- Set environment with execute command.
- Replace variables in the book.

### Example

**book.toml**
```toml
[preprocessor.environment]
# Just set environment
ENV_MDBOOK_VERSION1 = "0.0.1"
# Set environment with execute command
ENV_MDBOOK_VERSION2 = "$(curl -s 'https://api.github.com/repos/rust-lang/mdBook/tags' | jq -r '.[0].name' | sed 's/v//')"
```

**chapter_1.md**
```markdown
Just set environment: {{ENV_MDBOOK_VERSION1}}.

Set environment with execute command: {{ENV_MDBOOK_VERSION2}}.
```

### Preview

![preview](https://github.com/keygenqt/mdbook-environment/raw/main/data/preview.png)

### License

```
Copyright 2025 Vitaliy Zarubin

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
