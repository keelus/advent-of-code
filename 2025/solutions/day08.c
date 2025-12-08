#include "../aoc.h"
#include "../external/khash.h"
#include "../utils.h"
#include <float.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

KHASH_SET_INIT_INT64(BoxPtrSet)

struct Box {
	uint32_t x, y, z;
	khash_t(BoxPtrSet) * conn_set;
};

struct BoxPair {
	struct Box *a, *b;
	uint64_t dist;
};

struct ParsedInput {
	struct Box *boxes;
	size_t num_boxes;

	struct BoxPair *pairs;
	size_t num_pairs;
};

int cmp_pair(const void *a, const void *b) {
	const struct BoxPair *pair_a = (struct BoxPair *)a;
	const struct BoxPair *pair_b = (struct BoxPair *)b;

	if(pair_a->dist == pair_b->dist) { return 0; }
	return pair_a->dist < pair_b->dist ? -1 : 1;
}

double dist(const struct Box *a, const struct Box *b) {
	double dx = (double)a->x - (double)b->x;
	double dy = (double)a->y - (double)b->y;
	double dz = (double)a->z - (double)b->z;
	return sqrt(dx * dx + dy * dy + dz * dz);
}

struct BoxPair *make_pairs(struct Box *boxes, const size_t num_boxes,
						   size_t *out_num_pairs) {
	size_t num_pairs = num_boxes * (num_boxes - 1) / 2;
	struct BoxPair *pairs = malloc(sizeof(struct BoxPair) * num_pairs);

	size_t pairs_i = 0;
	for(size_t i = 0; i < num_boxes; i++) {
		for(size_t j = i + 1; j < num_boxes; j++) {
			pairs[pairs_i++] =
				(struct BoxPair){.a = boxes + i,
								 .b = boxes + j,
								 .dist = dist(boxes + i, boxes + j)};
		}
	}

	*out_num_pairs = num_pairs;
	return pairs;
}

void *day08_parse(const char *input_data, const size_t input_len) {
	size_t num_lines;
	char **lines = split_lines(input_data, input_len, &num_lines);

	struct Box *boxes = malloc(sizeof(struct Box) * num_lines);
	size_t boxes_i = 0;

	for(size_t i = 0; i < num_lines; i++) {
		const char *line = lines[i];
		uint32_t x, y, z;
		sscanf(line, "%u,%u,%u", &x, &y, &z);
		boxes[boxes_i++] = (struct Box){
			.x = x, .y = y, .z = z, .conn_set = kh_init(BoxPtrSet)};
	}

	for(size_t i = 0; i < num_lines; i++) {
		free(lines[i]);
	}
	free(lines);

	size_t num_pairs;
	struct BoxPair *pairs = make_pairs(boxes, num_lines, &num_pairs);
	qsort(pairs, num_pairs, sizeof(struct BoxPair), cmp_pair);

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	*parsed_input = (struct ParsedInput){
		.boxes = boxes,
		.num_boxes = num_lines,
		.pairs = pairs,
		.num_pairs = num_pairs,
	};
	return parsed_input;
}

void connect_pair(struct BoxPair pair) {
	int ret;
	kh_put(BoxPtrSet, pair.a->conn_set, (uint64_t)pair.b, &ret);
	kh_put(BoxPtrSet, pair.b->conn_set, (uint64_t)pair.a, &ret);
}

size_t get_circuit_len(struct Box *boxes, const size_t num_boxes,
					   khash_t(BoxPtrSet) * seen, struct Box *box) {
	if(kh_get(BoxPtrSet, seen, (uint64_t)box) != kh_end(seen)) { return 0; }
	int ret;
	kh_put(BoxPtrSet, seen, (uint64_t)box, &ret);

	size_t l = 1;
	for(khint_t k = kh_begin(box->conn_set); k != kh_end(box->conn_set); k++) {
		if(kh_exist(box->conn_set, k)) {
			l += get_circuit_len(boxes, num_boxes, seen,
								 (struct Box *)kh_key(box->conn_set, k));
		}
	}
	return l;
}

uint64_t day08_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	size_t pair_i = 0;
	for(size_t i = 0; i < 1000; i++) {
		struct BoxPair pair = parsed_input.pairs[pair_i++];
		connect_pair(pair);
	}

	uint64_t max3[3] = {0, 0, 0};
	khash_t(BoxPtrSet) *seen = kh_init(BoxPtrSet);
	for(size_t i = 0; i < parsed_input.num_boxes; i++) {
		if(kh_get(BoxPtrSet, seen, (uint64_t)(parsed_input.boxes + i)) !=
		   kh_end(seen)) {
			continue;
		}

		uint64_t l = get_circuit_len(parsed_input.boxes, parsed_input.num_boxes,
									 seen, parsed_input.boxes + i);

		if(l > max3[0]) {
			max3[2] = max3[1];
			max3[1] = max3[0];
			max3[0] = l;
		} else if(l > max3[1]) {
			max3[2] = max3[1];
			max3[1] = l;
		} else if(l > max3[2]) {
			max3[2] = l;
		}
	}

	kh_destroy(BoxPtrSet, seen);

	return max3[0] * max3[1] * max3[2];
}

uint64_t day08_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	size_t pair_i = 0;
	struct BoxPair pair;
	size_t first_circuit_len = 0;
	while(first_circuit_len != parsed_input.num_boxes) {
		pair = parsed_input.pairs[pair_i++];
		connect_pair(pair);

		khash_t(BoxPtrSet) *seen = kh_init(BoxPtrSet);
		first_circuit_len =
			get_circuit_len(parsed_input.boxes, parsed_input.num_boxes, seen,
							parsed_input.boxes);
		kh_destroy(BoxPtrSet, seen);
	}

	return (uint64_t)pair.a->x * (uint64_t)pair.b->x;
}

void day08_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(size_t i = 0; i < parsed_input->num_boxes; i++) {
		kh_destroy(BoxPtrSet, parsed_input->boxes[i].conn_set);
	}
	free(parsed_input->boxes);
	free(parsed_input->pairs);
	free(parsed_input);
}

const struct Day day08 = {
	.parse = day08_parse,
	.part_1 = day08_part_1,
	.part_2 = day08_part_2,
	.free = day08_free,
};
