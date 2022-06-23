
struct Data1 {
    int u;
    int v;
};


struct Data {
    int x;
    struct Data1 d;
};


int main(int argc)
{
    struct Data data = {.x = argc, .d = {.u = 2, .v = 8}};
    asm ("nop");
    int d = data.d.u;
}