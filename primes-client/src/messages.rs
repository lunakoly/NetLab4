use common::serializable;

serializable! {
    pub enum ClientMessage {
        GetMaxNPrimes { count: u32 },
        GetRange { count: u32 },
        PublishResults { primes: Vec<u32> },
    }

    pub enum ServerMessage {
        Notification { message: String },
        MaxNPrimes { primes: Vec<u32> },
        Range { lower_bound: u32 },
        PublishingResult {},
    }
}
