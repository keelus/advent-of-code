#include "../aoc.h"
#include "../utils.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct Tile {
	int64_t y, x;
};

struct ParsedInput {
	struct Tile *tiles;
	size_t num_tiles;
};

void *day09_parse(const char *input_data, const size_t input_len) {
	size_t num_lines;
	char **lines = split_lines(input_data, input_len, &num_lines);

	struct Tile *tiles = malloc(sizeof(struct Tile) * num_lines);
	size_t tile_i = 0;
	for(size_t l = 0; l < num_lines; l++) {
		uint32_t x, y;
		sscanf(lines[l], "%u,%u", &x, &y);
		tiles[tile_i++] = (struct Tile){
			.y = y,
			.x = x,
		};
	}

	free(lines);

	struct ParsedInput *parsed_input = malloc(sizeof(struct ParsedInput));
	*parsed_input =
		(struct ParsedInput){.tiles = tiles, .num_tiles = num_lines};
	return parsed_input;
}

#define abs(a) (a) < 0 ? (a) * -1 : (a)

uint64_t day09_part_1(void *vparsed_input) {
	struct ParsedInput parsed_input = *(struct ParsedInput *)vparsed_input;
	uint64_t max_area = 0;
	for(size_t i = 0; i < parsed_input.num_tiles; i++) {
		int64_t y1 = parsed_input.tiles[i].y;
		int64_t x1 = parsed_input.tiles[i].x;
		for(size_t j = i + 1; j < parsed_input.num_tiles; j++) {
			int64_t y2 = parsed_input.tiles[j].y;
			int64_t x2 = parsed_input.tiles[j].x;

			uint64_t width = abs(x1 - x2 + 1);
			if(width < 0) width *= -1;
			uint64_t height = abs(y1 - y2 + 1);
			if(height < 0) height *= -1;

			uint64_t area = width * height;
			if(area > max_area) { max_area = area; }
		}
	}
	return max_area;
}

bool is_rectangle_valid(char **map, size_t width, size_t height, struct Tile a,
						struct Tile b);


uint64_t day09_part_2(void *vparsed_input) {
	return 0;
}

void day09_free(void *vparsed_input) {
	struct ParsedInput *parsed_input = (struct ParsedInput *)vparsed_input;
	free(parsed_input->tiles);
	free(parsed_input);
}

const struct Day day09 = {
	.parse = day09_parse,
	.part_1 = day09_part_1,
	.part_2 = day09_part_2,
	.free = day09_free,
};
