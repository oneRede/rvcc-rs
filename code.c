void assert(int expected, int actual, char *code);

int main()
{
   assert('a', ({ char x[4]="abc"; x[0]; }));
}