#pragma once

#include <stdbool.h>

enum SMStates_test
{
     q1,
     q2,
     q3,

};

struct SMInput_test
{
    bool i0;
    bool i1;

};

struct SMOutput_test
{
    bool o5;
    bool o6;

};

enum SMStates_test state_test = q1;

void sm_test ( enum sm_states_test *state,
                        struct SMInput_test input,
                        struct SMOutput_test *output);
