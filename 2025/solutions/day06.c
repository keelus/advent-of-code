#include "../aoc.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

uint64_t parse_uint64(const char *number, const size_t len) {
	char buf[32] = {0};
	memcpy(buf, number, len);
	return strtoul(buf, NULL, 10);
}

bool is_col_empty(char **lines, const size_t num_lines, const size_t col) {
	for(size_t i = 0; i < num_lines; i++) {
		if(lines[i][col] != ' ') { return false; }
	}
	return true;
}

bool consume_number_horizontal(char *line, size_t *idx, uint64_t *number) {
	ssize_t begin_idx = -1;
	for(; line[*idx]; (*idx)++) {
		char c = line[*idx];
		if(c == ' ' || c == '_') {
			if(begin_idx >= 0) { break; }
			continue;
		}
		if(begin_idx < 0) { begin_idx = *idx; }
	}

	if(begin_idx < 0) { return false; }

	*number = parse_uint64(line + begin_idx, *idx - begin_idx);
	return true;
}

uint64_t consume_number_vertical(char **lines, const size_t num_lines,
								 const size_t col) {
	uint64_t number = 0;

	for(size_t i = 0; i < num_lines; i++) {
		char c = lines[i][col];
		if(c == '_') { continue; }
		number *= 10;
		number += (c - '0');
	}

	return number;
}

struct ParsedInput {
	char **lines;
	size_t num_lines, line_width;
};

void *day06_parse(const char *input_data, const size_t input_len) {
	size_t lines_len = 0;
	size_t line_width = 0;
	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '\n') {
			lines_len++;
		} else if(lines_len == 0) {
			line_width++;
		}
	}

	char **lines = malloc(sizeof(char *) * lines_len);
	for(size_t i = 0; i < lines_len; i++) {
		lines[i] = malloc(sizeof(char) * (line_width + 1));
		lines[i][line_width] = '\0';
	}

	size_t lines_i = 0;
	ssize_t cur_line_i = -1;
	for(size_t i = 0; i < input_len; i++) {
		if(input_data[i] == '\n') {
			memcpy(lines[lines_i++], input_data + cur_line_i, line_width);
			cur_line_i = -1;
		} else if(cur_line_i < 0) {
			cur_line_i = i;
		}
	}

	ssize_t cur_num_j = -1;
	for(size_t i = 0; i < lines_len - 1; i++) {
		for(size_t j = 0; j < line_width; j++) {
			char c = lines[i][j];
			if(c == ' ') {
				if(!is_col_empty(lines, lines_len - 1, j)) {
					lines[i][j] = '_';
				} else {
					cur_num_j = -1;
				}
				continue;
			}
			if(cur_num_j < 0 && c >= '0' && c <= '9') { cur_num_j = j; }
		}
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	*parsed_input = (struct ParsedInput){
		.lines = lines, .num_lines = lines_len, .line_width = line_width};
	return parsed_input;
}

enum Operation { ADD, MULTIPLY };

enum Operation *parse_operations(char *line, size_t *num_operations) {
	enum Operation *operations = NULL;
	*num_operations = 0;

	char c;
	while((c = *line++)) {
		if(c == '+' || c == '*') {
			operations = realloc(operations,
								 sizeof(enum Operation) * ++(*num_operations));
			operations[(*num_operations) - 1] = c == '+' ? ADD : MULTIPLY;
		}
	}

	return operations;
}

uint64_t day06_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	ssize_t cur_num_j = -1;

	size_t num_cols = 0;
	size_t num_rows = parsed_input.num_lines - 1;
	for(size_t j = 0; j < parsed_input.line_width; j++) {
		char c = parsed_input.lines[0][j];
		if(c == '_') { continue; }
		if(c == ' ' && cur_num_j >= 0) {
			if(cur_num_j >= 0) {
				num_cols++;
				cur_num_j = -1;
			}
		} else if(cur_num_j < 0 && c >= '0' && c <= '9') {
			cur_num_j = j;
		}
	}
	if(cur_num_j >= 0) { num_cols++; }

	uint64_t **cols = malloc(sizeof(uint64_t *) * num_cols);
	for(size_t i = 0; i < num_cols; i++) {
		cols[i] = malloc(sizeof(uint64_t) * num_rows);
	}

	size_t num_operations = 0;
	enum Operation *operations = parse_operations(
		parsed_input.lines[parsed_input.num_lines - 1], &num_operations);

	uint64_t *results = malloc(sizeof(uint64_t) * num_cols);
	for(size_t i = 0; i < num_cols; i++) {
		enum Operation op = operations[i];
		if(op == ADD) {
			results[i] = 0;
		} else {
			results[i] = 1;
		}
	}

	for(size_t i = 0; i < parsed_input.num_lines - 1; i++) {
		size_t cur_col = 0, j = 0;
		uint64_t n;
		bool parsed;
		enum Operation op;
		do {
			parsed = consume_number_horizontal(parsed_input.lines[i], &j, &n);
			if(parsed) {
				op = operations[cur_col];
				if(op == ADD) {
					results[cur_col++] += n;
				} else {
					results[cur_col++] *= n;
				}
			}
		} while(parsed);
	}

	uint64_t sum = 0;
	for(size_t i = 0; i < num_cols; i++) {
		sum += results[i];
	}

	for(size_t i = 0; i < num_cols; i++) {
		free(cols[i]);
	}
	free(cols);
	free(operations);
	free(results);

	return sum;
}

uint64_t day06_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	size_t num_cols = 1;
	char *l = parsed_input.lines[0];
	while((l = strchr(l, ' '))) {
		num_cols++;
		l++;
	}

	enum Operation *operations = malloc(sizeof(enum Operation) * num_cols);
	size_t col = 0;
	l = parsed_input.lines[parsed_input.num_lines - 1];
	while((l = strpbrk(l, "*+"))) {
		operations[col++] = *l == '+' ? ADD : MULTIPLY;
		l++;
	}

	uint64_t *results = malloc(sizeof(uint64_t) * num_cols);
	for(size_t i = 0; i < num_cols; i++) {
		results[i] = operations[i] == MULTIPLY ? 1 : 0;
	}

	col = 0;
	for(size_t j = 0; j < parsed_input.line_width; j++) {
		enum Operation op = operations[col];
		if(is_col_empty(parsed_input.lines, parsed_input.num_lines - 1, j)) {
			col++;
			continue;
		}

		uint64_t n = consume_number_vertical(parsed_input.lines,
											 parsed_input.num_lines - 1, j);
		if(op == ADD) {
			results[col] += n;
		} else {
			results[col] *= n;
		}
	}

	uint64_t sum = 0;
	for(size_t i = 0; i < num_cols; i++) {
		sum += results[i];
	}

	free(operations);
	free(results);

	return sum;
}

void day06_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(size_t i = 0; i < parsed_input->num_lines; i++) {
		free(parsed_input->lines[i]);
	}
	free(parsed_input->lines);
	free(parsed_input);
}

const struct Day day06 = {
	.parse = day06_parse,
	.part_1 = day06_part_1,
	.part_2 = day06_part_2,
	.free = day06_free,
};
