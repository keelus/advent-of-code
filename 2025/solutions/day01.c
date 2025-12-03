#include "../aoc.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

uint64_t mod(int64_t v, int64_t d) {
	return (v % d + d) % d;
}

struct ParsedInput {
	int64_t *rotations;
	size_t rotations_len;
};

void *day01_parse(const char *input_data, size_t input_len) {
	int64_t *rotations = NULL;
	size_t rotations_len = 0;

	const char *current_line = NULL;
	for(size_t i = 0; i < input_len; i++) {
		if(!current_line && input_data[i] != '\n') {
			current_line = input_data + i;
			continue;
		}

		if(input_data[i] == '\n' && current_line) {
			char trimmed_value[32];
			size_t line_len = input_data + i - (current_line + 1);
			memcpy(trimmed_value, current_line + 1, line_len);
			trimmed_value[line_len] = '\0';

			rotations = realloc(rotations, sizeof(int64_t) * (++rotations_len));
			int64_t rotation = strtol(trimmed_value, NULL, 10);
			if(current_line[0] == 'L') { rotation *= -1; }
			rotations[rotations_len - 1] = rotation;

			current_line = NULL;
		}
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->rotations = rotations;
	parsed_input->rotations_len = rotations_len;
	return parsed_input;
}

uint64_t day01_part_1(void *vparsed_input) {
	const struct ParsedInput parsed_input =
		*(struct ParsedInput *)vparsed_input;

	uint64_t dial_zeros = 0;
	int64_t dial = 50;
	for(size_t i = 0; i < parsed_input.rotations_len; i++) {
		dial = mod(dial + parsed_input.rotations[i], 100);
		if(dial == 0) { dial_zeros++; }
	}

	return dial_zeros;
}

uint64_t day01_part_2(void *vparsed_input) {
	const struct ParsedInput parsed_input =
		*(struct ParsedInput *)vparsed_input;

	uint64_t dial_zeros = 0;
	int64_t dial = 50;
	for(size_t i = 0; i < parsed_input.rotations_len; i++) {
		int64_t value = parsed_input.rotations[i];

		uint64_t hundreds = floor(abs(value) / 100.0f);
		value += (value < 0 ? 100 : -100) * hundreds;

		dial_zeros += hundreds;

		int64_t new_dial = dial + value;
		if(value != 0 && dial != 0) {
			if(new_dial <= 0 || new_dial > 99) { dial_zeros++; }
		}

		dial = mod(new_dial, 100);
	}

	return dial_zeros;
}

void day01_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	free(parsed_input->rotations);
	free(parsed_input);
}

const struct Day day01 = {
	.parse = day01_parse,
	.part_1 = day01_part_1,
	.part_2 = day01_part_2,
	.free = day01_free,
};
