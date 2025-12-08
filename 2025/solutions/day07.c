#include "../external/khash.h"
#include "../aoc.h"
#include "../utils.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

KHASH_MAP_INIT_INT64(SplitMap, size_t)

struct ParsedInput {
	char **lines;
	size_t num_lines;
	size_t start_j;
};

void *day07_parse(const char *input_data, const size_t input_len) {
	size_t num_lines = 0;
	char **lines = split_lines(input_data, input_len, &num_lines);

	size_t start_j = 0;
	size_t line0_len = strlen(lines[0]);
	for(size_t j = 0; j < line0_len; j++) {
		if(lines[0][j] == 'S') {
			start_j = j;
			break;
		}
	}

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	*parsed_input = (struct ParsedInput){
		.lines = lines, .num_lines = num_lines, .start_j = start_j};
	return parsed_input;
}

struct Point {
	uint32_t i, j;
	uint64_t n;
};

uint64_t point_to_int64(struct Point point) {
	uint64_t i = point.i;
	uint64_t j = point.j;
	return (i << 32) | (j & 0xFFFFFFFF);
}

void count_splittings(char **lines, const size_t num_lines,
					  const size_t line_width, kh_SplitMap_t *map,
					  struct Point cur_ray) {
	if(cur_ray.i >= num_lines || cur_ray.j >= line_width) { return; }

	char c = lines[cur_ray.i][cur_ray.j];
	if(c == '.' || c == 'S') {
		count_splittings(lines, num_lines, line_width, map,
						 (struct Point){
							 .i = cur_ray.i + 1,
							 .j = cur_ray.j,
						 });
	} else if(c == '^') {
		uint64_t kk = point_to_int64(cur_ray);
		khint_t k = kh_get(SplitMap, map, kk);
		if(k != kh_end(map)) { return; }

		int ret;
		k = kh_put(SplitMap, map, kk, &ret);
		kh_value(map, k) = cur_ray.n;

		count_splittings(lines, num_lines, line_width, map,
						 (struct Point){
							 .i = cur_ray.i + 1,
							 .j = cur_ray.j - 1,
						 });
		count_splittings(lines, num_lines, line_width, map,
						 (struct Point){
							 .i = cur_ray.i + 1,
							 .j = cur_ray.j + 1,
						 });
	}
}

uint64_t day07_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	khash_t(SplitMap) *map = kh_init(SplitMap);

	count_splittings(parsed_input.lines, parsed_input.num_lines,
					 strlen(parsed_input.lines[0]), map,
					 (struct Point){.i = 0, .j = parsed_input.start_j});
	uint64_t n = kh_size(map);
	kh_destroy(SplitMap, map);
	return n;
}

size_t count_timelines(char **lines, const size_t num_lines,
					   const size_t line_width, kh_SplitMap_t *map,
					   struct Point cur_ray) {
	if(cur_ray.i >= num_lines || cur_ray.j >= line_width) { return 1; }

	khint_t k = kh_get(SplitMap, map, point_to_int64(cur_ray));
	if(k != kh_end(map)) { return kh_value(map, k); }

	char c = lines[cur_ray.i][cur_ray.j];
	if(c == '.' || c == 'S') {
		return count_timelines(lines, num_lines, line_width, map,
							   (struct Point){
								   .i = cur_ray.i + 1,
								   .j = cur_ray.j,
							   });
	} else if(c == '^') {
		cur_ray.n = count_timelines(lines, num_lines, line_width, map,
									(struct Point){
										.i = cur_ray.i + 1,
										.j = cur_ray.j - 1,
									}) +
					count_timelines(lines, num_lines, line_width, map,
									(struct Point){
										.i = cur_ray.i + 1,
										.j = cur_ray.j + 1,
									});
		uint64_t kk = point_to_int64(cur_ray);
		int ret;
		k = kh_put(SplitMap, map, kk, &ret);
		kh_value(map, k) = cur_ray.n;
		return cur_ray.n;
	}

	return 0;
}

uint64_t day07_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;
	khash_t(SplitMap) *map = kh_init(SplitMap);
	uint64_t n =
		count_timelines(parsed_input.lines, parsed_input.num_lines,
						strlen(parsed_input.lines[0]), map,
						(struct Point){.i = 0, .j = parsed_input.start_j});
	kh_destroy(SplitMap, map);
	return n;
}

void day07_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(size_t i = 0; i < parsed_input->num_lines; i++) {
		free(parsed_input->lines[i]);
	}
	free(parsed_input->lines);
	free(parsed_input);
}

const struct Day day07 = {
	.parse = day07_parse,
	.part_1 = day07_part_1,
	.part_2 = day07_part_2,
	.free = day07_free,
};
