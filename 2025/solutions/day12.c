#include "../aoc.h"
#include "../utils.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Shape {
	char data[3][3];
	size_t area_size;
};

struct Region {
	uint32_t width, height;

	uint32_t *quantities;
	size_t num_quantities;
};

struct ParsedInput {
	struct Shape *shapes;
	size_t num_shapes;

	struct Region *regions;
	size_t num_regions;
};

void *day12_parse(const char *input_data, const size_t input_len) {
	size_t num_lines;
	char **lines = split_lines(input_data, input_len, &num_lines);

	struct Shape *shapes = NULL;
	size_t num_shapes = 0;

	bool parsing_shape_data = false;
	struct Shape current_shape = {0};
	uint8_t data_i = 0;

	struct Region *regions = NULL;
	size_t num_regions = 0;

	for(size_t i = 0; i < num_lines; i++) {
		const char *line = lines[i];

		if(strlen(line) == 0) {
			for(size_t j = 0; j < 3; j++) {
				for(size_t k = 0; k < 3; k++) {
					if(current_shape.data[j][k] == '#') {
						current_shape.area_size++;
					}
				}
			}
			shapes = realloc(shapes, sizeof(struct Shape) * ++num_shapes);
			shapes[num_shapes - 1] = current_shape;
			memset(&current_shape, 0, sizeof(current_shape));
			parsing_shape_data = false;
			data_i = 0;
			continue;
		}

		if(parsing_shape_data) {
			current_shape.data[data_i][0] = line[0];
			current_shape.data[data_i][1] = line[1];
			current_shape.data[data_i][2] = line[2];
			data_i++;
			continue;
		}

		bool is_shape = strchr(line, 'x') == NULL;
		if(is_shape) {
			parsing_shape_data = true;
			continue;
		} else {
			struct Region current_region = {0};
			sscanf(line, "%ux%u: ", &current_region.width,
				   &current_region.height);
			char *l = strchr(line, ' ') + 1;
			char *prev_l = l;

			while((l = strchr(l, ' '))) {
				size_t quantity;
				sscanf(prev_l, "%zu", &quantity);

				current_region.quantities =
					realloc(current_region.quantities,
							sizeof(size_t) * ++current_region.num_quantities);
				current_region.quantities[current_region.num_quantities - 1] =
					quantity;
				l++;
				prev_l = l;
			}
			size_t quantity;
			sscanf(prev_l, "%zu", &quantity);
			current_region.quantities =
				realloc(current_region.quantities,
						sizeof(size_t) * ++current_region.num_quantities);
			current_region.quantities[current_region.num_quantities - 1] =
				quantity;

			regions = realloc(regions, sizeof(struct Region) * ++num_regions);
			regions[num_regions - 1] = current_region;
		}
	}

	for(size_t i = 0; i < num_lines; i++) {
		free(lines[i]);
	}
	free(lines);

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	parsed_input->shapes = shapes;
	parsed_input->num_shapes = num_shapes;
	parsed_input->regions = regions;
	parsed_input->num_regions = num_regions;
	return parsed_input;
}

uint64_t day12_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;

	size_t count = 0;
	for(size_t i = 0; i < parsed_input.num_regions; i++) {
		struct Region reg = parsed_input.regions[i];
		size_t needed_area = reg.width * reg.height;
		size_t present_area = 0;
		for(size_t shape_i = 0; shape_i < reg.num_quantities; shape_i++) {
			present_area += reg.quantities[shape_i] *
							parsed_input.shapes[shape_i].area_size;
		}

		if(present_area < needed_area) { count++; }
	}

	return count;
}

uint64_t day12_part_2(void *vparsed_input) {
	return 0;
}

void day12_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	free(parsed_input->shapes);
	for(size_t i = 0; i < parsed_input->num_regions; i++) {
		free(parsed_input->regions[i].quantities);
	}
	free(parsed_input->regions);
	free(parsed_input);
}

const struct Day day12 = {
	.parse = day12_parse,
	.part_1 = day12_part_1,
	.part_2 = day12_part_2,
	.free = day12_free,
};
