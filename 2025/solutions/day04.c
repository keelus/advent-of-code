#include "../aoc.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct ParsedInput {
	size_t width, height;
	char **data;
};

void *day04_parse(const char *input_data, const size_t input_len) {
	size_t width = 0, height = 0;

	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '\n') {
			height++;
		} else if(height == 0) {
			width++;
		}
	}

	char **data = malloc(sizeof(char *) * height);
	size_t data_i = 0;
	const char *cur_line = NULL;
	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '\n') {
			size_t line_len = i - (cur_line - input_data);

			char *line = malloc(sizeof(char) * (line_len + 1));
			memcpy(line, cur_line, line_len);
			line[line_len] = '\0';
			data[data_i++] = line;

			cur_line = NULL;
			continue;
		} else if(!cur_line) {
			cur_line = input_data + i;
		}
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->width = width;
	parsed_input->height = height;
	parsed_input->data = data;
	return parsed_input;
}

uint8_t get_adjacent_rolls(struct ParsedInput input, size_t target_i,
						   size_t target_j) {
	// clang-format off
	ssize_t deltas[8][2] = {
		{-1, -1}, {-1, 0}, {-1, 1},
		{ 0, -1},          { 0, 1},
		{ 1, -1}, { 1, 0}, { 1, 1}
	};
	// clang-format on

	uint8_t count = 0;
	for(size_t i = 0; i < 8; i++) {
		size_t new_i = target_i + deltas[i][0];
		size_t new_j = target_j + deltas[i][1];

		if(new_i < input.height && new_j < input.width) {
			if(input.data[new_i][new_j] == '@') { count++; }
		}
	}

	return count;
}

uint64_t day04_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t count = 0;
	for(size_t i = 0; i < parsed_input.height; i++) {
		for(size_t j = 0; j < parsed_input.width; j++) {
			if(parsed_input.data[i][j] == '@') {
				if(get_adjacent_rolls(parsed_input, i, j) < 4) { count++; }
			}
		}
	}
	return count;
}

uint64_t day04_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t count, total_count = 0;
	do {
		count = 0;
		for(size_t i = 0; i < parsed_input.height; i++) {
			for(size_t j = 0; j < parsed_input.width; j++) {
				if(parsed_input.data[i][j] == '@') {
					if(get_adjacent_rolls(parsed_input, i, j) < 4) {
						parsed_input.data[i][j] = '.';
						count++;
					}
				}
			}
		}

		total_count += count;
	} while(count > 0);
	return total_count;
}

void day04_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(size_t i = 0; i < parsed_input->height; i++) {
		free(parsed_input->data[i]);
	}
	free(parsed_input->data);
	free(parsed_input);
}

const struct Day day04 = {
	.parse = day04_parse,
	.part_1 = day04_part_1,
	.part_2 = day04_part_2,
	.free = day04_free,
};
