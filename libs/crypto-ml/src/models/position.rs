use linfa;

pub struct Position {
    pub id: u64,
    pub crypto: String,
    pub bottom: f64,
    pub top: f64,
    // pub date time
    pub date: String,
}

// In rust programming, I wanna write test cases for Positions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let position = Position {
            id: 1,
            crypto: String::from("BTC"),
            bottom: 100.0,
            top: 200.0,
            date: String::from("2021-01-01"),
        };

        assert_eq!(position.id, 1);
        assert_eq!(position.crypto, "BTC");
        assert_eq!(position.bottom, 100.0);
        assert_eq!(position.top, 200.0);
        assert_eq!(position.date, "2021-01-01");
    }

    
    #[test]
    fn test_ml() {
        let position = Position {
            id: 1,
            crypto: String::from("BTC"),
            bottom: 100.0,
            top: 200.0,
            date: String::from("2021-01-01"),
        };
    }
}
