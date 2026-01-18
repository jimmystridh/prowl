# prowl

A fast, modern CLI for sending push notifications via the [Prowl](https://www.prowlapp.com/) API.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary at target/release/prowl
```

## Quick Start

```bash
# Set your API key (get one from https://www.prowlapp.com/)
export PROWL_API_KEY="your-api-key"

# Send a notification
prowl send "Hello from the command line!"

# That's it.
```

## Usage

```bash
# Simple notification
prowl send "Deployment complete"

# With custom event title and priority
prowl send "Build failed on main" -e "CI/CD" -p emergency

# Attach a URL (clickable in the iOS app)
prowl send "PR ready for review" -u "https://github.com/org/repo/pull/123"

# Pipe from stdin
tail -f /var/log/app.log | grep --line-buffered ERROR | while read line; do
  prowl send "$line" -e "Error"
done

# Or read entire stdin
cat report.txt | prowl send -

# Custom application name
prowl send "Job finished" -a "Cron"

# Multiple recipients
prowl send "Team standup in 5" -t "key1,key2,key3"

# See what would be sent
prowl send "Test" --dry-run
```

### Priority Levels

| Flag | Level | Description |
|------|-------|-------------|
| `-p very-low` | -2 | Lowest priority |
| `-p moderate` | -1 | Low priority |
| `-p normal` | 0 | Default |
| `-p high` | 1 | High priority |
| `-p emergency` | 2 | Bypass quiet hours |

## Configuration

Configuration is loaded in order of precedence:

1. Command-line flags (`-k`, `-a`, etc.)
2. Environment variables (`PROWL_API_KEY`, `PROWL_APPLICATION`)
3. Config file

### Config File

```bash
# Initialize config
prowl config init

# Set values
prowl config set api_key "your-api-key"
prowl config set application "my-server"

# Show current config
prowl config show

# Show config file path
prowl config path
```

Config file location:
- macOS: `~/Library/Application Support/prowl/config.toml`
- Linux: `~/.config/prowl/config.toml`

Example `config.toml`:

```toml
api_key = "abc123..."
application = "my-server"
```

## Commands

| Command | Description |
|---------|-------------|
| `prowl send <message>` | Send a push notification |
| `prowl verify` | Verify your API key is valid |
| `prowl config init` | Create config file |
| `prowl config show` | Show current configuration |
| `prowl config set <key> <value>` | Set a config value |
| `prowl completions <shell>` | Generate shell completions |

## Output Formats

```bash
# Human-readable (default)
prowl send "Hello"

# JSON output
prowl send "Hello" -F json

# Quiet (no output, exit code only)
prowl send "Hello" -F quiet
```

## Shell Completions

```bash
# Bash
prowl completions bash > /etc/bash_completion.d/prowl

# Zsh
prowl completions zsh > ~/.zfunc/_prowl

# Fish
prowl completions fish > ~/.config/fish/completions/prowl.fish
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error (invalid API key) |
| 3 | Rate limited |
| 4 | Token not approved |

## Examples

### CI/CD Integration

```bash
# GitHub Actions
- name: Notify on failure
  if: failure()
  run: prowl send "Build failed: ${{ github.repository }}" -e "GitHub" -p high
  env:
    PROWL_API_KEY: ${{ secrets.PROWL_API_KEY }}
```

### Cron Job Monitoring

```bash
#!/bin/bash
if ! /path/to/backup.sh; then
  prowl send "Backup failed on $(hostname)" -e "Backup" -p emergency
fi
```

### Log Monitoring

```bash
tail -F /var/log/syslog | grep --line-buffered "error" | while read line; do
  prowl send "$line" -e "Syslog Error" -p high
done
```

### Long-Running Command Notification

```bash
make build && prowl send "Build complete" || prowl send "Build failed" -p high
```

## API Reference

This CLI implements the [Prowl Public API](https://www.prowlapp.com/api.php):

- `POST /publicapi/add` - Send notification
- `GET /publicapi/verify` - Verify API key
- `GET /publicapi/retrieve/token` - Get registration token
- `GET /publicapi/retrieve/apikey` - Get API key from token

Rate limit: 1000 calls/hour per IP.

## License

MIT
