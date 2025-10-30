[A1] Page Load
    │
    ├─[A2] Determine Role
          │
          └─[A2.1] Check for session token in browser
                 │
                 ├─[A2.1.1] If not found: set role = Visitor ──► [B1] Show Login/Signup [END]
                 │
                 └─[A2.1.2] If found: validate token via REST API
                       │
                       ├─[A2.1.2.1] If invalid: set role = Visitor ──► [B1] Show Login/Signup [END]
                       │
                       └─[A2.1.2.2] If valid: fetch user profile
                             │
                             ├─[A2.1.2.2.1] If admin: set role = Admin ──► [B2] Proceed with Session Lookup
                             │
                             └─[A2.1.2.2.2] If member: set role = Member ──► [B2] Proceed with Session Lookup

[B2] Proceed with Session Lookup
    │
    ├─[B2.1] Check for existing game session token in cookies/local storage
           │
           ├─[B2.1.1] If found: validate session via REST API
           │        │
           │        ├─[B2.1.1.1] If session valid: show “Rejoin Game” button ──► [END]
           │        │
           │        └─[B2.1.1.2] If session invalid: clear token ──► [B2.1.2]
           │
           └─[B2.1.2] If not found: show “Join Game Session” button
                    │
                    ├─[B2.1.2.1] On button click, show join/create form
                    │
                    ├─[B2.1.2.2] User submits join/create form (REST call)
                    │
                    ├─[B2.1.2.3] Save session info in cookies/local storage
                    │
                    └─[B2.1.2.4] Redirect to game page

[C1] On Game Page Load
    │
    └─[C1.1] Establish WebSocket connection using session info