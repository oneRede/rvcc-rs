void assert(int expected, int actual, char *code);

int main()
{
   assert(2, 0 ? 1 : 2);
}