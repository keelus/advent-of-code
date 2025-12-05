#include "../aoc.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

struct Range {
	size_t from, to;
};

struct ParsedInput {
	struct Range *ranges;
	size_t ranges_len;

	uint64_t *numbers;
	size_t numbers_len;
};

bool range_contains_range(const struct Range a, const struct Range b) {
	return b.from >= a.from && b.to <= a.to;
}

bool range_contains_number(const struct Range a, uint64_t v) {
	return v >= a.from && v <= a.to;
}

void do_simplify_ranges(struct Range **current_ranges,
						size_t *current_ranges_len,
						struct Range *pending_ranges,
						ssize_t *pending_ranges_len) {
	if(pending_ranges_len == 0) { return; }

	struct Range new = pending_ranges[--(*pending_ranges_len)];
	for(size_t i = 0; i < *current_ranges_len; i++) {
		if(range_contains_range((*current_ranges)[i], new)) { return; }
	}

	for(size_t i = 0; i < *current_ranges_len; i++) {
		if(range_contains_number((*current_ranges)[i], new.to) &&
		   new.from < (*current_ranges)[i].from) {
			(*current_ranges)[i].from = new.from;
			return;
		}
	}

	*current_ranges = realloc(*current_ranges,
							  sizeof(struct Range) * ++(*current_ranges_len));
	(*current_ranges)[*current_ranges_len - 1] = new;
	return;
}

struct Range *simplify_ranges(struct Range *pending_ranges,
							  ssize_t *pending_ranges_len,
							  size_t *new_ranges_len) {
	struct Range *ranges = NULL;
	size_t ranges_len = 0;

	do {
		do_simplify_ranges(&ranges, &ranges_len, pending_ranges,
						   pending_ranges_len);
	} while(*pending_ranges_len > 0);

	free(pending_ranges);

	*new_ranges_len = ranges_len;
	return ranges;
}

uint64_t read_number(const char *number, const size_t len) {
	char buf[32] = {0};
	memcpy(buf, number, len);
	return strtoul(buf, NULL, 10);
}

void *day05_parse(const char *input_data, const size_t input_len) {
	bool parsing_ranges = true;
	struct Range *ranges = NULL;
	size_t ranges_len = 0;

	uint64_t *numbers = NULL;
	size_t numbers_len = 0;

	struct Range current_range = {};
	ssize_t current_number_idx = -1;
	for(size_t i = 0; i < input_len; i++) {
		char c = input_data[i];
		if(i > 0 && c == '\n' && input_data[i - 1] == '\n') {
			parsing_ranges = false;
			continue;
		}

		if(c == '-' || c == '\n') {
			uint64_t n = read_number(input_data + current_number_idx,
									 i - current_number_idx);
			current_number_idx = -1;

			if(c == '-') {
				current_range.from = n;
			} else {
				if(parsing_ranges) {
					current_range.to = n;
					ranges =
						realloc(ranges, sizeof(struct Range) * ++ranges_len);
					ranges[ranges_len - 1] = current_range;
				} else {
					current_number_idx = -1;
					numbers =
						realloc(numbers, sizeof(uint64_t) * ++numbers_len);
					numbers[numbers_len - 1] = n;
				}
			}
		} else if(current_number_idx < 0) {
			current_number_idx = i;
		}
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	*parsed_input = (struct ParsedInput){
		.numbers = numbers,
		.numbers_len = numbers_len,
		.ranges = ranges,
		.ranges_len = ranges_len,
	};

	size_t prev_ranges_len;
	do {
		prev_ranges_len = parsed_input->ranges_len;
		parsed_input->ranges = simplify_ranges(
			parsed_input->ranges, (ssize_t *)&parsed_input->ranges_len,
			&parsed_input->ranges_len);
	} while(prev_ranges_len != parsed_input->ranges_len);

	return parsed_input;
}

uint64_t day05_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t count = 0;
	for(size_t i = 0; i < parsed_input.numbers_len; i++) {
		for(size_t j = 0; j < parsed_input.ranges_len; j++) {
			if(range_contains_number(parsed_input.ranges[j],
									 parsed_input.numbers[i])) {
				count++;
				break;
			}
		}
	}

	return count;
}

uint64_t day05_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t count = 0;
	for(size_t i = 0; i < parsed_input.ranges_len; i++) {
		count += parsed_input.ranges[i].to - parsed_input.ranges[i].from + 1;
	}

	return count;
}

void day05_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	free(parsed_input->numbers);
	free(parsed_input->ranges);
	free(parsed_input);
}

const struct Day day05 = {
	.parse = day05_parse,
	.part_1 = day05_part_1,
	.part_2 = day05_part_2,
	.free = day05_free,
};
