#include <stdio.h>

#include "example.h"

void helloWorld(int val) {
	printf("Hello World from C. Val=%d\n", val);
}

void power(int base, unsigned int exp, callback_t callback) {
	unsigned long result = base;
	if( exp == 0 ) {
		result = 1;
	}

	for(int i=1; i<exp; i++) {
		result*=base;
	}

	callback(result);
}