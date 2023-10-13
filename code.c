void assert(int expected, int actual, char *code);

int main()
{
   assert(2, ({ char x[!0+1]; sizeof(x); }));
}