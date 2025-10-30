.PHONY: generate check build
generate: ; buf generate
check: ; buf lint && buf breaking
build: ; pnpm build