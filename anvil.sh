#!/usr/bin/bash

main() {
    if [ "$#" -lt 1 ]; then
        echo >&2 "USAGE: $0 <region-directory>"
        exit 2
    fi

    declare -r region_directory="$1"

    for file in "$region_directory"/*; do
        if [ ! -f "$file" ]; then
            continue
        fi

        anvil --input "$file"
    done
}

main "$@"
