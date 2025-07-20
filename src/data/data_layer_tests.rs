#[cfg(test)]
mod tests {
    use crate::data::data_layer::Data;
    use std::io::Cursor;

    #[test]
    fn test_data_serialization() {
        let data = Data {
            field1: 42,
            field2: 1337,
            field3: "Hello, World!".to_string(),
        };

        let serialized = data.serialize().expect("Serialization should succeed");
        
        // Check that we have some data
        assert!(!serialized.is_empty());
        
        // The serialized data should have:
        // 4 bytes for field1 + 2 bytes for field2 + 4 bytes for string length + string bytes
        let expected_len = 4 + 2 + 4 + "Hello, World!".len();
        assert_eq!(serialized.len(), expected_len);
    }

    #[test]
    fn test_data_deserialization() {
        let original = Data {
            field1: 12345,
            field2: 999,
            field3: "Test String".to_string(),
        };

        let serialized = original.serialize().expect("Serialization should succeed");
        let mut cursor = Cursor::new(serialized.as_slice());
        let deserialized = Data::deserialize(&mut cursor).expect("Deserialization should succeed");

        assert_eq!(deserialized.field1, original.field1);
        assert_eq!(deserialized.field2, original.field2);
        assert_eq!(deserialized.field3, original.field3);
    }

    #[test]
    fn test_data_round_trip() {
        let test_cases = vec![
            Data {
                field1: 0,
                field2: 0,
                field3: "".to_string(),
            },
            Data {
                field1: u32::MAX,
                field2: u16::MAX,
                field3: "Maximum values".to_string(),
            },
            Data {
                field1: 42,
                field2: 1337,
                field3: "Hello, 世界! 🦀".to_string(), // Unicode test
            },
        ];

        for original in test_cases {
            let serialized = original.serialize().expect("Serialization should succeed");
            let mut cursor = Cursor::new(serialized.as_slice());
            let deserialized = Data::deserialize(&mut cursor).expect("Deserialization should succeed");

            assert_eq!(deserialized.field1, original.field1);
            assert_eq!(deserialized.field2, original.field2);
            assert_eq!(deserialized.field3, original.field3);
        }
    }

    #[test]
    fn test_data_large_string() {
        let large_string = "A".repeat(10000);
        let data = Data {
            field1: 1,
            field2: 2,
            field3: large_string.clone(),
        };

        let serialized = data.serialize().expect("Serialization should succeed");
        let mut cursor = Cursor::new(serialized.as_slice());
        let deserialized = Data::deserialize(&mut cursor).expect("Deserialization should succeed");

        assert_eq!(deserialized.field1, 1);
        assert_eq!(deserialized.field2, 2);
        assert_eq!(deserialized.field3, large_string);
    }

    #[test]
    fn test_data_deserialization_truncated() {
        let data = Data {
            field1: 42,
            field2: 1337,
            field3: "Hello".to_string(),
        };

        let mut serialized = data.serialize().expect("Serialization should succeed");
        
        // Truncate the data
        serialized.truncate(5);
        
        let mut cursor = Cursor::new(serialized.as_slice());
        let result = Data::deserialize(&mut cursor);
        
        assert!(result.is_err(), "Deserialization should fail with truncated data");
    }

    #[test]
    fn test_data_deserialization_invalid_utf8() {
        let mut bytes = Vec::new();
        
        // Write valid field1 and field2
        bytes.extend_from_slice(&42u32.to_ne_bytes());
        bytes.extend_from_slice(&1337u16.to_ne_bytes());
        
        // Write length for string
        bytes.extend_from_slice(&3u32.to_ne_bytes());
        
        // Write invalid UTF-8 bytes
        bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD]);
        
        let mut cursor = Cursor::new(bytes.as_slice());
        let result = Data::deserialize(&mut cursor);
        
        assert!(result.is_err(), "Deserialization should fail with invalid UTF-8");
    }

    #[test]
    fn test_empty_string_serialization() {
        let data = Data {
            field1: 100,
            field2: 200,
            field3: "".to_string(),
        };

        let serialized = data.serialize().expect("Serialization should succeed");
        let mut cursor = Cursor::new(serialized.as_slice());
        let deserialized = Data::deserialize(&mut cursor).expect("Deserialization should succeed");

        assert_eq!(deserialized.field1, 100);
        assert_eq!(deserialized.field2, 200);
        assert_eq!(deserialized.field3, "");
    }
}
