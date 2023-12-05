#!/bin/bash

# toplevel git repo
ROOT=$(git rev-parse --show-toplevel)

TEMPLATE_DIR="$ROOT/template"

# validate user input
if [[ ! ("$#" == 1 && $1 =~ ^[0-9]+$) ]]; then
    echo "Numeric argument required, '$1' provided" >&2
    exit 1
fi

if ((!($1 >= 1 && $1 <= 25))); then
    echo "Input should be a valid advent-of-code date, was '$1'" >&2
    exit 1
fi

PARAM_DAY=$1
PARAM_DAY_PADDED=$(printf "%02d" $PARAM_DAY)

TARGET_DIR="$ROOT/day-$PARAM_DAY_PADDED"

if [[ -d "$TARGET_DIR" ]]; then
    echo "Directory already exists '$TARGET_DIR'. Exiting" >&2
    exit 1
fi

for template_file in $(find $TEMPLATE_DIR -type f -name '*'); do
    relative_file=$(grealpath --relative-to="$TEMPLATE_DIR" "$template_file")
    target_file=$TARGET_DIR/$relative_file

    # ensure directory exists
    mkdir -p $(dirname $target_file)

    sed -e "s/{{.dayPadded}}/$PARAM_DAY_PADDED/g" \
        -e "s/{{.day}}/$PARAM_DAY/g" \
        $template_file >$target_file
done
