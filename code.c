void assert(int expected, int actual, char *code);

int main()
{
   assert(2, ({ int x[2][3]={{1,2,3},{4,5,6}}; x[0][1]; }));
}