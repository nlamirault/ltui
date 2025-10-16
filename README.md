# ltui

A Terminal UI for Linear - A TUI client for managing Linear issues and projects.

## Features

- 🚀 **Fast Terminal Interface**: Navigate Linear issues, projects, and teams with keyboard shortcuts
- 📋 **Issue Management**: View, browse, and manage Linear issues
- 👥 **Team Support**: Switch between teams and view team-specific data
- 🎯 **Project Tracking**: Browse and monitor project progress
- ⚡ **Real-time Updates**: Auto-refresh data to stay up to date
- 🎨 **Intuitive UI**: Clean, responsive interface inspired by k9s and similar tools

## Installation

### Prerequisites

- Rust 1.70+ installed
- Linear Personal API Key (not OAuth2)

### Build from source

```bash
git clone https://github.com/nlamirault/ltui
cd ltui
cargo build --release
```

The binary will be available at `target/release/ltui`.

## Configuration

### Personal API Key

You can provide your Linear Personal API Key in several ways:

1. **Environment variable**: `export LINEAR_API_KEY="your-api-key-here"`
2. **Command line argument**: `ltui --apikey your-api-key-here`
3. **Config file**: Set `api_key` in `~/.config/ltui/config.toml`

### Configuration File

ltui creates a configuration file at `~/.config/ltui/config.toml` with the following default settings:

```toml
refresh_interval = 30
default_team_id = ""

[theme]
primary_color = "blue"
secondary_color = "cyan"
background_color = "black"
text_color = "white"
```

## Usage

```bash
# Run with token from environment variable
LINEAR_API_KEY="your-apikey" ltui

# Run with token as argument
ltui --apikey your-apikey

# Show help
ltui --help
```

## Keyboard Shortcuts

### Navigation

- `Tab`/`Shift+Tab` - Switch between views
- `j`/`k` or `↓`/`↑` - Navigate up/down in lists
- `1`/`2`/`3` - Jump to Issues/Projects/Teams view
- `Enter` - Select item (e.g., switch to team's issues)

### Actions

- `r` - Refresh current view
- `?` - Toggle help screen
- `q` or `Ctrl+C` - Quit application

### Views

#### Issues View (1)

- Browse team issues with priority indicators
- View issue details including assignee, status, and description
- Color-coded priorities and states

#### Projects View (2)

- Browse team projects
- View project details including lead, status, and target dates
- Track project progress

#### Teams View (3)

- Browse all available teams
- Select a team to view its issues and projects
- View team descriptions and keys

## API Permissions

ltui requires a Linear Personal API Key with the following permissions:

- Read issues
- Read projects
- Read teams
- Read users

You can create a Personal API Key in your Linear account settings under "API" → "Personal API keys".

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [k9s](https://github.com/derailed/k9s) for Kubernetes
- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses [Linear's GraphQL API](https://linear.app/developers/graphql) for data access
