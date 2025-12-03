#include "../aoc.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct ParsedInput {
	uint8_t **banks;
	size_t banks_len;
	uint64_t bank_width;
};

void *day03_parse(const char *input_data, const size_t input_len) {
	uint64_t bank_width = 0;
	uint64_t banks_len = 0;
	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '\n') {
			if(bank_width == 0) { bank_width = i; }
			banks_len++;
		}
	}

	uint8_t **banks = malloc(sizeof(uint8_t *) * banks_len);
	for(size_t i = 0; i < banks_len; i++) {
		banks[i] = malloc(sizeof(uint8_t) * bank_width);
	}

	uint64_t bank_i = 0;
	uint64_t bank_j = 0;
	for(size_t i = 0; i < input_len; i++) {
		char b = input_data[i];
		if(b == '\n') {
			bank_i++;
			bank_j = 0;
			continue;
		}

		banks[bank_i][bank_j++] = b - '0';
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->banks = banks;
	parsed_input->banks_len = banks_len;
	parsed_input->bank_width = bank_width;
	return parsed_input;
}

uint64_t day03_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;
	uint64_t sum = 0;
	for(size_t i = 0; i < parsed_input.banks_len; i++) {
		uint8_t *bank = parsed_input.banks[i];

		uint8_t max_num = 0;
		for(size_t j = 0; j < parsed_input.bank_width; j++) {
			uint8_t l = bank[j];
			for(size_t k = j + 1; k < parsed_input.bank_width; k++) {
				uint8_t r = bank[k];
				uint8_t num = l * 10 + r;
				if(num > max_num) { max_num = num; }
			}
		}

		sum += max_num;
	}
	return sum;
}

uint64_t day03_part_2(void *vparsed_input) {
	return 0;
}

void day03_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(size_t i = 0; i < parsed_input->banks_len; i++) {
		free(parsed_input->banks[i]);
	}
	free(parsed_input->banks);
	free(parsed_input);
}

const struct Day day03 = {
	.parse = day03_parse,
	.part_1 = day03_part_1,
	.part_2 = day03_part_2,
	.free = day03_free,
};
