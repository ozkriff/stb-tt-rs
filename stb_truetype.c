#include <stdio.h>
#include <stdlib.h>

#define STBTT_malloc(x,u)  malloc(x)
#define STBTT_free(x,u)    free(x)

// force following include to generate implementation
#define STB_TRUETYPE_IMPLEMENTATION
#include "stb_truetype.h"

