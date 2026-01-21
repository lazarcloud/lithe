# 🗺️ Lithe.rs Implementation Roadmap

This roadmap outlines the step-by-step development of the Lithe.rs framework. Each step is designed to be small, incremental, and highly testable.

---

## Phase 1: The Reactive Core (`lithe-core`)
*Goal: Create the fine-grained reactivity system that avoids the Virtual DOM.*

- [ ] **1.1 Signal Identity:** Define `SignalId` as a lightweight `u32` wrapper.
- [ ] **1.2 The Arena:** Implement `SignalArena` to store any type of data indexed by `SignalId`. Use a `SlotMap` or similar for stable, O(1) access.
- [ ] **1.3 Dependency Tracking:** Implement a thread-local "Tracking Stack" to record which signals are accessed during an effect.
- [ ] **1.4 Basic Signals:** Implement `create_signal<T>(value)`.
    - **Test:** Verify `signal.get()` returns the value and `signal.set()` updates it.
- [ ] **1.5 Effects:** Implement `create_effect(f)`. 
    - **Test:** Verify the function re-runs automatically when a tracked signal changes.
- [ ] **1.6 Memos:** Implement `create_memo(f)` for derived computations.
    - **Test:** Verify memos only re-calculate if their dependencies change and are themselves trackable.
- [ ] **1.7 Batching:** Implement a `batch(|ui| { ... })` function to prevent multiple re-runs during multiple signal updates.

## Phase 2: UI Representation & SSR (`lithe-core` / `lithe-ui`)
*Goal: Represent the UI as a tree of Rust structures that can render to HTML.*

- [ ] **2.1 The View Trait:** Define a `View` trait with a `render()` method that returns HTML strings.
- [ ] **2.2 Element Primitives:** Implement the `Element` struct for common tags (`div`, `button`, etc.).
- [ ] **2.3 Pattern A (Append):** Implement the `.child()` and `.append()` methods for the Builder Pattern.
    - **Test:** Build a nested tree and verify the generated HTML string matches expectations.
- [ ] **2.4 Attribute Types:** Create enums for `Attribute` (Id, Class, Value) and `Event` (Click, Input).
- [ ] **2.5 Suspense:** Implement a `Suspense` component that takes a `Future` and shows a fallback while loading.
- [ ] **2.6 Error Boundaries:** Implement an `ErrorBoundary` component to catch panics in the UI tree.

## Phase 3: WASM Runtime & Resumability (`lithe-core`)
*Goal: Allow the UI to "wake up" in the browser without re-rendering everything.*

- [ ] **3.1 DOM Mounting:** Implement `mount_to_body(app)` using `web-sys`.
- [ ] **3.2 Surgical Text Nodes:** Implement logic to bind a `Signal<String>` to a raw DOM `Text` node.
    - **Test:** Verify that updating the signal updates the text in the browser WITHOUT touching the parent element.
- [ ] **3.3 Surgical Attributes:** Implement logic to bind signals to DOM attributes (e.g., `class`, `disabled`).
- [ ] **3.4 Resumability (Serialization):** Implement logic on the server to serialize the `SignalArena` into a JSON blob inside the HTML.
- [ ] **3.5 Resumability (Deser/Resume):** Implement logic in WASM to initialize the `SignalArena` from the JSON blob and re-attach event listeners.

## Phase 4: Procedural Macros & The Bridge (`lithe-macros`)
*Goal: Provide the "Magic" syntax and the cross-tier communication.*

- [ ] **4.1 Pattern B (Declarative Macro):** Implement the `div!( ... )` macro to allow HTML-like nesting in Rust.
- [ ] **4.2 The `#[island]` Macro:** Create a macro that marks a component for WASM compilation and generates the hydration marker.
- [ ] **4.3 The `#[server]` Macro:**
    - **SSR mode:** Compiles to an Axum/Actix route.
    - **WASM mode:** Compiles to a type-safe `fetch()` call.
- [ ] **4.4 The `#[native]` Macro:**
    - **Native mode:** Compiles to a Tauri command.
    - **Client mode:** Compiles to a `tauri::invoke()` call.
- [ ] **4.5 Middlewares:** Add support for `#[middleware]` on server functions for Auth/Logging.

## Phase 5: Routing & Navigation
*Goal: Support both Website (MPA) and App (SPA) modes.*

- [ ] **5.1 URL Matcher:** Implement a basic regex-based router.
- [ ] **5.2 MPA Mode:** Standard link behavior with full page reloads.
- [ ] **5.3 SPA Mode:** Client-side routing that intercepts link clicks and performs a WASM-based transition.
- [ ] **5.4 View Transitions:** Integrate the browser's View Transition API for seamless "morphing" between routes.

## Phase 6: Full-Stack Features
*Goal: High-level features for real-world apps.*

- [ ] **6.1 Universal Storage:** Implement `create_persisted_signal` with drivers for LocalStorage and Server Databases.
- [ ] **6.2 Form Actions:** Implement the `#[action]` macro for forms with isomorphic validation logic.
- [ ] **6.3 Optimistic UI:** Add the `.execute_optimistic()` method to Actions for instant UI feedback.
- [ ] **6.4 Telepathy:** Implement WebSocket sync for `Signal` objects that should live-sync between server and all clients.

## Phase 7: Styling & Assets
*Goal: Type-safety for the "Look and Feel".*

- [ ] **7.1 Style Builder:** Implement a typed `Style` struct (e.g., `.bg_color(Color::Red)`).
- [ ] **7.2 Atomic CSS Extractor:** Build a compile-time tool that extracts these styles into a single `.css` file.
- [ ] **7.3 Asset Macros:** Implement `img!` and `svg!` for compile-time resizing and SVG-to-Rust conversion.
- [ ] **7.4 Type-Safe i18n:** Implement the `t!` macro that validates keys against translation files at compile time.

## Phase 8: CLI & Distribution (`lithe-cli`)
*Goal: Ship the framework to the world.*

- [ ] **8.1 Project Scaffolding:** `lithe init` command to generate the 3-tier workspace.
- [ ] **8.2 Dev Engine:** `lithe dev` with HMR and proxying between Axum and WASM.
- [ ] **8.3 Cross-Compilation:** `lithe build` targets for Linux, Windows, Mac, Android, iOS, and Web.
- [ ] **8.4 NPM/Bun Package:** Create the binary wrapper for `@lithe/cli`.
