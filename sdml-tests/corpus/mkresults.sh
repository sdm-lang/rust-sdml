#!/usr/bin/env bash

SCRIPT_NAME=$(basename "$0")

make_all=0
force_empty=0
format=
verbose=0

function verbose_success {
    if [[ $verbose -eq 1 ]]; then
        success "\033[2m$1\033[0m"
    fi
}

function success {
    echo -e "\033[1;32m✓\033[0m ${1}"
}

function warning {
    echo -e "\033[1;36mWarning:\033[0m ${1}"
}

function error {
    echo -e "\033[1;31mError:\033[0m ${1}"
}

function make_result {
    file=$1
    file_no_path=${file##*/}
    file_no_suffix=${file_no_path%%.*}
    new_file_path="./${format}/${file_no_suffix}.${format}"

    if [[ -f "${new_file_path}" ]]; then
        if [[ ${force_empty} -eq 1 ]]; then
            success "file ${new_file_path} exists, clearing content"
            echo "" > "${new_file_path}"
        else
            verbose_success "file ${new_file_path} exists, skipping"
        fi
    else
        touch "${new_file_path}"
        success "file ${new_file_path} created"
    fi
}

function make_results {
    if [[ -d "${format}" ]]; then
        verbose_success "directory ${format} exists"
    else
        mkdir "${format}"
        success "directory ${format} created"
    fi

    if [[ ${make_all} -eq 1 ]]; then
        for file in ./*.sdm; do
            make_result "${file}"
        done
    else
        for file in $@; do
            make_result "${file}"
        done
    fi
}

function usage {
    cat << EOF

Usage:

    ${SCRIPT_NAME} [ARG...] --result-format FORMAT [FILE...]

    Create missing result files in the result-format directory for
    test cases in the corpus directory.

Arguments:

    --verbose -v          Output more messages.
    --force | -f          Overwrite existing result files.
    --all   | -a          Create files for all test cases.
    --result-format | -r  Result format/directory name.

Notes:

    1. Any listed FILEs correspond to individual test cases.
    2. Listed FILEs are ignored if --all is specified.
EOF
}

while [[ $# -gt 0 ]]; do
    case ${1} in
        --verbose | -v)
            verbose=1
            shift 1
            ;;
        --force | -f)
            force_empty=1
            shift 1
            ;;
        --all | -a)
            make_all=1
            shift 1
            ;;
        --result-format | -r)
            format="${2}"
            shift 2
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

if [ -z "${format}" ]; then
    error "result-format is required"
    usage
    exit 1
else
    make_results $@
fi
