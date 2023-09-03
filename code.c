int main()
{
   assert(16, ({ struct {int a; char b;} x; sizeof(x); }), "({ struct {int a; char b;} x; sizeof(x); })");
}