class Write {
public:
    virtual void write() = 0;
};

class Data: public Write {
    void write() override {

    }
};

extern "C" void af(Write *w) {
    w->write(); 
}

int main() {
    Data d;

    af(dynamic_cast<Write*>(&d));
}