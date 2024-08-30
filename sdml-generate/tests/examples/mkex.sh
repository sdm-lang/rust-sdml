#!/usr/bin/env bash

function success {
    echo -e "\033[1;32m✓\033[0m ${1}"
}

function warning {
    echo -e "\033[1;36mWarning:\033[0m ${1}"
}

function error {
    echo -e "\033[1;31mError:\033[0m ${1}"
}

if [[ -z "${1}" ]]; then
    error "expecting new extension as argument"
    exit 1
fi

old_ext=sdm
new_ext=${1}

for old_file in *.${old_ext}; do
    new_file="${old_file%${old_ext}}${new_ext}"
    if [[ -f "${new_file}" ]]; then
        warning "skipping existing file ${new_file}"
    else
        if touch "${new_file}"; then
            success "Create new test file ${new_file}"
        else
            error "Could not create file ${new_file}; errno: $?"
        fi
    fi
done

