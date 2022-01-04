#include <stdint.h>

uintptr_t Usize_copy(const uintptr_t *x) {
  return *x;
}

String Usize_str(uintptr_t x) {
  int size = snprintf(NULL, 0, "%" PRIuPTR, x) + 1;
  String buffer = CARP_MALLOC(size);
  sprintf(buffer, "%" PRIuPTR, x);
  return buffer;
}

String Usize_format(const String *str, uintptr_t x) {
  int size = snprintf(NULL, 0, *str, x) + 1;
  String buffer = CARP_MALLOC(size);
  sprintf(buffer, *str, x);
  return buffer;
}
