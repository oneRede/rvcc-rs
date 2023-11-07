void assert(int expected, int actual, char *code);

int main()
{
   assert(0, ({ int x[3]={}; x[0]; }));
}