source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh

test_ref_file_path() {
    echo "/tmp/$$"
}

create_test_ref_file() {
    echo "" > $(test_ref_file_path)
}

imag-ref() {
    imag-call-binary "$(dirname ${BASH_SOURCE[0]})/../target/debug/" imag-ref $*
}

