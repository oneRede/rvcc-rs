void assert(int expected, int actual, char *code);

int main()
{
   assert(3, ({ int i=0; goto a; a: i++; b: i++; c: i++; i; }));
}