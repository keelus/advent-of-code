#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int x, y;
} Coordinate;

typedef enum { LEFT, RIGHT, UP, DOWN } Direction;

typedef struct {
	Direction dir;
	int amount;
} Instruction;

int manhattanDistance(Coordinate endPos) {
	Coordinate startPos = {0, 0};

	int dist = abs(startPos.x - endPos.x) + abs(startPos.y - endPos.y);

	return dist;
}

Coordinate findVisitedTwice(Instruction *instructions, int instructionAmount) {
	Coordinate *visitedCoordinates = NULL;
	int visitedCoordinatesCount = 0;

	Coordinate coordinate = {0, 0};
	Direction nowFacing = UP;

	for (int i = 0; i < instructionAmount; i++) {
		Instruction instruction = instructions[i];

		switch (nowFacing) {
		case UP:
			nowFacing = instruction.dir == RIGHT ? RIGHT : LEFT;
			break;
		case DOWN:
			nowFacing = instruction.dir == RIGHT ? LEFT : RIGHT;
			break;
		case RIGHT:
			nowFacing = instruction.dir == RIGHT ? DOWN : UP;
			break;
		case LEFT:
			nowFacing = instruction.dir == RIGHT ? UP : DOWN;
			break;
		}

		for (int j = 0; j < instruction.amount; j++) {
			if (nowFacing == UP || nowFacing == DOWN)
				coordinate.y += (nowFacing == UP ? 1 : -1);
			else
				coordinate.x += (nowFacing == RIGHT ? 1 : -1);

			for (int k = 0; k < visitedCoordinatesCount; k++) {
				if (visitedCoordinates[k].x == coordinate.x && visitedCoordinates[k].y == coordinate.y)
					return coordinate;
			}

			Coordinate coordinateCopy = {coordinate.x, coordinate.y};
			visitedCoordinatesCount++;

			if (visitedCoordinates == NULL) {
				visitedCoordinates = (Coordinate *)malloc(sizeof(Coordinate *));
			} else {
				visitedCoordinates = realloc(visitedCoordinates, visitedCoordinatesCount * sizeof(Coordinate *));
			}
			visitedCoordinates[visitedCoordinatesCount - 1] = coordinateCopy;
		}
	}

	// for(int i = 0; i < visitedCoordinatesCount; i++) {

	// }

	return coordinate;
}

Coordinate calculateEndPosition(Instruction *instructions, int instructionAmount) {
	Coordinate coordinate = {0, 0};
	Direction nowFacing = UP;

	for (int i = 0; i < instructionAmount; i++) {
		Instruction instruction = instructions[i];

		switch (nowFacing) {
		case UP:
			nowFacing = instruction.dir == RIGHT ? RIGHT : LEFT;
			break;
		case DOWN:
			nowFacing = instruction.dir == RIGHT ? LEFT : RIGHT;
			break;
		case RIGHT:
			nowFacing = instruction.dir == RIGHT ? DOWN : UP;
			break;
		case LEFT:
			nowFacing = instruction.dir == RIGHT ? UP : DOWN;
			break;
		}

		if (nowFacing == UP || nowFacing == DOWN)
			coordinate.y += instruction.amount * (nowFacing == UP ? 1 : -1);
		else
			coordinate.x += instruction.amount * (nowFacing == RIGHT ? 1 : -1);
	}

	return coordinate;
}

Instruction *parseInstructions(char *instructions, int *newInstructionCount) {
	printf("Parsing instructions...\n");

	int instructionCount = 1;
	for (int i = 0; i < strlen(instructions); i++) {
		if (instructions[i] == ',') {
			instructionCount++;
		}
	}

	char **newInstructions = malloc(sizeof(char *) * instructionCount);
	char *currentInstructionBuffer = NULL;
	int currentInstructionIndex = 0;

	for (int i = 0; i < strlen(instructions); i++) {
		if (instructions[i] == ' ')
			continue;

		if (instructions[i] == ',') { // Store last instruction and free current
			newInstructions[currentInstructionIndex] = currentInstructionBuffer;
			currentInstructionIndex++;
			currentInstructionBuffer = NULL;
		} else {
			int newBufferSize = 0;
			if (currentInstructionBuffer != NULL)
				newBufferSize += strlen(currentInstructionBuffer);
			newBufferSize += 2;

			char *newInstructionBuffer = malloc(sizeof(char) * newBufferSize);

			int charIndex = 0;

			if (currentInstructionBuffer != NULL) {
				for (int j = 0; j < strlen(currentInstructionBuffer); j++) {
					newInstructionBuffer[j] = currentInstructionBuffer[j];
					charIndex++;
				}
			}

			newInstructionBuffer[charIndex] = instructions[i];
			newInstructionBuffer[charIndex + 1] = '\0';

			if (currentInstructionBuffer != NULL)
				free(currentInstructionBuffer);

			currentInstructionBuffer = newInstructionBuffer;
		}
	}

	// Append last instruction
	newInstructions[currentInstructionIndex] = currentInstructionBuffer;
	currentInstructionBuffer = NULL;

	// Create structs
	Instruction *parsedInstructions = malloc(sizeof(Instruction) * instructionCount);
	for (int i = 0; i < instructionCount; i++) {
		char *instructionStr = newInstructions[i];

		char dirStr = instructionStr[0];
		int amount = atoi(instructionStr + 1);

		Instruction newParsedInstruction = {dirStr == 'L' ? LEFT : RIGHT, amount};
		parsedInstructions[i] = newParsedInstruction;
	}

	// Free used memory
	for (int i = 0; i < instructionCount; i++) {
		free(newInstructions[i]);
	}
	free(newInstructions);
	newInstructions = NULL;

	*newInstructionCount = instructionCount;
	return parsedInstructions;
}

int main(int argc, char **argv) {
	// clang-format off
	char *instructions = "R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, R3, R1, L1, R1, R2, L1, L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, L2, R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, R3, R2, L1, R5, R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, R2, L5, L5, R2, R3, R5, R4, R2, R1, L1, L5, L2, L3, L4, L5, L4, L5, L1, R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, L4, R3, R1, L2, R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, L1, L4, R2, L3, R5, R3, R1, L3";
	// clang-format on

	Instruction *parsedInstructions;
	int parsedInstructionCount;
	parsedInstructions = parseInstructions(instructions, &parsedInstructionCount);

	printf("\n== PART 1 ==\n");
	Coordinate endCoord = calculateEndPosition(parsedInstructions, parsedInstructionCount);
	printf("We end in coordinate {x:%d, y:%d}\n", endCoord.x, endCoord.y);
	printf("Manhattan distance would be: %d\n", manhattanDistance(endCoord));

	printf("\n== PART 2 ==\n");
	Coordinate twiceCoord = findVisitedTwice(parsedInstructions, parsedInstructionCount);
	printf("First twice visited coordinate {x:%d, y:%d}\n", twiceCoord.x, twiceCoord.y);
	printf("Manhattan distance would be: %d\n", manhattanDistance(twiceCoord));
}
