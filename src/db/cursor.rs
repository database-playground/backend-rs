#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Cursor {
    /// Get the offset value, defaulting to 0.
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    /// Get the limit value, defaulting to 10 and capped at 100.
    pub fn get_limit(&self) -> i64 {
        self.limit.map(|n| n.min(100)).unwrap_or(10)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cursor_default() {
        let cursor = super::Cursor {
            offset: None,
            limit: None,
        };

        assert_eq!(cursor.get_offset(), 0);
        assert_eq!(cursor.get_limit(), 10);
    }

    #[test]
    fn test_cursor_acceptable() {
        let cursor = super::Cursor {
            offset: Some(99),
            limit: Some(99),
        };

        assert_eq!(cursor.get_offset(), 99);
        assert_eq!(cursor.get_limit(), 99);
    }

    #[test]
    fn test_cursor_rounded() {
        let cursor = super::Cursor {
            offset: None,
            limit: Some(114514),
        };

        assert_eq!(cursor.get_limit(), 100);
    }
}
