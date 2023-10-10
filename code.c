void assert(int expected, int actual, char *code);

int main()
{
   assert(0, ({ enum { zero, one, two }; zero; }));
}