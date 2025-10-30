pastello/
  apps/
    api/                     # Go service ("backend" → "api")
      cmd/server/...
      internal/...
      migrations/
      pkg/
      scripts/
      test/
      go.mod
      go.sum
      Makefile
      .gitignore
    web/                     # Vite SPA (UI)
      public/
      src/
      e2e/
      .editorconfig
      .gitattributes
      .gitignore
      env.d.ts
      eslint.config.ts
      index.html
      package.json
      pnpm-lock.yaml
      playwright.config.ts
      README.md
      tsconfig.*.json
      vite.config.ts
      vitest.config.ts

  packages/
    contracts/               # proto → generated Go/TS, built ESM committed
      proto/
        runecraftstudios/...
      gen/
        go/
          go.mod
        ts/
          runecraftstudios/...
      src/
        index.ts             # curated exports (tiny)
      dist/                  # built JS + d.ts (commit this)
      buf.yaml
      buf.gen.yaml
      package.json
      tsconfig.json
      Makefile

    ui/                      # (optional) shared TS UI kit if you want it later
      package.json
      src/...

  go.work                    # local dev links api <-> contracts
  pnpm-workspace.yaml        # workspaces for web + TS packages
  package.json               # root scripts
  .gitignore                 # repo-wide
  Makefile                   # convenience targets (generate, build, dev)
