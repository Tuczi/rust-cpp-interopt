#include <iostream>

#include "example2.hpp"

namespace example {

void Power::helloWorld(int val) const {
    std::cout << "Hello World from C++. Val=" << val << std::endl;
}

void Power::power(unsigned int exp, const Callback& callback) const {
    long result = base;
    if( exp == 0 ) {
        result = 1;
    }

    for(int i=1; i<exp; i++) {
        result*=base;
    }

    callback.onResult(result);
}

}
