#ifndef _AOC_H
#define _AOC_H

#include <stdint.h>

struct Day {
	void *(*parse)(const char *);
	uint64_t (*part_1)(const void *);
	uint64_t (*part_2)(const void *);
	void (*free)(const void *);
};

#endif
