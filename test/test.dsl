template Data<T> {
    x: T
}

spec Write {
    write(self) =>
}

action <T> Data<T> {
    act(const self, yes: int32) => {
    }
}

main() => {
    let list = List {
        len: 7
    } 

    println("Poop {}", "Potato")
}