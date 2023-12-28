#include "test.h"

void sm_test (enum SMStates_test *state,
    struct SMInput_test input,
    struct SMOutput_test *output)
{
    output->o5 = false;
    output->o6 = false;

    switch (*state)
    {
    case q1:
        output->o5 = true;
        output->o6 = true;
        if (input.i0 && input.i1)
        {
            *state = q2;
        }

    break;
    
    case q2:
        
        if (input.i1)
        {
            *state = q3;
        }

    break;
    
    case q3:
        
    
    break;
    

    default:
        break;
    }
}
