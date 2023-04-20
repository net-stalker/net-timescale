@0x8407268e65d4ceb4;

struct DataToSend {
    requirement @0 :Data;
    #In other words it is an API provided by all the microservice 

    union {
        packedData :group {
        #A protocol for inserting data in the timescaleDB

            frameTime @1 :Int64;
            #The time when the data was captured

            srcAddr @2 :Text;
            #Source ip address which is taken from a pcap file
            dstAddr @3 :Text;
            #Sedtination ip address which is taken from a pcap file

            json @4 :Data;
            #The rest of the data from pcap converted to json
        }
        timeInterval :group {
        #A protocol for selecting data within time interval from timescaleDB

            startInterval @5 :Int64;
            #The start of the time interval within which data must be selected
            endInterval @6 :Int64;
            #The end of the time interval within which data must be selected
        }
    }
}