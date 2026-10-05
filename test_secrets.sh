#!/usr/bin/env bash
# Sourced by integration_test.sh, performance_test.sh and security_test.sh (MAIR-428).
#
# The test stacks run the published image, which refuses a missing, short or well-known
# JWT_SECRET (mairie360_api_lib 2.0.0). Nothing is committed: every run gets a random secret
# (unless JWT_SECRET is already exported) and the tokens the runners need are signed from it at
# run time, so no committed token is valid anywhere.

export JWT_SECRET="${JWT_SECRET:-$(openssl rand -hex 32)}"

# Base64url without padding, from stdin.
b64url() {
    openssl base64 -A | tr '+/' '-_' | tr -d '='
}

# sign_jwt <sub> <role> <ttl-seconds>: HS256 JWT with the claims mairie360_api_lib reads
# ({ sub, role, exp }, no `sid`), signed with JWT_SECRET.
sign_jwt() {
    local header payload signature
    header=$(printf '{"alg":"HS256","typ":"JWT"}' | b64url)
    payload=$(printf '{"sub":"%s","role":"%s","exp":%d}' "$1" "$2" "$(($(date +%s) + $3))" | b64url)
    signature=$(printf '%s.%s' "$header" "$payload" | openssl dgst -sha256 -hmac "$JWT_SECRET" -binary | b64url)
    printf '%s.%s.%s' "$header" "$payload" "$signature"
}
