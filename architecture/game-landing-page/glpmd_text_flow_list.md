A1. Page Load
    A2. Determine Role
        A2.1. Check for session token in browser
            A2.1.1. If not found: set role = Visitor, go to [B1]
            A2.1.2. If found: validate token via REST API
                A2.1.2.1. If invalid: set role = Visitor, go to [B1]
                A2.1.2.2. If valid: fetch user profile
                    A2.1.2.2.1. If admin: set role = Admin, go to [B2]
                    A2.1.2.2.2. If member: set role = Member, go to [B2]
B1. Show Login/Signup [END]
B2. Proceed with Session Lookup