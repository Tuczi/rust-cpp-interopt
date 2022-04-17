#pragma once

typedef void (*callback_t)(unsigned long);

void helloWorld(int val);

void power(int base, unsigned int exp, callback_t callback);
