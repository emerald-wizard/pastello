I'm getting this: $ pnpm -F @runecraft-studios/pastello-contracts build > @runecraft-studios/pastello-contracts@0.0.0 build /Users/michaelrodriguez/Library/Mobile Documents/com~apple~CloudDocs/pastello/packages/contracts > tsc -p tsconfig.json sh: tsc: command not found  ERR_PNPM_RECURSIVE_RUN_FIRST_FAIL  @runecraft-studios/pastello-contracts@0.0.0 build: tsc -p tsconfig.json spawn ENOENT  WARN  Local package.json exists, but node_modules missing, did you mean to install?



You just don’t have TypeScript installed for that package yet (and no `node_modules`), so `tsc` isn’t on the PATH.

Do one of these:

# Option A (recommended): install TS in the **contracts** package

```bash
# from the repo root
pnpm -F @runecraft-studios/pastello-contracts add -D typescript
pnpm -F @runecraft-studios/pastello-contracts install
pnpm -F @runecraft-studios/pastello-contracts exec tsc -v   # sanity check
pnpm -F @runecraft-studios/pastello-contracts build
```

# Option B: install everything workspace-wide (if you haven’t yet)

```bash
# from the repo root
pnpm install
pnpm -F @runecraft-studios/pastello-contracts build
```

Tips:

* Make sure your workspace file includes the package:

  ```yaml
  # pnpm-workspace.yaml
  packages:
    - "apps/*"
    - "packages/*"   # <-- includes packages/contracts
  ```
* `contracts/package.json` should list TS as a dev dep:

  ```json
  {
    "devDependencies": { "typescript": "^5.6.0" }
  }
  ```
* You can test the compiler directly:

  ```bash
  pnpm -F @runecraft-studios/pastello-contracts exec tsc -p tsconfig.json
  ```

After that, `pnpm -F @runecraft-studios/pastello-contracts build` will work.
