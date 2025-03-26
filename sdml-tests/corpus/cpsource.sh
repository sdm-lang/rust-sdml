#!/usr/bin/env bash

SCRIPT_NAME=$(basename "$0")

no_strip=0
source_dir=

function success {
    echo -e "\033[1;32m✓\033[0m ${1}"
}

function warning {
    echo -e "\033[1;36mWarning:\033[0m ${1}"
}

function error {
    echo -e "\033[1;31mError:\033[0m ${1}"
}

function copy_test_file {
    local source_path="${1}"
    local target_path="${2}"
    local state=0

    touch "${target_path}"

    while IFS= read -r line; do
        case ${state} in
            0)
                if [[ "${line}" =~ ^[=]+ ]] ; then
                    state=1
                else
                    warning "Unexpected line in test file: ${line}"
                fi
                ;;
            1)
                if [[ "${line}" =~ ^[=]+ ]] ; then
                    state=2
                else
                    echo ";;; ${line}" >> "${target_path}"
                fi
                ;;
            2)
                if [[ "${line}" =~ ^[\-]+ ]] ; then
                    break
                else
                    echo "${line}" >> "${target_path}"
                fi
                ;;
        esac
    done < "${source_path}"
}

function copy_tree_sitter_tests {
    if [[ -d "${source_dir}" ]]; then
        for file in ${source_dir}/*.sdm; do
            local_name=$(basename ${file})
            if [[ ! -f "${local_name}" ]] ; then
                success "About to copy new test case from source ${file}"
                copy_test_file "${file}" "${local_name}"
            fi
        done
    else
        error "Value '${source_dir}' is not a directory."
    fi
}

function usage {
    cat << EOF

Usage:

    ${SCRIPT_NAME} [ARG...] [REPO-DIR]

    Copy missing test cases from the tree-sitter-sdml repository into
    this directory. REPO-DIR is the root directory of the local git
    copy. The tree-sitter test meta-data will be stripped from the file
    during copy to leave a clean SDML file.

Arguments:

    --no-strip | -n      Do not strip tree-sitter test meta-data.

EOF
}


while [[ $# -gt 0 ]]; do
    case ${1} in
        --no-strip | -n)
            no_strip=1
            shift 1
            ;;
        -*)
            error "Unrecognized argument ${1}."
            usage
            exit 1
            ;;
        *)
            break
            ;;
    esac
done

source_dir="${1}"

if [ -z "${source_dir}" ]; then
    error "REPO-DIR is required"
    usage
    exit 1
else
    copy_tree_sitter_tests
fi
