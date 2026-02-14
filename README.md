# FiTui

[![Built With Ratatui](https://ratatui.rs/built-with-ratatui/badge.svg)](https://ratatui.rs/)

A lightweight terminal-based personal finance tracker built in Rust. Record transactions, track spending, and view financial insights—all from your terminal.

**Version:** 0.1.4

---

## Features

- **Transaction Management** – Add, view, and delete credit/debit transactions
- **Smart Stats** – View totals (earned, spent, balance) and spending breakdowns by tag
- **Recurring Transactions** – Auto-insert monthly bills, salary, and subscriptions
- **Local & Private** – SQLite database with configurable tags and currency (YAML)
- **Keyboard-Driven** – Fast, efficient terminal UI

### Screenshots

| Main Interface | Stats View |
|----------------|------------|
| ![Main interface](assets/main_page.png) | ![Stats view](assets/stats_page.png) |

---

## Controls

| Mode | Key | Action |
|------|-----|--------|
| **Normal** | `↑/↓` | Navigate transactions |
| | `a` | Add transaction |
| | `d` | Delete selected |
| | `s` | Open stats |
| | `q` | Quit |
| **Form** | `Tab` | Next field |
| | `←/→` | Toggle type/tag/recurring |
| | `Enter` | Save |
| | `Esc` | Cancel |
| **Stats** | `Esc` | Back to main |

---

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) installed

### Build
```bash
cargo build --release
```

Binary location: `target/release/fitui` (Windows: `fitui.exe`)

### Install

**Linux / macOS**
```bash
mkdir -p ~/.local/bin
cp target/release/fitui ~/.local/bin/
chmod +x ~/.local/bin/fitui
fitui
```

**Windows**
1. Copy `fitui.exe` to a permanent location (e.g., `C:\Users\<you>\cli\`)
2. Add that folder to your PATH
3. Run `fitui` from any terminal

**Termux (Android)**
```bash
pkg install rust
cargo build --release
cp target/release/fitui ~/.local/bin/
fitui
```
*Note: First build may take 10-15 minutes on mobile devices.*

---

## Configuration

### File Locations

| OS | Database | Config |
|----|----------|--------|
| **Linux** | `~/.local/share/fitui/budget.db` | `~/.config/fitui/config.yaml` |
| **macOS** | `~/Library/Application Support/com.ayan.fitui/budget.db` | `~/Library/Preferences/com.ayan.fitui/config.yaml` |
| **Windows** | `AppData\Roaming\ayan\fitui\data\budget.db` | `AppData\Roaming\ayan\fitui\config\config.yaml` |

*Config file is auto-created on first run.*

### Tags & Currency

Edit `config.yaml` to customize:

```yaml
currency: ₹  # Common symbols: $, €, £, ¥, ₹, ₽, ₩, ฿, ₪

tags:
  - food
  - travel
  - shopping
  - bills
  - salary
  - other
```

---

## Recurring Transactions

Automate monthly transactions like salary, rent, and subscriptions.

**Setup:**
1. Add a transaction and press `Tab` to reach "Recurring" field
2. Toggle to `🔄 Yes` with `←/→`
3. Save with `Enter`

**Behavior:**
- Auto-inserts on the same date each month (e.g., created Feb 15 → auto-adds Mar 15, Apr 15, etc.)
- Prevents duplicates (only once per month)

**Use Cases:** Monthly salary, rent, subscriptions, insurance, utilities

---

## Planned Features

### 🚧 Coming Soon

- **Flexible Recurring Intervals** – Set transactions to repeat daily, weekly, or monthly
- **Delete Confirmation** – Confirmation dialog to prevent accidental deletions
- **Enhanced Stats Page** – More visualizations, charts, and filtering options
- **CSV Import** – Bulk import transactions from PayPal, GPay, bank statements, and other sources
- **Budget Goals & Alerts** – Set monthly spending limits per tag with notifications
- **Search & Filter** – Find transactions by amount, date range, tag, or description
- **Export Reports** – Generate CSV/PDF reports for tax or accounting purposes
- **Custom Date Ranges** – View stats for specific periods (last week, quarter, year)

### 💡 Under Consideration

- **Multi-Currency Support** – Track expenses in different currencies with conversion
- **Transaction Notes** – Add detailed descriptions or memos to entries
- **Split Transactions** – Assign a single expense to multiple tags
- **Data Backup/Sync** – Export/import database for backup or cross-device sync
- **Themes & Colors** – Customizable color schemes for the terminal UI

> Have a feature request? [Open an issue](https://github.com/ayanchavand/fitui/issues) or contribute!

---

## License

MIT