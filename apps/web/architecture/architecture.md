public/
  favicon.ico
  robots.txt
  sitemap.xml
  site.webmanifest           # if PWA
  icons/
    favicon-16.png
    favicon-32.png
    apple-touch-icon-180.png
    safari-pinned-tab.svg
  og-image.png               # social share image
  .well-known/…              # domain verification, etc.
src/
  app/
    main.ts               # createApp, plugins, global styles
    router.ts             # routes & lazy imports
    store.ts              # Pinia setup (if you use it)
    styles/
      tokens.css          # design tokens (colors, spacing)
      globals.css
  features/
    tile-mover/
      components/
        TileBoard.vue
        TileTile.vue
      pages/
        TileMoverPage.vue  # route entry for the game
      logic/
        board.ts           # pure domain logic (shuffle, win check)
        verifyLocal.ts     # local verifier seam
      api/
        moveVerifierClient.ts  # future RPC/proto client wrapper
    puzzle-x/              # next game follows same shape
      ...
  shared/
    components/            # buttons, modals, layout shells
    composables/           # useHotkeys, useAudio, useLocalStorage
    lib/                   # helpers (rng, clamp), date, math
    types/                 # global TS types
  gen/                     # <- ts-proto output (isolated)
    protobuf/...
  pages/                   # optional: top-level pages (Home, About)
  App.vue
  env.d.ts
