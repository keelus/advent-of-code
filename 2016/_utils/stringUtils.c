#include "stringUtils.h"
#include <stdlib.h>
#include <string.h>

char **stringUtils_Split(char *str, char sep, int *count) {
	int substringCount = 1;
	for (int i = 0; i < strlen(str); i++) {
		if (str[i] == sep)
			substringCount++;
	}

	char **substrings = (char **)malloc(sizeof(char *) * substringCount);
	int substringIdx = 0;

	char *buff = NULL;
	int lbuff = 0;

	for (int i = 0; i <= strlen(str); i++) {
		if (str[i] == sep || str[i] == '\0') {
			if (buff == NULL) { // Allow empty substrings
				buff = malloc(sizeof(char));
				buff[0] = 0;
			}

			substrings[substringIdx++] = buff;

			buff = NULL;
			lbuff = 0;
		} else {
			if (buff == NULL) {
				buff = malloc(sizeof(char) * 2);
				lbuff = 2;

				buff[0] = str[i];
				buff[1] = '\0';
			} else {
				char *tbuff = malloc(sizeof(char) * lbuff + 1);
				int charIdx = 0;

				for (int j = 0; j < lbuff - 1; j++) {
					tbuff[j] = buff[j];
					charIdx++;
				}

				tbuff[charIdx] = str[i];
				tbuff[charIdx + 1] = '\0';

				lbuff++;

				free(buff);
				buff = tbuff;
				tbuff = NULL;
			}
		}
	}

	(*count) = substringCount;
	return substrings;
}
