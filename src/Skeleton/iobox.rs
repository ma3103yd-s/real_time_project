use std::io::Result;

// TODO: 
// Define constants relating to subdev, ports, etc of the ioboc


enum AnalogChannel {
    AnalogIn(u32),
    AnalogOut(u32),

}

enum DigitalChannel {
    DigitalIn(u32),
    DigitalOut(u32),
}



impl AnalogChannel {

    pub fn read(&self) -> io::Result<u32> {
        // TODO: read analog data from iobox.
        unimplemented!();
    }

    pub fn write(&self) -> io::Result<u32> {
        // TODO: write data to iobox
        unimplemented!();
    }


}


impl DigitalChannel {

    pub fn read(&self) -> io::Result<u32> {
        // TODO: read analog data from iobox.
        unimplemented!();
    }

    pub fn write(&self) -> io::Result<u32> {
        // TODO: write data to iobox
        unimplemented!();
    }


}