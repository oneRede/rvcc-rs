#include "test.h"

int main() {
  // [74] 支持enum\n
  ASSERT(0, ({ enum { zero, one, two }; zero; }));
  ASSERT(1, ({ enum { zero, one, two }; one; }));
  ASSERT(2, ({ enum { zero, one, two }; two; }));

  printf("OK\n");
  return 0;
}
