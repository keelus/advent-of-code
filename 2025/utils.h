#ifndef _AOC_UTILS_H
#define _AOC_UTILS_H

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// FNV-1a
#define FNV_OFFSET_BASIS 0x811c9dc5
#define FNV_PRIME 0x01000193

static inline uint32_t utils_hash_init(void) {
	return FNV_OFFSET_BASIS;
}

static inline void utils_hash_add_uint8(uint32_t *hash, uint8_t b) {
	(*hash) ^= b;
	(*hash) *= FNV_PRIME;
}

static inline void utils_hash_add_uint16(uint32_t *hash, uint16_t v) {
	utils_hash_add_uint8(hash, v & 0xFF);
	utils_hash_add_uint8(hash, (v >> 8) & 0xFF);
}

static inline void utils_hash_add_uint32(uint32_t *hash, uint32_t v) {
	utils_hash_add_uint16(hash, v & 0xFFFF);
	utils_hash_add_uint16(hash, (v >> 16) & 0xFFFF);
}

static inline void utils_hash_add_uint64(uint32_t *hash, uint64_t v) {
	utils_hash_add_uint32(hash, v & 0xFFFFFFFF);
	utils_hash_add_uint32(hash, (v >> 32) & 0xFFFFFFFF);
}

static char **split_lines(const char *buf, const size_t len,
						  size_t *ret_num_lines) {
	if(!buf) {
		*ret_num_lines = 0;
		return NULL;
	}

	size_t num_lines = 0;
	const char *l = buf;
	while((l = strchr(l, '\n'))) {
		num_lines++;
		l++;
	}
	if(len > 0 && buf[len - 1] != '\n') { num_lines++; }

	char **lines = malloc(sizeof(char *) * num_lines);
	size_t lines_i = 0;

	const char *cur_line = buf;
	l = buf;
	while((l = strchr(l, '\n'))) {
		size_t len = l - cur_line;
		lines[lines_i++] = strndup(cur_line, len);
		cur_line = ++l;
	}
	if(cur_line < buf + len) { lines[lines_i++] = strdup(cur_line); }

	*ret_num_lines = num_lines;
	return lines;
}

#endif
