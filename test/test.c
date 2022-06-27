

struct Data {
    int x;
};

void act(struct Data *d) {
    d->x = 8;
}

int main(int argc)
{
    struct Data data = {.x = argc};

    act(&data);
}