This document serves as a comprehensive architectural blueprint for the **LZR Framework**: a polyglot, full-stack component system that unifies server-side logic, client-side reactivity, and styling into a single file format.

---

# **Technical Design Document: The LZR Framework**

## **1. Executive Summary**

**LZR** (`.lzr`) is a new file format that enables "Single File Full-Stack Components." It allows developers to write backend logic (in Rust, Go, Python, or TypeScript), frontend logic (TypeScript), and reactive UI (HTML/CSS) in one cohesive unit.

**Key Capabilities:**

* **Polyglot Backend:** The `<server>` block supports multiple languages but compiles to a single binary.
* **Zero-Boilerplate RPC:** Functions exported from the server block are automatically available as `async` functions in the client block.
* **Native DX:** Full IntelliSense, "Go to Definition," and type safety across the server/client boundary using VS Code.
* **High Performance:** The backend compiles to a native binary (e.g., Rust/Go), and the frontend compiles to efficient, surgical DOM updates (Svelte-style).

---

## **2. The File Format (`.lzr`)**

A single `.lzr` file is split into four distinct sections.

```html
<server lang="rust">
    use serde::{Serialize, Deserialize};

    #[derive(Serialize)]
    pub struct User { id: u32, name: String }

    // This function is automatically exposed as an API endpoint
    #[expose] 
    pub fn get_user_data(id: u32) -> User {
        User { id, name: "Alice".to_string() }
    }
</server>

<client lang="ts">
    // MAGIC: Import directly from the sibling server block
    // The build system rewrites this into a fetch() call
    import { get_user_data } from "./server";

    let user = $state(null);

    async function load() {
        // TypeScript knows 'id' is u32 and returns 'User'
        user = await get_user_data(101);
    }
</client>

<template>
    <div class="card">
        <h1>{user?.name ?? "Loading..."}</h1>
        <button on:click={load}>Reload</button>
    </div>
</template>

<style>
    .card { border: 1px solid #ccc; padding: 1rem; }
    h1 { color: blue; }
</style>

```

---

## **3. Architecture Part I: The Developer Experience (VS Code)**

*Goal: Provide seamless IntelliSense, linting, and cross-language type checking.*

We do not build a monolithic LSP. Instead, we use **Volar.js** as a "Hub" that delegates work to existing specialized tools.

### **The "Shadow File" Strategy**

Since `rust-analyzer`, `gopls`, and `Pylance` expect real files on disk, we cannot keep code purely in memory.

1. **The Virtualizer (Volar Extension):**
* Intercepts `.lzr` file events.
* Extracts the `<server>` content.
* Writes it to a hidden "Shadow Directory": `.lzr/shadow/user-card.rs`.
* Maps cursor positions between the real `.lzr` file and the shadow file.


2. **The Type Bridge (LSP Middleware):**
* Watches the Shadow File for changes.
* Runs a specialized CLI tool to generate TypeScript definitions (`.d.ts`).
* Injects these definitions into the `<client>` virtual environment.



### **Tooling Stack by Language**

| Feature | Tool / Library | Role |
| --- | --- | --- |
| **Extension Framework** | **Volar.js** | Manages virtual documents and mapping. |
| **Rust Bridge** | `syn` + `specta` | Parses Rust structs -> Outputs TS Interfaces. |
| **Go Bridge** | `tygo` | Parses Go structs -> Outputs TS Interfaces. |
| **Python Bridge** | `pydantic-to-ts` | Parses Pydantic models -> Outputs TS Interfaces. |
| **Frontend Logic** | TypeScript Server | Standard TS analysis provided by VS Code. |

---

## **4. Architecture Part II: The Compiler (Build Time)**

*Goal: Compile the separated blocks into a single deployable artifact.*

The compiler is a CLI tool (likely written in Rust for speed) that runs a 4-stage pipeline.

### **Stage 1: Parsing & Splitting**

* **Tool:** `swc` (HTML/JS) + `nom` or `pest` (File splitting).
* **Action:** Breaks `component.lzr` into `Server AST`, `Client AST`, `Template AST`, and `Style AST`.

### **Stage 2: Backend Generation (The "Wrapper")**

Transforms the user's simple function code into a robust HTTP server.

* **Step A:** Scans the code for exported functions (e.g., `get_user_data`).
* **Step B:** Generates an entry point file (`main.rs` / `main.go`).
* **Step C:** Wraps the user's functions in HTTP handlers (Axum/Gin/FastAPI).
* **Step D:** Compiles to a native binary.

### **Stage 3: Frontend Compilation (The "Hydrator")**

Transforms the UI code into a JavaScript bundle.

* **Step A (RPC Rewrite):** Finds imports from `./server`. Replaces function calls with `fetch('/api/component_name/function_name')`.
* **Step B (Reactivity):** Compiles HTML templates into JavaScript DOM instructions (Create, Mount, Update, Destroy) similar to Svelte.
* **Step C (CSS Scoping):** Hashes CSS classes (e.g., `.card-xyz123`) and applies them to HTML elements to prevent style leaks.

### **Stage 4: Bundling**

* **Tool:** `vite` or `rolldown` (Rust port of Rollup).
* **Action:** Bundles all client logic into `bundle.js` and `style.css`. Places them in a `dist/public` folder served by the backend binary.

---

## **5. Implementation Roadmap**

### **Phase 1: The "Volar" Foundation**

* Initialize a VS Code extension using the **Volar** starter template.
* Implement `createVirtualCode` to split `.lzr` files into virtual TS/HTML/CSS files.
* **Result:** You see `.lzr` files with syntax highlighting and basic JS/CSS autocomplete.

### **Phase 2: The Rust Bridge (Proof of Concept)**

* Implement the "Shadow File" writer in the extension.
* Create the `lzr-rust-bridge` CLI tool (using `syn` crate).
* Hook them together: Editing `<server lang="rust">` updates the generic TS types in `<client>`.
* **Result:** You can import a Rust struct into the TS block.

### **Phase 3: The Polyglot Expansion**

* Add logic to detect `lang="go"` and `lang="python"`.
* Integrate `tygo` (Go) and `pydantic` (Python) type generators.
* **Result:** One file format supports three different backend languages.

### **Phase 4: The Compiler (Reactivity)**

* Build the `lzr build` CLI.
* Implement the HTML-to-JS compiler (parsing `{variable}` and outputting DOM code).
* **Result:** `lzr build` outputs a `index.html` that actually renders data.

### **Phase 5: The Compiler (RPC & Binary)**

* Implement the HTTP Wrapper generation (wrapping the user's code in Axum/Gin).
* Implement the client-side `fetch` rewriter.
* **Result:** A fully functional app where clicking a button in HTML triggers a Rust function on the server.