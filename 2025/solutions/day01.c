#include "../aoc.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

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

uint64_t day01_part_1(const void *vparsed_input) {
	const struct ParsedInput parsed_input =
		*(const struct ParsedInput *)vparsed_input;

	uint64_t dial_zeros = 0;
	int64_t dial = 50;
	for(size_t i = 0; i < parsed_input.rotations_len; i++) {
		dial = (dial + parsed_input.rotations[i]) % 100;
		if(dial == 0) { dial_zeros++; }
	}

	return dial_zeros;
}

uint64_t day01_part_2(const void *vparsed_input) {
	const struct ParsedInput parsed_input =
		*(const struct ParsedInput *)vparsed_input;

	uint64_t dial_zeros = 0;
	int64_t dial = 50;
	for(size_t i = 0; i < parsed_input.rotations_len; i++) {
		int64_t value = parsed_input.rotations[i];
		int8_t delta = value < 0 ? -1 : 1;

		while(value) {
			dial = (dial + delta) % 100;
			value -= delta;
			if(dial == 0) { dial_zeros++; }
		}
	}

	return dial_zeros;
}

void day01_free(const void *vparsed_input) {
	const struct ParsedInput parsed_input =
		*(const struct ParsedInput *)vparsed_input;
	free(parsed_input.rotations);
	free((void *)vparsed_input);
}

const struct Day day01 = {
	.parse = day01_parse,
	.part_1 = day01_part_1,
	.part_2 = day01_part_2,
	.free = day01_free,
};
