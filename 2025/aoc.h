#ifndef _AOC_H
#define _AOC_H

#include <stddef.h>
#include <stdint.h>

struct Day {
	void *(*parse)(const char *, const size_t);
	uint64_t (*part_1)(void *);
	uint64_t (*part_2)(void *);
	void (*free)(void *);
};

#endif
