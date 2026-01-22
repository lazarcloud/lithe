# State, Storage, and Telepathy

Lithe.rs features a surgical reactivity system designed for high performance and seamless cross-tier synchronization.

## 1. Copy-able Signals
Signals are lightweight `u32` IDs that point to a Global Arena. This allows them to be `Copy`, avoiding `.clone()` and `Rc<RefCell<T>>`.
```rust
let count = create_signal(0);
button().on_click(move |_| count.set(count.get() + 1));
```

## 2. Universal Storage
A single API to persist state across all tiers.
- **Web:** Persists to `localStorage`.
- **Native:** Persists to `settings.json` or SQLite.
- **Server:** Persists to the primary Database.
```rust
let theme = create_persisted_signal("user_theme", Theme::Light);
```

## 3. Telepathy (Live Sync)
Telepathy allows server-side signals to automatically sync with the client via WebSockets/SSE.
```rust
#[server]
fn get_live_stats() -> Signal<Stats> {
    // This signal will "telepathically" update all connected clients
    // whenever the server-side value changes.
    TELEMETRY_SIGNAL.clone()
}
```

## 4. Optimistic UI
Update the UI instantly while an action is in flight.
```rust
action.execute_optimistic(|state| {
    state.likes += 1;
});
// If the server call fails, Lithe automatically rolls back the signal.
```

## 5. Observables (Streams)
For complex data flows that are time-dependent, Lithe integrates Observables. Unlike Signals which represent a value, Observables represent a stream of events.
```rust
let search_results = search_input
    .as_observable()
    .debounce(Duration::from_millis(300))
    .switch_map(|query| fetch_results(query));
```

## 6. State Machines
Formalize complex UI logic using Finite State Machines. This prevents "impossible states" and makes transitions predictable.
```rust
let machine = create_machine(VideoPlayer {
    state: Idle,
    transitions: [
        (Idle, Play) => Playing,
        (Playing, Pause) => Paused,
        (Paused, Play) => Playing,
    ]
});
```
