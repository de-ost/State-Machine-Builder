#pragma once

#include <stdbool.h>

enum SMStates_/*CODE:NAME*/
{
/*CODE:STATES_ENUM*/
};

struct SMInput_/*CODE:NAME*/
{
/*CODE:INPUTS_DECLARATION*/
};

struct SMOutput_/*CODE:NAME*/
{
/*CODE:OUTPUTS_DECLARATION*/
};

enum SMStates_/*CODE:NAME*/ state_/*CODE:NAME*/ = /*CODE:INITIAL_STATE*/;

void sm_/*CODE:NAME*/ ( enum sm_states_/*CODE:NAME*/ *state,
    struct SMInput_/*CODE:NAME*/ input,
    struct SMOutput_/*CODE:NAME*/ *output);
