#include <errno.h>
#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <time.h>
#include <unistd.h>

#include "aoc.h"

#define ARG_DAY_MIN 1U
#define ARG_DAY_MAX 12U

#define ARG_USE_SAMPLE_DEFAULT false

#define ARG_BENCH_RUNS_MIN 1U
#define ARG_BENCH_RUNS_MAX UINT32_MAX

#define ANSI_RED "\033[1;31m"
#define ANSI_YEL "\033[1;33m"
#define ANSI_CYA "\033[1;36m"
#define ANSI_GRA "\033[38;2;150;150;150m"
#define ANSI_RES "\033[0m"

#define BENCH_PART(func, out_ns)                                               \
	do {                                                                       \
		struct timespec ts_start, ts_end;                                      \
		clock_gettime(CLOCK_MONOTONIC, &ts_start);                             \
		(func);                                                                \
		clock_gettime(CLOCK_MONOTONIC, &ts_end);                               \
		out_ns =                                                               \
			(uint64_t)(((ts_end).tv_sec - (ts_start).tv_sec) * 1000000000ULL + \
					   ((ts_end).tv_nsec - (ts_start).tv_nsec));               \
	} while(0);

#define RUN_DAY(day_n)                                                         \
	do {                                                                       \
		res = run_day(day##day_n, #day_n, "inputs/day" #day_n,                 \
					  use_sample ? "sample.txt" : "input.txt", bench_runs);    \
	} while(0);

extern struct Day day01, day02, day03, day04, day05, day06, day07, day08, day09,
	day10, day11, day12;

char *read_file_content(const char *folder, const char *filename,
						size_t *input_len) {
	size_t len = strlen(folder) + strlen(filename) + 2;
	char *full_path;
	if(!(full_path = malloc(len))) {
		fprintf(stderr,
				"[ERROR] Could not allocate %zu bytes for the full path.\n",
				len);
		return NULL;
	}
	snprintf(full_path, len, "%s/%s", folder, filename);

	char *data = NULL;

	FILE *f = fopen(full_path, "r");
	if(!f) {
		const char *errstr = strerror(errno);
		fprintf(stderr, "[ERROR] Could not open input file \"%s\": %s\n",
				full_path, errstr);
		free(full_path);
		return NULL;
	}

	free(full_path);

	if(fseek(f, 0, SEEK_END) < 0) {
		fprintf(stderr, "[ERROR] fseek() failed: %s\n", strerror(errno));
		fclose(f);
		return NULL;
	}

	long expected_size;
	if((expected_size = ftell(f)) < 0) {
		fprintf(stderr, "[ERROR] ftell() failed: %s\n", strerror(errno));
		fclose(f);
		return NULL;
	}
	rewind(f);

	if(!(data = malloc(expected_size + 1))) {
		fprintf(stderr,
				"[ERROR] Could not allocate %zu bytes for the input file.\n",
				expected_size + 1);
		fclose(f);
		return NULL;
	}

	size_t bytes_read = fread(data, 1, expected_size, f);
	fclose(f);

	if(bytes_read != (size_t)expected_size) {
		fprintf(stderr, "[ERROR] Expected %ld bytes, but only read %zu.\n",
				expected_size, bytes_read);
		free(data);
		return NULL;
	}

	data[expected_size] = '\0';
	*input_len = expected_size;
	return data;
}

void init_rand() {
	FILE *f = fopen("/dev/urandom", "r");
	if(!f) {
		srand(time(NULL));
		return;
	}

	unsigned int x;
	fread(&x, 1, sizeof(x), f);
	fclose(f);
	srand(x);
}

void print_centered(const char *text, const size_t columns,
					const float extra_pad) {
	float pad = columns / 2.0f - strlen(text) / 2.0f + extra_pad;
	printf("%*s", (int)(floor(pad)), "");
	printf("%s", text);
	printf("%*s", (int)(ceil(pad)), "");
}

void print_header(const struct winsize w, const char *day_str,
				  const uint32_t bench_runs) {
	char *emojis[] = {"🔔", "🎁", "🎅", "🌟", "🎄", "🦌",
					  "🍪", "🥛", "🛷", "🍰", "🍫", "🍬"};

	char *emoji_1 = emojis[rand() % (sizeof(emojis) / sizeof(emojis[0]))];
	char *emoji_2 = emojis[rand() % (sizeof(emojis) / sizeof(emojis[0]))];

	printf("+");
	for(uint16_t i = 0; i < w.ws_col - 2; i++) {
		putchar('=');
	}
	printf("+\n");

	char buf[1024];

	// Day line
	snprintf(buf, sizeof(buf),
			 ANSI_YEL "%s "
					  "Day %s"
					  " %s" ANSI_RES,
			 emoji_1, day_str, emoji_2);
	putchar('|');
	print_centered(buf, w.ws_col,
				   2 + strlen(ANSI_YEL) / 2.0f + strlen(ANSI_RES) / 2.0f - 1);
	putchar('|');
	putchar('\n');

	// Benchmark line
	if(bench_runs > 0) {
		snprintf(buf, sizeof(buf), ANSI_RED "Benchmarking %u times" ANSI_RES,
				 bench_runs);
		putchar('|');
		print_centered(buf, w.ws_col,
					   strlen(ANSI_RED) / 2.0f + strlen(ANSI_RES) / 2.0f - 1);
		putchar('|');
	}

	printf("+");
	for(uint16_t i = 0; i < w.ws_col - 2; i++) {
		putchar('=');
	}
	printf("+\n");
}

void format_ns(const uint64_t ns, char *buf) {
	if(ns <= 999) {
		sprintf(buf, "%luns", ns);
	} else if(ns <= 999999) {
		sprintf(buf, "%luus", ns / 1000);
	} else if(ns <= 999999999) {
		sprintf(buf, "%lums", ns / 1000000);
	} else {
		sprintf(buf, "%.2fs", (double)(ns / 1000000000.0f));

		char *decimals = NULL;
		if((decimals = strstr(buf, ".00")) != NULL) {
			*decimals = 's';
			*(decimals + 1) = '\0';
		}
	}
}

void print_stat(const size_t cols, const char *left, const char *right,
				const size_t extra_pad) {
	int pad = cols - strlen(left) - strlen(right) + extra_pad;
	printf("%s%*s%s\n", left, pad, "", right);
}

void print_stats(const struct winsize w, const bool benchmarking,
				 const uint64_t parse_ns, const uint64_t res_1,
				 const uint64_t part_1_ns, const uint64_t res_2,
				 const uint64_t part_2_ns) {
	char ns_buf[32], left_buf[128], right_buf[128];

	// Parse
	format_ns(parse_ns, ns_buf);
	snprintf(right_buf, sizeof(right_buf), ANSI_GRA "(%s%s)" ANSI_RES "\n",
			 benchmarking ? "avg=" : "", ns_buf);
	print_stat(w.ws_col, ANSI_CYA "Parse", right_buf,
			   strlen(ANSI_CYA) + strlen(ANSI_GRA) + strlen(ANSI_RES) + 1);

	// Part 1
	snprintf(left_buf, sizeof(left_buf), ANSI_CYA "Part 1" ANSI_RES);
	if(!benchmarking) { sprintf(left_buf + strlen(left_buf), ": %lu", res_1); }

	format_ns(part_1_ns, ns_buf);
	snprintf(right_buf, sizeof(right_buf), ANSI_GRA "(%s%s)" ANSI_RES,
			 benchmarking ? "avg=" : "", ns_buf);
	print_stat(w.ws_col, left_buf, right_buf,
			   strlen(ANSI_CYA) + strlen(ANSI_RES) + strlen(ANSI_GRA) +
				   strlen(ANSI_RES));

	// Part 2
	snprintf(left_buf, sizeof(left_buf), ANSI_CYA "Part 2" ANSI_RES);
	if(!benchmarking) { sprintf(left_buf + strlen(left_buf), ": %lu", res_2); }

	format_ns(part_2_ns, ns_buf);
	snprintf(right_buf, sizeof(right_buf), ANSI_GRA "(%s%s)" ANSI_RES,
			 benchmarking ? "avg=" : "", ns_buf);
	print_stat(w.ws_col, left_buf, right_buf,
			   strlen(ANSI_CYA) + strlen(ANSI_RES) + strlen(ANSI_GRA) +
				   strlen(ANSI_RES));
}


void print_day_progress(struct winsize w, uint8_t part_i) {
	static char *parts_str[4] = {
		"Parse",
		"Part 1",
		"Part 2",
		"Free parse",
	};
	static uint8_t max_parts_str_len = 10;
	char buf[1024];

	sprintf(buf, "Part %u/4: ", part_i + 1);
	sprintf(buf + strlen(buf), "%*s [", max_parts_str_len, parts_str[part_i]);

	size_t bar_max_width = w.ws_col - strlen(buf) - 1;

	size_t bar_width = ceil(bar_max_width / 4.0f * part_i);
	size_t pad_width = bar_max_width - bar_width;

	printf("%s", buf);
	for(size_t i = 0; i < bar_width; i++) {
		putchar('#');
	}
	for(size_t i = 0; i < pad_width; i++) {
		putchar(' ');
	}
	printf("]\r");
	fflush(stdout);
}

void print_bench_progress(const struct winsize w, const uint32_t bench_runs,
						  const uint32_t run_i) {
	char buf[1024];
	sprintf(buf, "Run %u/%u (%u%% completed) [", run_i + 1, bench_runs,
			(uint8_t)(ceil(run_i * 100.0f / bench_runs)));

	size_t bar_max_width = w.ws_col - strlen(buf) - 1;

	size_t bar_width = ceil((float)bar_max_width / bench_runs * run_i);
	size_t pad_width = bar_max_width - bar_width;

	printf("%s", buf);
	for(size_t i = 0; i < bar_width; i++) {
		putchar('#');
	}
	for(size_t i = 0; i < pad_width; i++) {
		putchar(' ');
	}
	printf("]\r");
	fflush(stdout);
}

int run_day(const struct Day day, const char *day_str,
			const char *day_input_folder, const char *day_input_filename,
			const uint32_t bench_runs) {
	struct winsize w;
	ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);

	print_header(w, day_str, bench_runs);

	size_t input_len;
	char *input_data =
		read_file_content(day_input_folder, day_input_filename, &input_len);
	if(!input_data) { return EXIT_FAILURE; }

	uint64_t parse_ns, part_1_ns, part_2_ns;
	uint64_t res_1, res_2;
	void *parsed_input;
	if(bench_runs > 0) {
		uint64_t total_parse_ns = 0, total_part_1_ns = 0, total_part_2_ns = 0;

		for(uint32_t i = 0; i < bench_runs; ++i) {
			print_bench_progress(w, bench_runs, i);

			BENCH_PART(parsed_input = day.parse(input_data, input_len),
					   parse_ns);
			BENCH_PART(res_1 = day.part_1(parsed_input), part_1_ns);
			BENCH_PART(res_2 = day.part_2(parsed_input), part_2_ns);

			total_parse_ns += parse_ns;
			total_part_1_ns += part_1_ns;
			total_part_2_ns += part_2_ns;

			day.free(parsed_input);
		}

		total_parse_ns /= bench_runs;
		total_part_1_ns /= bench_runs;
		total_part_2_ns /= bench_runs;

		print_stats(w, true, total_parse_ns, 0, total_part_1_ns, 0,
					total_part_2_ns);
	} else {
		print_day_progress(w, 0);
		BENCH_PART(parsed_input = day.parse(input_data, input_len), parse_ns);

		print_day_progress(w, 1);
		BENCH_PART(res_1 = day.part_1(parsed_input), part_1_ns);

		print_day_progress(w, 2);
		BENCH_PART(res_2 = day.part_2(parsed_input), part_2_ns);

		print_day_progress(w, 3);
		day.free(parsed_input);

		print_stats(w, false, parse_ns, res_1, part_1_ns, res_2, part_2_ns);
	}

	free(input_data);
	return EXIT_SUCCESS;
}

int parse_ulong_arg(const char *arg, const char *prefix,
					const unsigned long min, const unsigned long max,
					unsigned long *value) {
	const char *value_str = arg + strlen(prefix);
	*value = strtoul(value_str, NULL, 10);
	return errno == ERANGE || *value < min || *value > max;
}

int main(const int argc, const char *argv[]) {
	uint8_t day;
	uint32_t bench_runs = 0;

	bool use_sample = ARG_USE_SAMPLE_DEFAULT;
	bool day_specified = false;

	for(int i = 1; i < argc; i++) {
		const char *arg = argv[i];

		if(strstr(arg, "--day=") == arg) {
			unsigned long argval;
			if(parse_ulong_arg(arg, "--day=", ARG_DAY_MIN, ARG_DAY_MAX,
							   &argval)) {
				fprintf(stderr,
						"[ERROR] Invalid day passed. Day must be a number "
						"between %u and %u.\n",
						ARG_DAY_MIN, ARG_DAY_MAX);
				return EXIT_FAILURE;
			}
			day = argval;
			day_specified = true;
		} else if(strstr(arg, "--sample") == arg) {
			use_sample = true;
		} else if(strstr(arg, "--bench=") == arg) {
			unsigned long argval;
			if(parse_ulong_arg(arg, "--bench=", ARG_BENCH_RUNS_MIN,
							   ARG_BENCH_RUNS_MAX, &argval)) {
				fprintf(stderr,
						"[ERROR] Invalid bench run amount passed. It must be a "
						"number "
						"between %u and %u.\n",
						ARG_BENCH_RUNS_MIN, ARG_BENCH_RUNS_MAX);
				return EXIT_FAILURE;
			}
			bench_runs = argval;
		}
	}

	if(argc == 1 || !day_specified) {
		printf("Usage: %s [OPTIONS]\n", argv[0]);
		printf("Options:\n");
		printf("  --day=<N>\tDay to run (between %u and %u)\n", ARG_DAY_MIN,
			   ARG_DAY_MAX);
		printf("  --sample\tUse the day's sample input\n");
		printf("  --bench=<N>\tNumber of benchmark runs (between %u and %u)\n",
			   ARG_BENCH_RUNS_MIN, ARG_BENCH_RUNS_MAX);

		return !day_specified;
	}

	init_rand();

	int res = EXIT_SUCCESS;
	switch(day) {
	case 1: RUN_DAY(01); break;
	case 2: RUN_DAY(02); break;
	case 3: RUN_DAY(03); break;
	case 4: RUN_DAY(04); break;
	default:
		fprintf(stderr, "[ERROR] The day %u is not yet implemented!\n", day);
		return EXIT_FAILURE;
	}

	return res;
}
