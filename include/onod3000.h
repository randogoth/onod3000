#pragma once
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Returns true on success. Writes three doubles: obs, z, p
bool onod_run(const char* test,
              const uint8_t* samples,
              size_t len,
              double* result /* length >= 3 */);

#ifdef __cplusplus
}
#endif
