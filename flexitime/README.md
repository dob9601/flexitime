# Flexitime

WIP: A humantime-equivalent crate for parsing user-inputted time strings, with support for both relative and absolute time strings.

| Feature                   | Status   | Example(s)                        |
|---------------------------|----------|-----------------------------------|
| Day offsets               | ✅ Supported | `yesterday`, `tomorrow`, `next Monday`, `last Friday` |
| Wallclock time            | ✅ Supported | `9:00pm`, `21:00`, `07:30`        |
| Relative formats          | ✅ Supported | `2 years 3 months`, `5 days ago`, `in 2 weeks` |
| Date strings              | ✅ Supported | `2025-10-25`, `25-10-2025`, `10/25/2025` |
| Timezones                 | 🚧 Planned | `EST`, `UTC` |
| Month offsets             | 🚧 Planned | `next month`, `last month` |

## Cli

The cli can be installed via cargo:

```bash
cargo install flexitime-cli
```

Currently it will convert parsed time strings into ISO 8601 format with more features potentially coming soon.
