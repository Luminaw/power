# Power

A minimal, high-performance CLI utility for managing Windows power states (Sleep and Hibernate), written in Rust.

## Features

- **Sleep**: Put your computer into suspend mode.
- **Hibernate**: Save your state to disk and power off completely.
- **Optimized**: Compiled with extreme optimizations for minimal binary size (opt-level = "z", LTO, etc.).
- **Safety Checks**: Verifies if the requested power state is allowed by the system before attempting to switch.
- **Force Mode**: Option to override safety checks if needed.

## Download

The easiest way to use Power is to download the pre-compiled binary from the **[Releases](https://github.com/Luminaw/power/releases)** page.

1. Go to the latest release.
2. Download `power.exe`.
3. Run it from your terminal!

## Usage

```powershell
power.exe <ACTION> [FLAGS]
```

### Actions
- `sleep`: Transitions the system to the suspend state.
- `hibernate`: Transitions the system to the hibernate state.

### Flags
- `-f`, `--force`: Force critical suspension, bypassing safety checks.
- `-d`, `--disable-wake`: Disables wake events (useful for preventing accidental wakes).
- `-h`, `--help`: Print help information.

### Examples

```powershell
# Put the system to sleep
power.exe sleep

# Hibernate the system immediately
power.exe hibernate

# Force sleep even if the system reports it's not allowed
power.exe sleep --force
```

## Requirements

- **Windows**: This tool uses the Win32 SetSuspendState API and is exclusive to Windows.
- **Administrator Privileges**: Changing power states often requires running the terminal as an Administrator.

## Build from Source (Optional)

If you prefer to build the project yourself, ensure you have [Rust](https://rustup.rs/) installed, then:

```powershell
git clone https://github.com/Luminaw/power
cd power
cargo build --release
```

The optimized binary will be located at `target/release/power.exe`.

## PowerShell Script

A PowerShell version of this utility is also provided in the `scripts/` directory for users who prefer scripts over binaries or want to integrate this into PowerShell workflows.

### Functions

- `Suspend-Computer`: Transitions the system to the suspend state (Sleep).
- `Stop-ComputerHibernate`: Transitions the system to the hibernate state.

### Usage

1. **Import the script**:
   ```powershell
   . ./scripts/power.ps1
   ```

2. **Run the functions**:
   ```powershell
   # Put the system to sleep
   Suspend-Computer

   # Hibernate the system forcefully
   Stop-ComputerHibernate -Force
   ```
