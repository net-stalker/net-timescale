@0x9b69d735000968dc;

struct TimeInterval {
    frameTime @0:Int64;     #The time when the data was captured

    intervalStart @1:Int64; #The time interval start time
    intervalEnd @2:Int64;   #The time interval end time
}