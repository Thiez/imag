#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh
source $(dirname ${BASH_SOURCE[0]})/utils.sh

empty_ref() {
    cat <<EOS
---
[imag]
links = []
version = "0.2.0"

[ref]
content_hash = "adc83b19e793491b1c6ea0fd8b46cd9f32e592fc"
path = "/tmp/foo"

[ref.permissions]
---
EOS
}

test_call() {
    create_test_ref_file

    expected_content="$(empty_ref)"

    imag-ref add -C --path $(test_ref_file_path)

    out=$(cat $(test_ref_file_path))
    if [[ "$out" != "$(empty_ref)" ]]; then
        err "Expected ref for empty file in store. Didn't work"
        return 1
    fi
}


invoke_tests                                                \
    test_call                                               \
