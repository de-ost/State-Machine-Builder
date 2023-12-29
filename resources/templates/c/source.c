#include "/*CODE:NAME*/.h"

void sm_/*CODE:NAME*/ (enum SMStates_/*CODE:NAME*/ *state,
    struct SMInput_/*CODE:NAME*/ input,
    struct SMOutput_/*CODE:NAME*/ *output)
{
/*CODE:RESET_OUTPUT*/
    switch (*state)
    {
/*CODE:CASE*/
    default:
        break;
    }
}

bool sm_/*CODE:NAME*/_is_end_state(enum sm_states_/*CODE:NAME*/ state){
    return /*CODE:END_STATE*/;
}
