#include "../aoc.h"
#include "../utils.h"
#include "../external/khash.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct OutputList {
	char **outputs;
	size_t num_outputs;
};

struct MemoKey {
	uint32_t device;
	bool visited_fft, visited_dac;
};

uint32_t dev_to_uint32(const char *dev) {
	return (uint32_t)(dev[2]) << 16 | (uint32_t)(dev[1]) << 8 |
		   (uint32_t)(dev[0]);
}

uint32_t __memo_key_hash(struct MemoKey mk) {
	uint32_t h = utils_hash_init();
	utils_hash_add_uint32(&h, mk.device);
	utils_hash_add_uint8(&h, mk.visited_dac);
	utils_hash_add_uint8(&h, mk.visited_fft);
	return h;
}
#define memo_key_hash(key) __memo_key_hash(key)

#define memo_key_eq(a, b)                                                      \
	((a).device == (b).device && (a).visited_fft == (b).visited_fft &&         \
	 (a).visited_dac == (b).visited_dac)

KHASH_INIT(Memo, struct MemoKey, uint64_t, 1, memo_key_hash, memo_key_eq)
KHASH_MAP_INIT_STR(DeviceMap, struct OutputList)

struct ParsedInput {
	khash_t(DeviceMap) * map;
};

void *day11_parse(const char *input_data, const size_t input_len) {
	size_t num_lines;
	char **lines = split_lines(input_data, input_len, &num_lines);

	khash_t(DeviceMap) *map = kh_init(DeviceMap);

	for(size_t i = 0; i < num_lines; i++) {
		const char *line = lines[i];
		char *dots = strchr(line, ':');
		char *current_device = strndup(line, dots - line);

		char **outputs = NULL;
		size_t num_outputs = 0;

		char *l = dots + 2;
		char *prev_l = l;
		while((l = strchr(l, ' ')) != NULL) {
			outputs = realloc(outputs, sizeof(char *) * ++num_outputs);
			outputs[num_outputs - 1] = strndup(prev_l, l - prev_l);
			l++;
			prev_l = l;
		}
		outputs = realloc(outputs, sizeof(char *) * ++num_outputs);
		outputs[num_outputs - 1] = strndup(prev_l, strlen(prev_l));

		int ret;
		khint_t k = kh_put(DeviceMap, map, current_device, &ret);
		kh_val(map, k) =
			(struct OutputList){.outputs = outputs, .num_outputs = num_outputs};
	}

	for(size_t i = 0; i < num_lines; i++) {
		free(lines[i]);
	}
	free(lines);

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->map = map;
	return parsed_input;
}

uint64_t count_paths(khash_t(DeviceMap) * map, const char *key) {
	if(strcmp(key, "out") == 0) { return 1; }

	khint_t k = kh_get(DeviceMap, map, key);
	struct OutputList o = kh_value(map, k);
	uint64_t count = 0;
	for(size_t i = 0; i < o.num_outputs; i++) {
		count += count_paths(map, o.outputs[i]);
	}
	return count;
}

uint64_t count_paths_2(khash_t(DeviceMap) * map, khash_t(Memo) * memo,
					   const char *key, bool visited_dac, bool visited_fft) {
	if(strcmp(key, "out") == 0) { return (visited_dac && visited_fft) ? 1 : 0; }
	visited_dac |= strcmp(key, "dac") == 0;
	visited_fft |= strcmp(key, "fft") == 0;

	struct MemoKey memo_key = {.device = dev_to_uint32(key),
							   .visited_fft = visited_fft,
							   .visited_dac = visited_dac};
	khint_t k = kh_get(Memo, memo, memo_key);
	if(k != kh_end(memo)) { return kh_value(memo, k); }

	k = kh_get(DeviceMap, map, key);
	struct OutputList outputs = kh_value(map, k);
	uint64_t sum = 0;
	for(size_t i = 0; i < outputs.num_outputs; i++) {
		uint64_t local_sum = count_paths_2(map, memo, outputs.outputs[i],
										   visited_dac, visited_fft);
		sum += local_sum;
	}

	int ret;
	k = kh_put(Memo, memo, memo_key, &ret);
	kh_val(memo, k) = sum;

	return sum;
}

uint64_t day11_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;
	return count_paths(parsed_input.map, "you");
}

uint64_t day11_part_2(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;
	khash_t(Memo) *memo = kh_init(Memo);
	uint64_t count = count_paths_2(parsed_input.map, memo, "svr", false, false);
	kh_destroy(Memo, memo);
	return count;
}

void day11_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	for(khint_t k = kh_begin(parsed_input->map); k != kh_end(parsed_input->map);
		k++) {
		if(kh_exist(parsed_input->map, k)) {
			free((char *)kh_key(parsed_input->map, k));
			struct OutputList outputs = kh_value(parsed_input->map, k);
			for(size_t i = 0; i < outputs.num_outputs; i++) {
				free(outputs.outputs[i]);
			}
			free(outputs.outputs);
		}
	}
	kh_destroy(DeviceMap, parsed_input->map);
	free(parsed_input);
}

const struct Day day11 = {
	.parse = day11_parse,
	.part_1 = day11_part_1,
	.part_2 = day11_part_2,
	.free = day11_free,
};
