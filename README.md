# ⚡ Spike-Spy

**Spike-Spy** is a lightweight, terminal-based CPU spike monitoring tool. Perfect for casual users, developers, and sysadmins, it tracks CPU usage in real-time, highlighting offending processes for quick analysis or automated logging.

## 🚀 Features

* Real-time CPU spike detection.
* Configurable poll and update intervals for precise monitoring.
* Display top CPU offenders, all processes, or processes exceeding a specified dynamic threshold.
* Human-readable output including process name, PID, CPU delta, and memory usage.
* Cross-platform support: Linux and macOS (Windows users can compile from source).
* Homebrew support: Simple install via Homebrew for macOS users.

## 💾 Installation

### Manual Installation

Clone the repository and compile with Rust:

```bash
git clone https://github.com/leekalan/spike-spy
cd spike-spy
cargo build --release
```

The compiled binary will be in `target/release/spike-spy`.

### Homebrew (macOS)

## 🛠 Usage

```bash
spike-spy [OPTIONS] [POLL_INTERVAL] [SPIKE_THRESHOLD] [UPDATE_INTERVAL]
```

### Arguments


| Argument          | Description                           | Default |
| ------------------- | --------------------------------------- | --------- |
| `POLL_INTERVAL`   | Poll interval in milliseconds (>=0)   | `1000`  |
| `SPIKE_THRESHOLD` | CPU spike threshold (0-100%)          | `10`    |
| `UPDATE_INTERVAL` | Update interval in milliseconds (>=0) | `5000`  |

### Options


| Option                    | Description                                                                                                                                                     |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-n`, `--no-update`       | Disable update interval.                                                                                                                                        |
| `-s`, `--suppress-update` | Suppress update logging.                                                                                                                                        |
| `-a`, `--all`             | Display all processes in the update.                                                                                                                            |
| `-o`, `--over <OVER>`     | Display processes exceeding a threshold of max CPU usage (0-100%). For example,`--over 50` shows processes above 50% of the maximum CPU delta of all processes. |
| `-t`, `--top <TOP>`       | Display top N CPU offenders.                                                                                                                                    |
| `-h`, `--help`            | Show help information.                                                                                                                                          |
| `-V`, `--version`         | Show version information.                                                                                                                                       |

## 📊 Output Example

**Using default settings:**

```
[CPU usage spike detected! ΔCPU=72.12%]
Top CPU offenders (over last 4983ms):
  firefox (PID 1234): ΔCPU=45.32%, MEM=12 GB
  code (PID 5678): ΔCPU=37.18%, MEM=800 MB
```

## ⚙️ Notes

* CPU deltas are calculated over the polling interval, giving a snapshot of CPU usage change.
* Memory is displayed in a human-readable format.
* Update intervals adjust the elapsed time shown in snapshots for higher accuracy.
* Output is console-friendly and can be piped to other tools for automated logging or analysis.

## 💡 Example Commands

**Monitor CPU spikes every second, threshold 15%, update every 5 seconds:**

```bash
spike-spy 1000 15 5000
```

**Show top 5 CPU offenders every 2 seconds:**

```bash
spike-spy 2000 --top 5
```

**Display all processes exceeding 50% of the maximum CPU delta of all processes:**

```bash
spike-spy --over 50
```

## 📜 License

This project is licensed under the **MIT License**.
