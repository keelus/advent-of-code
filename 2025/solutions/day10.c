#include "../aoc.h"
#include "../utils.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Button {
	uint8_t toggles[16];
	size_t num_toggles;
};

struct Machine {
	char light[16];

	struct Button buttons[32];
	size_t num_buttons;
};

struct ParsedInput {
	struct Machine *machines;
	size_t num_machines;
};

void *day10_parse(const char *input_data, const size_t input_len) {
	size_t num_lines;
	char **lines = split_lines(input_data, input_len, &num_lines);

	struct Machine *machines = NULL;
	size_t num_machines = 0;

	for(size_t i = 0; i < num_lines; i++) {
		const char *line = lines[i];
		size_t line_len = strlen(line);

		bool parsing_light = false;
		bool parsing_buttons = false;
		bool parsing_button = false;
		bool parsing_joltage = false;

		struct Machine machine;
		memset(machine.light, 0, sizeof(machine.light));
		memset(machine.buttons, 0, sizeof(machine.buttons));
		machine.num_buttons = 0;

		size_t light_i = 0;

		for(size_t j = 0; j < line_len; j++) {
			const char c = line[j];

			if(parsing_joltage) { break; }

			if(c == ' ') { continue; }

			switch(c) {
			case '[': parsing_light = true; continue;
			case ']':
				parsing_light = false;
				parsing_buttons = true;
				continue;
			case '{': parsing_joltage = true; continue;
			case '(':
				assert(parsing_buttons &&
					   "Got open parenthesis while not parsing buttons.");
				parsing_button = true;
				continue;
			case ' ': continue;
			default: break;
			}

			if(parsing_light) {
				machine.light[light_i++] = c;
				continue;
			} else if(parsing_button) {
				uint8_t num = 0;
				while(1) {
					char c = line[j++];

					if(c == ')' || c == ',') {
						machine.buttons[machine.num_buttons]
							.toggles[machine.buttons[machine.num_buttons]
										 .num_toggles++] = num;
						num = 0;

						if(c == ')') {
							machine.num_buttons++;
							parsing_button = false;
							break;
						}
					}

					if(c >= '0' && c <= '9') { num = num * 10 + c - '0'; }
				}
			}
		}

		machines = realloc(machines, sizeof(struct Machine) * ++num_machines);
		machines[num_machines - 1] = machine;
	}

	for(size_t i = 0; i < num_lines; i++) {
		free(lines[i]);
	}
	free(lines);

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->machines = machines;
	parsed_input->num_machines = num_machines;
	return parsed_input;
}

struct Comb {
	uint8_t *values;
	size_t len;
};

struct Comb *gen_combinations(const size_t item_i, const size_t num_items,
							  size_t *num_combinations) {
	if(num_items == 0) {
		*num_combinations = 0;
		return NULL;
	}

	size_t num_without;
	struct Comb *without =
		gen_combinations(item_i + 1, num_items - 1, &num_without);

	size_t num_with = num_without + 1;
	struct Comb *with = malloc(sizeof(struct Comb) * num_with);

	with[0].len = 1;
	with[0].values = malloc(sizeof(char));
	with[0].values[0] = item_i;

	for(size_t i = 0; i < num_without; i++) {
		size_t new_len = without[i].len + 1;
		with[i + 1].len = new_len;
		with[i + 1].values = malloc(sizeof(char) * new_len);
		memcpy(with[i + 1].values + 1, without[i].values,
			   sizeof(char) * new_len - 1);
		with[i + 1].values[0] = item_i;
	}

	struct Comb *final_combs =
		malloc(sizeof(struct Comb) * (num_with + num_without));
	if(num_without > 0) {
		memcpy(final_combs, without, sizeof(struct Comb) * num_without);
	}
	memcpy(final_combs + num_without, with, sizeof(struct Comb) * num_with);
	*num_combinations = num_with + num_without;

	free(without);
	free(with);

	return final_combs;
}

int cmp(const void *a, const void *b) {
	struct Comb *aa = (struct Comb *)a;
	struct Comb *bb = (struct Comb *)b;

	if(aa->len == bb->len) { return 0; }

	return aa->len > bb->len ? 1 : -1;
}

char *try_combination(struct Button *buttons, uint8_t *indexes,
					  size_t num_indexes, size_t light_len) {
	static char buf[128];
	memset(buf, 0, sizeof(buf));
	for(size_t i = 0; i < light_len; i++) {
		buf[i] = '.';
	}

	for(size_t i = 0; i < num_indexes; i++) {
		size_t idx = indexes[i];
		struct Button b = buttons[idx];

		for(size_t t = 0; t < b.num_toggles; t++) {
			size_t tidx = b.toggles[t];
			char c = buf[tidx] == '.' ? '#' : '.';
			buf[tidx] = c;
		}
	}
	return buf;
}

uint64_t day10_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	uint64_t sum = 0;
	for(size_t i = 0; i < parsed_input.num_machines; i++) {
		struct Machine *m = parsed_input.machines + i;
		size_t num_combs;
		struct Comb *combs = gen_combinations(0, m->num_buttons, &num_combs);
		qsort(combs, num_combs, sizeof(struct Comb), cmp);
		for(size_t j = 0; j < num_combs; j++) {
			char *light_res = try_combination(m->buttons, combs[j].values,
											  combs[j].len, strlen(m->light));
			if(strcmp(light_res, m->light) == 0) {
				sum += combs[j].len;
				break;
			}
		}

		for(size_t j = 0; j < num_combs; j++) {
			free(combs[j].values);
		}
		free(combs);
	}

	return sum;
}

uint64_t day10_part_2(void *vparsed_input) {
	return 0;
}

void day10_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	free(parsed_input->machines);
	free(parsed_input);
}

const struct Day day10 = {
	.parse = day10_parse,
	.part_1 = day10_part_1,
	.part_2 = day10_part_2,
	.free = day10_free,
};
