# Lithe.rs Evolutionary Roadmap

This roadmap is structured into **Functional Milestones**. After each milestone, you will have a working, testable framework with an increasing set of capabilities.

---

## Milestone 1: The Static Renderer (SSR Only)
*Goal: A pure Rust library that renders HTML strings on the server.*

- [ ] **1.1 The View Trait:** Define `View` with `render_to_string()`.
- [ ] **1.2 Element Builders:** Implement `div()`, `span()`, `p()`, `button()`.
- [ ] **1.3 Pattern A (Append):** Support `.child()` and `.push()` for dynamic tree building.
- [ ] **1.4 Static Attributes:** Support `.class()`, `.id()`, and custom `.attr()`.
- [ ] **1.5 Unit Test Suite:** Verify complex nested trees render to correct HTML.
- **Outcome:** You can build a website in Rust and serve it via Axum as static HTML.

## Milestone 2: The Reactive Client (WASM Interactivity)
*Goal: Surgical DOM updates in the browser without re-rendering.*

- [ ] **2.1 Signal Arena:** Implement the `SignalArena` (SignalIds and Value storage).
- [ ] **2.2 Basic Reactivity:** `create_signal`, `create_effect`, and dependency tracking.
- [ ] **2.3 WASM Mounting:** Implement `mount_to_body()` using `web-sys`.
- [ ] **2.4 Surgical Binding:** Link a `Signal<String>` to a DOM `TextNode`.
- [ ] **2.5 Event Listeners:** Implement `.on_click()` using global event delegation.
- **Outcome:** A "Counter" app where only the number updates in the DOM when the button is clicked.

## Milestone 3: The Resumable Bridge (Automatic State Sync)
*Goal: State created on the server "wakes up" in the browser without a full reload.*

- [ ] **3.1 Arena Serialization:** Use `serde` to turn the Server's Signal Arena into JSON.
- [ ] **3.2 State Injection:** Automatically inject `<script id="lithe-state">` into the SSR output.
- [ ] **3.3 Resumption Logic:** WASM client boots by reading the JSON and "resuming" the Arena.
- [ ] **3.4 Island Markers:** Implement `data-lithe-island` attributes to tell WASM where to attach.
- **Outcome:** You set a counter to `5` on the server; the browser starts at `5` and continues counting.

## Milestone 4: The Cloud Bridge (Type-Safe RPC)
*Goal: Call server functions from the browser as if they were local.*

- [ ] **4.1 `#[server]` Macro:** Implemet the procedural macro to split logic.
- [ ] **4.2 RPC Transport:** Implement the Fetch-based client inside `lithe-core`.
- [ ] **4.3 Simple API Generation:** The compiler generates an Axum route for every `#[server]` function.
- [ ] **4.4 Async Resources:** Implement `create_resource` to wrap async server calls in a signal.
- **Outcome:** Clicking a button in WASM triggers a database save on the server with full type safety.

## Milestone 5: The Native Bridge (Desktop & Mobile)
*Goal: Access OS hardware and file systems via Tauri.*

- [ ] **5.1 `#[native]` Macro:** Implement the macro to route calls to the Tauri Main Process.
- [ ] **5.2 IPC Transport:** Implement `tauri::invoke` integration in the WASM client.
- [ ] **5.3 Unified Build:** Ensure the CLI can bundle the project into a Tauri app.
- **Outcome:** One codebase that runs in a browser (calling APIs) and a Desktop app (reading files).

## Milestone 6: The Modern Meta-Framework
*Goal: Polish the DX with professional-grade features.*

- [ ] **6.1 SPA Routing:** Client-side navigation without page refreshes + View Transitions.
- [ ] **6.2 Type-Safe Styling:** The `Style` builder and Atomic CSS extraction.
- [ ] **6.3 Optimistic UI:** `action.execute_optimistic()` with rollback on failure.
- [ ] **6.4 Telepathy:** WebSocket-based real-time signal syncing.
- [ ] **6.5 Asset Pipeline:** `img!` and `svg!` compile-time optimization.
- **Outcome:** A full-featured, competitive Rust alternative to Next.js or Astro.

## Milestone 7: Ecosystem & Distribution
*Goal: Ship Lithe.rs to the community.*

- [ ] **7.1 The `lithe` CLI:** Finalize `init`, `dev` (with HMR), and `build`.
- [ ] **7.2 NPM/Bun Wrapper:** Publish the platform-specific binary wrappers.
- [ ] **7.3 Documentation:** Generate the "Lithe.rs Book" and API references.
- **Outcome:** Users can `bun install @lithe/cli` and start building apps instantly.
