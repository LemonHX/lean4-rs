// c11 atomic support is not available for bindgen yet
// so we need to define _Atomic(X) X
#define _Atomic(X) X
#include <lean/lean.h>
