#!/bin/sh
set -e

DAY=$1

if [ -z "$DAY" ]; then
	echo "[ERROR] Day not set.";
	exit
fi

if ! [[ "$DAY" =~ ^([1-9]|1[0-2])$ ]]; then
    echo "[ERROR] \"$DAY\" is not a valid day (1–12)."
    exit
fi

DAY=$(printf "%02d" "$DAY")
DAY_NAME="day$DAY"
IMPL_PATH="solutions/$DAY_NAME.c"
INPT_PATH="inputs/$DAY_NAME"

if  [ -f "$IMPL_PATH" ] || [ -d "$INPT_PATH" ]; then
	echo "[ERROR] Day $DAY already exists.";
	exit
fi

cat <<EOF > "$IMPL_PATH"
#include "../aoc.h"
#include <stdlib.h>

void *day${DAY}_parse(const char *input_data) {
	return NULL;
}

uint64_t day${DAY}_part_1(const void *vparsed_input) {
	return 0;
}

uint64_t day${DAY}_part_2(const void *vparsed_input) {
	return 0;
}

void day${DAY}_free(const void *vparsed_input) {}

const struct Day day$DAY = {
	.parse = day${DAY}_parse,
	.part_1 = day${DAY}_part_1,
	.part_2 = day${DAY}_part_2,
	.free = day${DAY}_free,
};
EOF

mkdir "$INPT_PATH"
touch "$INPT_PATH/input.txt"
touch "$INPT_PATH/sample.txt"

echo "Day $DAY created successfully"
