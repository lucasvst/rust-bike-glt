# Rust Bike GLT Monitor 🚴‍♂️

A simple Command Line Interface (CLI) tool written in Rust to connect to **Gallant Smart Spinning** bikes (and potentially other compatible "GLT" Bluetooth models) via Bluetooth Low Energy (BLE).

It calculates and displays real-time workout metrics by decoding the raw data packets sent by the bike.

## 🚀 Features

- **Auto-Discovery**: Scans for Bluetooth devices named "GLT" or "Gallant".
- **Real-Time Dashboard**: Displays a live-updating console dashboard with:
  - 🚀 **Speed** (km/h)
  - 🔄 **Cadence** (RPM)
  - ⚡ **Power** (Watts)
  - 📏 **Distance** (meters - calculated)
  - ⏱️ **Elapsed Time** (from bike MCU)

## 🛠️ Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install).
- **Bluetooth Adapter**: A computer with Bluetooth 4.0+ support.
  - **Linux**: May require `libdbus-1-dev` and `pkg-config`.
  - **macOS/Windows**: Native support usually works out of the box.

## 📦 Installation

Clone this repository:

```bash
git clone https://github.com/your-username/rust-bike-glt.git
cd rust-bike-glt
```

## 🏃 Usage

1. **Activate the Bike**: Start pedaling to wake up the bike's Bluetooth transmitter.
2. **Run the Monitor**:

```bash
cargo run
```

3. **Wait for Connection**: The tool will scan for 15 seconds. Once connected, the dashboard will appear.

### Troubleshooting

- **"Adaptador não encontrado"**: Ensure your Bluetooth adapter is enabled and not blocked.
- **"Bicicleta não encontrada"**: Make sure you are pedaling so the bike is broadcasting.
- **Incompatible Metrics**: The conversion factors for Speed, Power, and RPM are reverse-engineered for a specific Gallant model. If your values look wrong, you may need to adjust the multipliers in `src/main.rs`.

## 🧩 How it Works

The tool subscribes to the standard Indoor Bike Data characteristic (`00002ad2-0000-1000-8000-00805f9b34fb`) and decodes two specific packet formats found in Gallant bikes:
- **Length 18**: Contains Speed and Power data.
- **Length 6**: Contains Cadence and Time data.

Distance is calculated locally based on speed over time updates.

## 📝 License

This project is a Proof of Concept (PoC). Feel free to modify and use it as you wish.
