#include "../aoc.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <threads.h>

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

int8_t get_max_index(uint8_t *numbers, uint8_t from, uint8_t to) {
	uint8_t max = 0;
	uint8_t max_i = 0;
	for(size_t i = from; i <= to; i++) {
		if(i == from || numbers[i] > max) {
			max = numbers[i];
			max_i = i;
		}
	}

	return max_i;
}

uint64_t calculate_joltage(uint8_t *bank, uint64_t bank_width,
						   uint8_t batteries) {
	uint64_t number = 0;
	uint8_t digits = 0;

	uint8_t i = 0;
	while(digits != batteries) {
		uint8_t needed = batteries - digits;

		int8_t max_index =
			get_max_index((uint8_t *)bank, i, bank_width - needed);
		number = number * 10 + bank[max_index];
		digits++;
		i = max_index + 1;
	}

	return number;
}

uint64_t day03_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t sum = 0;
	for(size_t i = 0; i < parsed_input.banks_len; i++) {
		sum += calculate_joltage(parsed_input.banks[i], parsed_input.bank_width,
								 2);
	}

	return sum;
}

uint64_t day03_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t sum = 0;
	for(size_t i = 0; i < parsed_input.banks_len; i++) {
		sum += calculate_joltage(parsed_input.banks[i], parsed_input.bank_width,
								 12);
	}

	return sum;
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
