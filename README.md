# 🚀 HelixRT

> Experimental high-performance robotics runtime built in Rust.

HelixRT is a modular robotics simulation and telemetry platform focused on realtime runtime orchestration, autonomous navigation pipelines, sensor processing, and distributed telemetry streaming.

Built to explore how modern robotics infrastructure can be designed using async systems programming, concurrent runtime services, and modular robotics architecture.

---

# ✨ Features

* ⚡ Async robotics runtime powered by Tokio
* 🤖 Multi-agent robot simulation
* 📡 Realtime telemetry streaming over WebSockets
* 🛰️ Lidar scan simulation
* 🧭 Autonomous waypoint navigation
* 🗺️ Occupancy grid mapping
* 🧠 Localization and sensor fusion layer
* 🔄 Runtime scheduler and command bus
* 💾 Persistent runtime snapshots
* 📈 Live telemetry dashboard
* 🧩 Modular subsystem architecture
* 🌐 Concurrent websocket client handling

---

# 🏗️ Runtime Architecture

HelixRT is structured around independent runtime systems:

```text id="n8x8kl"
Runtime Engine
├── Robot Controllers
├── Telemetry Server
├── WebSocket Stream Layer
├── Navigation Grid
├── Localization Engine
├── Sensor Pipeline
├── Runtime Scheduler
├── Command Bus
└── Persistence Layer
```

The runtime is designed around isolated subsystems to keep orchestration scalable and maintainable.

---

# 🖥️ Live Dashboard

The dashboard visualizes:

* 📍 Realtime robot movement
* 📊 Telemetry metrics
* 🛰️ Occupancy state
* 🛣️ Robot trajectory trails
* 🌐 WebSocket streamed runtime packets

---

# ⚙️ Tech Stack

## Backend

* 🦀 Rust
* ⚡ Tokio
* 🔌 Tokio Tungstenite
* 📦 Serde
* 🔄 Async Runtime Systems

## Frontend

* 🟨 JavaScript
* ⚡ Vite
* 🎨 HTML5 Canvas

---

# 📂 Project Structure

```text id="vbblkh"
src/
├── autonomy/
├── bus/
├── fusion/
├── logging/
├── navigation/
├── network/
├── persistence/
├── robot/
├── runtime/
├── sensors/
├── simulation/
├── state/
├── tasks/
├── telemetry/
└── ws/
```

---

# 🚀 Running The Project

## Backend

```bash id="f43qz6"
cargo run
```

## Dashboard

```bash id="wm9d6m"
cd dashboard
npm install
npm run dev
```

---

# 🧠 Engineering Focus

This project explores:

* Realtime robotics runtime orchestration
* Async systems programming
* Telemetry infrastructure
* Robotics simulation pipelines
* Concurrent runtime services
* Navigation and mapping systems
* Modular robotics architecture

---

# 📸 Preview

* Live telemetry visualization
* Realtime robot movement
* Runtime metrics dashboard
* WebSocket streaming infrastructure

---

# 🛣️ Future Work

* 🧠 SLAM experimentation
* 🤝 Distributed robot coordination
* 🚘 ROS2 interoperability
* 🖼️ Advanced map rendering
* ⚙️ Runtime profiling systems
* 🧪 Hardware integration support
* 📷 Computer vision pipeline
