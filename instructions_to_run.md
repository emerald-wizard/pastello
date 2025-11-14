### Part 1: One-Time Setup (or after changing Protobufs)

You only need to do these steps once, or any time you make a change to your `.proto` files.

**Run all of these from your project's root `pastello` directory:**

1.  **Install All Dependencies:**

    ```bash
    pnpm install
    ```

    This reads your `pnpm-workspace.yaml` and installs all dependencies for the root, `apps/web`, and `packages/contracts` at once.

2.  **Generate Protobuf Code:**

    ```bash
    buf generate
    ```

    This reads your `buf.yaml` and `buf.gen.yaml` and creates the raw `gen/ts` and `gen/go` files.

3.  **Build Your Contracts Package:**

    ```bash
    pnpm build:contracts
    ```

    This is the script from your root `package.json` that runs the `build` command in `packages/contracts`. It compiles the `gen/ts` code into the final `dist` folder that your Vue app imports.

-----

### Part 2: Daily Workflow (Running the App)

This is what you'll do every day. You'll need **two terminals.**

#### Terminal 1: Start the Rust Backend

1.  Navigate to the Rust app directory:
    ```bash
    cd apps/api-rust
    ```
2.  Run the server:
    ```bash
    cargo run
    ```
3.  Leave this terminal running. You should see it listen on `0.0.0.0:8080`.

#### Terminal 2: Start the Vue Frontend

1.  Navigate to your project's **root `pastello` directory**.
2.  Run the `dev:web` script from your root `package.json`:
    ```bash
    pnpm dev:web
    ```
3.  This will start the Vite server, and you can now open the app in your browser\!