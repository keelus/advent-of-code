#ifndef _AOC_UTILS_H
#define _AOC_UTILS_H

#include <stdint.h>

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

#endif
