#include "../aoc.h"
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

struct ParsedInput {
	uint64_t *raw_ranges;
	size_t raw_ranges_len;
};

void *day02_parse(const char *input_data, const size_t input_len) {
	uint64_t *raw_ranges = NULL;
	size_t raw_ranges_len = 0;

	const char *cur_num = NULL;
	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '-' || input_data[i] == ',' ||
		   input_data[i] == '\n') {
			char buf[32] = {0};
			strncpy(buf, cur_num,
					MIN(sizeof(buf), (size_t)((input_data + i) - cur_num)));

			uint64_t num = strtoul(buf, NULL, 10);
			raw_ranges =
				realloc(raw_ranges, sizeof(uint64_t) * ++raw_ranges_len);
			raw_ranges[raw_ranges_len - 1] = num;
			cur_num = NULL;
			continue;
		} else if(cur_num) {
			continue;
		}

		cur_num = input_data + i;
	}
	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->raw_ranges = raw_ranges;
	parsed_input->raw_ranges_len = raw_ranges_len;
	return parsed_input;
}

uint64_t day02_part_1(const void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t sum = 0;
	for(size_t i = 0; i < parsed_input.raw_ranges_len - 1; i += 2) {
		uint64_t from = parsed_input.raw_ranges[i];
		uint64_t to = parsed_input.raw_ranges[i + 1];

		for(uint64_t num = from; num <= to; num++) {
			if(num <= 9) { continue; }
			uint64_t digits = floor(log10(num)) + 1;
			if(digits % 2 != 0) { continue; }

			uint64_t half = pow(10.0f, digits / 2.0f);
			uint64_t rem = num % half;
			uint64_t div = num / half;
			if(rem == div) { sum += num; }
		}
	}

	return sum;
}

uint64_t day02_part_2(const void *vparsed_input) {
	return 0;
}

void day02_free(const void *vparsed_input) {
}

const struct Day day02 = {
	.parse = day02_parse,
	.part_1 = day02_part_1,
	.part_2 = day02_part_2,
	.free = day02_free,
};
