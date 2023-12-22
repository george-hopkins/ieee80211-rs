use macro_bits::serializable_enum;

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    /// The sub type of the data frame.
    pub enum DataFrameSubtype: u8 {
        #[default]
        Data => 0b0000,
        DataCFAck => 0b0001,
        DataCFPoll => 0b0010,
        DataCFAckCFPoll => 0b0011,
        Null => 0b0100,
        CFAck => 0b0101,
        CFPoll => 0b0110,
        CFAckCFPoll => 0b0111,
        QoSData => 0b1000,
        QoSDataCFAck => 0b1001,
        QoSDataCFPoll => 0b1010,
        QoSDataCFAckCFPoll => 0b1011,
        QoSNull => 0b1100,
        QoSCFPoll => 0b1110,
        QoSCFAckCFPoll => 0b1111
    }
}
impl DataFrameSubtype {
    /// Check if the data frame is QoS.
    pub const fn is_qos(&self) -> bool {
        matches!(
            self,
            Self::QoSData
                | Self::QoSDataCFAck
                | Self::QoSDataCFPoll
                | Self::QoSDataCFAckCFPoll
                | Self::QoSNull
                | Self::QoSCFPoll
                | Self::QoSCFAckCFPoll
        )
    }
    /// Check if the data frame has a payload.
    pub const fn has_payload(&self) -> bool {
        matches!(
            self,
            Self::Data
                | Self::DataCFAck
                | Self::DataCFPoll
                | Self::DataCFAckCFPoll
                | Self::QoSData
                | Self::QoSDataCFAck
                | Self::QoSDataCFPoll
                | Self::QoSDataCFAckCFPoll
        )
    }
}
