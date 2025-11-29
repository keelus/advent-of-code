#ifndef _AOC_H
#define _AOC_H

#define DAY_PARSE_NAME(day) day_##day##_parse
#define DAY_PART_NAME(day, part) day_##day##_part_##part
#define DAY_FREE_PARSE_NAME(day) day_##day##_free_parse

#define DAY_PARSE(day) static void *DAY_PARSE_NAME(day)(const char *input_data)
#define DAY_PART(day, argname, part)                                           \
	static uint64_t DAY_PART_NAME(day, part)(void *argname)
#define DAY_FREE_PARSE(day) static void DAY_FREE_PARSE_NAME(day)(void *data)

#endif
