#pragma once

namespace example {

class Callback {
public:
    virtual void onResult(long) const = 0;
    virtual ~Callback() {}
};


class Power {
    int base;

public:
    Power(int base_) : base(base_) {}

    void helloWorld(int val) const;

    void power(unsigned int exp, const Callback& callback) const;
};

struct Pod {
    int i;
    int j;
};

}
