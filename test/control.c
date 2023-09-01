#include "test.h"

/*
 * This is a block comment.
 */

int main() {
  // [16] 支持for语句\n
  ASSERT(55, ({ int i=0; int j=0; for (i=0; i<=10; i=i+1) j=i+j; j; }));
  printf("OK\n");
  return 0;
}
