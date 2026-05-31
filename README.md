# MMFG (Memory Mapped Fast Gateway)

MMFG is a high-performance, low-latency IPC framework using shared memory for zero-copy data transfer. It utilizes a Hub-and-Node architecture to facilitate chained processing of data across multiple processes.

## Quick Start

1. Initialize a Hub:
   ```go
   h, _ := hub.NewHub()
   ```
2. Request a local connection and transfer to a Node:
   ```go
   conn, _ := h.Request(ctx, 2, false)
   conn.Write([]byte("Data"))
   conn.Next("TargetNode")
   ```

For detailed architectural information, protocol specifications, and API references, please see the [docs](docs) directory.

- [Architecture Overview](docs/architecture.md)
- [Chained Processing Protocol](docs/handoff.md)
- [API Reference](docs/api.md)
