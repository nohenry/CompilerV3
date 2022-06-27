
template Data {
    x: uint32
}

action uint32 {
    inc(self) => {

    }
}

action Data {
    act(self) => {
        self.x = 6
    }
}

main(x: int32) => {
    let data = Data {
        x: 9
    } 

    data.x = 8
}