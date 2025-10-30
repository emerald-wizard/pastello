So now how do I run my web app?

Here’s the quickest way to get your web app running from your **monorepo**.

## 0) From repo root, install deps

```bash
pnpm install
```

## 1) Build (or watch) the contracts package

You only need this when protos change, but do it once now so the web can import the built ESM.

```bash
# one-off build
pnpm -F @runecraft-studios/pastello-contracts build

# or keep it up to date while you dev:
pnpm -F @runecraft-studios/pastello-contracts build --watch
```

> If that fails, make sure `packages/contracts/package.json` has `devDependencies.typescript` and `dependencies["@bufbuild/protobuf"]`, and your `src/index.ts` imports **files**, not folders.

## 2) Run the Vite dev server

```bash
# either target the path…
pnpm --filter ./apps/web dev

# …or if you added a root script:
pnpm dev:web
```

Vite will print a URL like `http://localhost:5173/`.

## 3) (If you call your API) set the env URL

Create `apps/web/.env` (or `.env.local`) with your API/WS endpoint:

```
# HTTP example:
VITE_API_HTTP_URL=http://localhost:8080

# WebSocket example:
VITE_API_WS_URL=ws://localhost:8080/ws
```

Then in your code:

```ts
const ws = new WebSocket(import.meta.env.VITE_API_WS_URL);
```

## 4) (Optional) run API and Web together

If you added the `concurrently` script at the root:

```bash
pnpm dev:all
```

That runs `apps/web` and your Go server (`make -C apps/api dev`) at once.

---

### Quick checklist if it doesn’t start

* **Workspace includes the web app**: `pnpm-workspace.yaml` has

  ```
  packages:
    - "apps/*"
    - "packages/*"
  ```
* **Web depends on contracts**:
  `apps/web/package.json` includes `"@runecraft-studios/pastello-contracts": "workspace:*"`.
* **.vite cache**: if you changed `cacheDir` to `.vite`, it’s ignored in `.gitignore`. To clean:

  ```bash
  rm -rf apps/web/.vite
  ```
* **Port already in use**: `pnpm --filter ./apps/web dev -- --port 5174`
* **CORS / WS origin**: if API rejects, allow `http://localhost:5173` in your server’s CORS/WS origin check.

That’s it—run step 1 once, then step 2 is your normal “start the SPA” command.



I'm getting this: packages/contracts $ pnpm -F @runecraft-studios/pastello-contracts build > @runecraft-studios/pastello-contracts@0.0.0 build /Users/michaelrodriguez/Library/Mobile Documents/com~apple~CloudDocs/pastello/packages/contracts > tsc -p tsconfig.json node:internal/modules/cjs/loader:1372 throw err; ^ Error: Cannot find module '/Users/michaelrodriguez/Library/Mobile Documents/com~apple~CloudDocs/pastello/packages/contracts/node_modules/typescript/bin/tsc' at Module._resolveFilename (node:internal/modules/cjs/loader:1369:15) at defaultResolveImpl (node:internal/modules/cjs/loader:1025:19) at resolveForCJSWithHooks (node:internal/modules/cjs/loader:1030:22) at Module._load (node:internal/modules/cjs/loader:1179:37) at TracingChannel.traceSync (node:diagnostics_channel:322:14) at wrapModuleLoad (node:internal/modules/cjs/loader:235:24) at Module.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:152:5) at node:internal/main/run_main_module:33:47 { code: 'MODULE_NOT_FOUND', requireStack: [] } Node.js v24.3.0  ERR_PNPM_RECURSIVE_RUN_FIRST_FAIL  @runecraft-studios/pastello-contracts@0.0.0 build: tsc -p tsconfig.json Exit status 1



That error means the **contracts package doesn’t have TypeScript installed/linked**, so the script `tsc -p tsconfig.json` can’t find `tsc` in `packages/contracts/node_modules/.bin`.

Do this from the **repo root**:

1. Make sure the workspace includes the package

```yaml
# pnpm-workspace.yaml
packages:
  - "apps/*"
  - "packages/*"   # <- must include contracts
```

2. Install deps (creates node_modules links for each workspace)

```bash
pnpm install
```

3. Ensure the contracts package has the right deps

```bash
pnpm -F @runecraft-studios/pastello-contracts add -D typescript
pnpm -F @runecraft-studios/pastello-contracts add @bufbuild/protobuf
```

4. (Optional but robust) change the build script to use `pnpm exec`
   In `packages/contracts/package.json`:

```json
"scripts": {
  "build": "pnpm exec tsc -p tsconfig.json"
}
```

This resolves `tsc` from the workspace even if it’s only at the root.

5. Build

```bash
pnpm -F @runecraft-studios/pastello-contracts build
```

If it still fails, quick checks:

* `ls packages/contracts/node_modules/.bin/tsc` → should exist (symlink).
* `pnpm why typescript` → confirms where TS is installed.
* Path with spaces is fine, but if you see weirdness, try from a path without spaces (iCloud Drive sometimes causes odd issues with watchers).
