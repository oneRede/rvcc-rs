void assert(int expected, int actual, char *code);

int main()
{
   assert(0, ({ int i=0; switch(-1) { case 0xffffffff: i=3; break; } i; }));
}