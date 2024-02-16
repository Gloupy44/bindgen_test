 #include "my_c_lib.h"

#include <stdio.h>

int32_t my_c_func(char const * const s)
{
    printf("Hello %s, I'm a C library !\n", s);
    return 123456;
}
