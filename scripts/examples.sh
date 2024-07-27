#!/usr/bin/env bash

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
source "$SCRIPT_DIR/utils.sh"

provider="$1"
# If provider is not provided, or is not one of the supported providers, exit with an error
if [[ "$provider" != "ollama" && "$provider" != "openai" && "$provider" != "anthropic" ]]; then
	echo "ERROR: Invalid provider. Supported providers: ollama, openai, anthropic"
	exit 1
fi

PASSED=1

info "Running all examples in the 'orch' crate for provider $provider..."
pushd core 2>&1 >/dev/null
for example in $(find examples -name '*.rs'); do
	example=${example%.rs}
	info "Running example: $(basename $example)"
	cargo run --quiet --example $(basename $example) -- $provider 1>/dev/null
	if [ $? -ne 0 ]; then
		PASSED=0
		error "Example $(basename $example) failed"
	fi
	success "Example $(basename $example) succeeded"
done
popd 2>&1 >/dev/null
info "Ran all examples in the 'orch' crate for provider $provider"

exit $PASSED
