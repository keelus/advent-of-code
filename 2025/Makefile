NAME := aoc

CC := gcc
CFLAGS := -Wextra -Wall -pedantic -std=gnu23
LDFLAGS := -lm

SOURCES := aoc.c

BUILD_TYPE ?= default

ifeq ($(BUILD_TYPE),default)
    CFLAGS += -O3
    OBJDIR := obj/default
endif

ifeq ($(BUILD_TYPE),debug)
    CFLAGS += -O0 -g
    OBJDIR := obj/debug
endif

OBJECTS := $(SOURCES:%.c=$(OBJDIR)/%.o)
DEPFILES := $(OBJECTS:.o=.d)

.PHONY: build debug clean
.SILENT:

build:
	$(MAKE) BUILD_TYPE=default $(NAME)

debug:
	$(MAKE) BUILD_TYPE=debug $(NAME)

$(NAME): $(OBJECTS)
	$(CC) $(OBJECTS) $(LDFLAGS) -o $(NAME)

$(OBJDIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

clean:
	rm -rf $(NAME) obj

-include $(DEPFILES)
