# Steep Ruby LSP Extension for Zed

A Zed extension that integrates the [Steep](https://github.com/soutaro/steep) Ruby type checker language server into your development workflow.

## Overview

Steep is a gradual type checker for Ruby that helps you write more reliable Ruby code by providing static type analysis. This extension brings Steep's language server capabilities directly into Zed, offering real-time type checking, intelligent code completion, and enhanced Ruby development experience.

## Features

- **Real-time type checking** - Get instant feedback on type errors as you write Ruby code
- **Intelligent code completion** - Context-aware suggestions based on type information
- **Type-aware navigation** - Jump to definitions and references with type context
- **Hover information** - View type signatures and documentation on hover
- **Diagnostic reporting** - Clear error messages and warnings for type issues
- **Smart project detection** - Automatically activates when a `Steepfile` is present
- **Flexible execution** - Works with Bundler, global installations, or custom setups

## Requirements

### Prerequisites

1. **Ruby** - A working Ruby installation
2. **Steep** - The Steep gem installed either:
   - Via Bundler (recommended for projects)
   - Globally via `gem install steep`
3. **Steepfile** - A `Steepfile` configuration in your project root

### Installation Methods

#### Option 1: Via Bundler (Recommended)

Add Steep to your project's `Gemfile`:

```ruby
group :development do
  gem 'steep'
end
```

Then run:
```bash
bundle install
```

#### Option 2: Global Installation

```bash
gem install steep
```

## Installation

### From Zed Extension Gallery

1. Open Zed
2. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux) to open the Extensions view
3. Search for "Steep Ruby LSP"
4. Click "Install"

### Manual Installation (Development)

1. Clone this repository
2. Open Zed
3. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux) to open the command palette
4. Run `zed: extensions`
5. Click "Install Dev Extension"
6. Select the cloned repository directory

## Setup

### 1. Create a Steepfile

In your Ruby project root, create a `Steepfile`:

```ruby
# Steepfile
target :app do
  signature "sig"
  check "app"
  library "pathname"
end
```

### 2. Set up type signatures

Create a `sig` directory and add your type signatures:

```bash
mkdir sig
# Add .rbs files with your type definitions
```

### 3. Configure Zed (Optional)

You can customize which language servers run for Ruby files in your Zed settings (`Cmd+,` or `Ctrl+,`):

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["steep", "..."]
    }
  }
}
```

Note: This extension currently runs `steep langserver` without additional configuration options. Steep's behavior is controlled through your `Steepfile` and project setup.

## Configuration

#### Default Behavior

By default, this extension sets `primary = false`, meaning Steep will work as a **secondary language server** alongside your primary Ruby language server. This prevents conflicts and provides the best of both worlds:

- **Primary Ruby LSP** (like `ruby-lsp` or `solargraph`): Handles general Ruby features
- **Steep LSP**: Adds type checking and type-aware intelligence

#### Recommended Configuration

For the best experience with multiple Ruby language servers:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "steep", "..."],
      "format_on_save": "language_server"
    }
  }
}
```

#### Advanced Language Server Selection

If you prefer Steep as your primary Ruby language server:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["steep", "!ruby-lsp", "!solargraph", "..."]
    }
  }
}
```

Or to disable Steep in specific projects:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "!steep", "..."]
    }
  }
}
```

### Multiple Language Servers Benefits

Using Steep alongside other Ruby language servers provides complementary benefits:

- **Steep**: Type checking, type-aware completions, signature help
- **ruby-lsp**: General Ruby support, formatting, basic completions
- **Solargraph**: Additional intellisense, documentation lookup

Each language server contributes different capabilities, and Zed merges their responses intelligently.

### Performance Optimization

To ensure optimal performance with multiple language servers:

#### 1. Project-Specific Configuration

Create a `.zed/settings.json` file in your project root for project-specific settings:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "steep"],
      "tab_size": 2,
      "format_on_save": "language_server"
    }
  }
}
```

Note: This extension doesn't currently support LSP initialization options. Steep's behavior is configured through your `Steepfile` and project structure.

## How It Works

The extension automatically detects Ruby projects with Steep configuration and starts the appropriate language server:

1. **Project Detection**: Looks for a `Steepfile` in the project root
2. **Execution Strategy**:
   - If `Gemfile` exists and `bundle` is available → Uses `bundle exec steep langserver`
   - If global `steep` is available → Uses `steep langserver`
   - Otherwise → Skips activation with helpful error message

3. **Language Server**: Communicates with Steep's built-in language server via the Language Server Protocol (LSP)

## Troubleshooting

### Extension Not Starting

**Check the logs**: Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux), then run `zed: open log`

**Common issues**:

1. **No Steepfile found**
   ```
   Solution: Create a Steepfile in your project root
   ```

2. **Steep executable not found**
   ```
   Solution: Install steep via bundle or globally
   gem install steep  # Global installation
   # OR add to Gemfile and run bundle install
   ```

3. **Bundle command not available**
   ```
   Solution: Install Bundler
   gem install bundler
   ```


## Development

### Building the Extension

```bash
cargo build --release
```

### Local Development

1. Make changes to the source code
2. Run `cargo build`
3. Reload the extension in Zed (restart Zed or use `zed: reload extensions`)

## Resources

- **Steep Documentation**: https://github.com/soutaro/steep
- **RBS Documentation**: https://github.com/ruby/rbs
- **Zed Extensions Guide**: https://zed.dev/docs/extensions
- **Language Server Protocol**: https://microsoft.github.io/language-server-protocol/

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This extension is available under the same license as Zed.
